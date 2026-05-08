use chess_game::board::cell::Color::*;
use chess_game::board::moves::move_structs::Move;
use chess_game::engine::minimax::iterative_deepening;
use chess_game::engine::search_context::{SearchContext, SearchParams};
use chess_game::game::Game;
use std::error::Error;
use std::io;
use std::io::{BufRead, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const MAX_DEPTH_UCI: u8 = 16;

struct GoParams {
    movetime: f64,
    wtime: f64,
    btime: f64,
    winc: f64,
    binc: f64,
    depth: u8,
    nodes: u64,
    infinite: bool,
}

impl Default for GoParams {
    fn default() -> Self {
        Self {
            movetime: 0.0,
            wtime: 0.0,
            btime: 0.0,
            winc: 0.0,
            binc: 0.0,
            depth: 0,
            nodes: 0,
            infinite: false,
        }
    }
}

struct Engine {
    game: Game,
    search_ctx: SearchContext,
    debug: bool,
    search_handle: Option<thread::JoinHandle<()>>,
}

impl Engine {
    fn new() -> Self {
        Self {
            game: Game::new(),
            search_ctx: SearchContext::new(),
            debug: false,
            search_handle: None,
        }
    }

    fn wait_search(&mut self) {
        if let Some(h) = self.search_handle.take() {
            let _ = h.join();
        }
    }

    fn compute_budget(&self, p: &GoParams) -> f64 {
        if p.infinite {
            return 0.0;
        }
        if p.movetime > 0.0 {
            return p.movetime;
        }
        if p.depth > 0 || p.nodes > 0 {
            return 0.0;
        }
        let (time, inc) = if self.game.active_player == White {
            (p.wtime, p.winc)
        } else {
            (p.btime, p.binc)
        };
        if time == 0.0 {
            return 0.0;
        }
        let mut b = time / 30.0 + inc * 0.8;
        let hard_limit = time * 0.9;
        if b > hard_limit {
            b = hard_limit;
        }
        if b < 50.0 && time > 50.0 {
            b = 50.0;
        }
        b
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut engine = Engine::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let words: Vec<&str> = line.split_whitespace().collect();
        if engine.debug {
            eprintln!("< {line}");
        }
        match words.first().copied() {
            Some("uci") => engine.cmd_uci()?,
            Some("quit") => {
                engine.wait_search();
                break;
            }
            Some("setoption") => engine.cmd_setoption(&words)?,
            Some("position") => engine.cmd_position(words)?,
            Some("ucinewgame") => engine.cmd_ucinewgame()?,
            Some("isready") => engine.cmd_isready()?,
            Some("go") => engine.cmd_go(words)?,
            Some("stop") => engine.cmd_stop()?,
            Some("ponderhit") => engine.cmd_ponderhit()?,
            Some("debug") => engine.cmd_debug(&words)?,
            Some("register") => {}
            Some(_) => eprintln!("unknown command: {line}"),
            None => continue,
        }
    }
    engine.wait_search();
    Ok(())
}

impl Engine {
    fn cmd_uci(&self) -> Result<()> {
        println!("id name ChessGame");
        println!("id author Muffin");
        println!("option name Hash type spin default 32 min 1 max 2048");
        println!("option name Threads type spin default 1 min 1 max 1");
        println!("uciok");
        io::stdout().flush()?;
        Ok(())
    }

    fn cmd_isready(&self) -> Result<()> {
        println!("readyok");
        io::stdout().flush()?;
        Ok(())
    }

    fn cmd_ucinewgame(&mut self) -> Result<()> {
        self.game = Game::new();
        self.search_ctx.reset_for_new_game();
        Ok(())
    }

    fn cmd_setoption(&self, _words: &[&str]) -> Result<()> {
        Ok(())
    }

    fn cmd_debug(&mut self, words: &[&str]) -> Result<()> {
        match words.get(1).copied() {
            Some("on") => self.debug = true,
            Some("off") => self.debug = false,
            _ => {}
        }
        Ok(())
    }

    fn cmd_stop(&self) -> Result<()> {
        self.search_ctx.stop.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn cmd_ponderhit(&self) -> Result<()> {
        Ok(())
    }

    fn cmd_position(&mut self, words: Vec<&str>) -> Result<()> {
        let mut i = 1;
        match words.get(i).copied() {
            Some("startpos") => {
                self.game = Game::new();
                i += 1;
            }
            Some("fen") => {
                i += 1;
                let fen = words[i..i + 6].join(" ");
                self.game = Game::from_fen(&fen);
                i += 6;
            }
            _ => {}
        }
        if words.get(i).copied() == Some("moves") {
            i += 1;
            for mv_str in &words[i..] {
                if let Some(mv) = self
                    .game
                    .board
                    .move_from_uci(mv_str, self.game.active_player)
                {
                    self.game.board.apply_move(&mv, self.game.active_player);
                    self.game.active_player = match self.game.active_player {
                        White => Black,
                        Black => White,
                    };
                    *self
                        .game
                        .draw
                        .board_hashs
                        .entry(self.game.board.hash)
                        .or_insert(0) += 1;
                }
            }
        }
        Ok(())
    }

    fn cmd_go(&mut self, words: Vec<&str>) -> Result<()> {
        let mut go = GoParams::default();
        let mut i = 1;
        while i < words.len() {
            match words[i] {
                "movetime" => {
                    if let Some(v) = words.get(i + 1).and_then(|s| s.parse::<f64>().ok()) {
                        go.movetime = v;
                        i += 1;
                    }
                }
                "wtime" => {
                    if let Some(v) = words.get(i + 1).and_then(|s| s.parse::<f64>().ok()) {
                        go.wtime = v;
                        i += 1;
                    }
                }
                "btime" => {
                    if let Some(v) = words.get(i + 1).and_then(|s| s.parse::<f64>().ok()) {
                        go.btime = v;
                        i += 1;
                    }
                }
                "winc" => {
                    if let Some(v) = words.get(i + 1).and_then(|s| s.parse::<f64>().ok()) {
                        go.winc = v;
                        i += 1;
                    }
                }
                "binc" => {
                    if let Some(v) = words.get(i + 1).and_then(|s| s.parse::<f64>().ok()) {
                        go.binc = v;
                        i += 1;
                    }
                }
                "depth" => {
                    if let Some(v) = words.get(i + 1).and_then(|s| s.parse::<u8>().ok()) {
                        go.depth = v;
                        i += 1;
                    }
                }
                "nodes" => {
                    if let Some(v) = words.get(i + 1).and_then(|s| s.parse::<u64>().ok()) {
                        go.nodes = v;
                        i += 1;
                    }
                }
                "infinite" => go.infinite = true,
                _ => {}
            }
            i += 1;
        }

        let budget = self.compute_budget(&go);
        let max_depth = if go.depth > 0 {
            go.depth.min(MAX_DEPTH_UCI)
        } else {
            MAX_DEPTH_UCI
        };

        let fresh_stop = Arc::new(AtomicBool::new(false));
        self.search_ctx.stop = fresh_stop.clone();

        let debug = self.debug;
        let mut game = self.game.clone();
        let mut search_ctx = self.search_ctx.clone();
        search_ctx.stop = fresh_stop.clone();

        if go.nodes > 0 {
            search_ctx.stats.max_nodes = go.nodes;
        }

        if budget > 0.0 {
            let stop = fresh_stop.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(budget as u64));
                stop.store(true, Ordering::Relaxed);
            });
        }

        self.search_handle = Some(thread::spawn(move || {
            let board_hashs = game.draw.board_hashs.clone();
            let draw_count = game.draw.draw_moves_count;
            let mut params =
                SearchParams::new(&mut search_ctx, &board_hashs, draw_count);

            let mv_str = iterative_deepening(
                &mut game.board,
                game.active_player,
                max_depth,
                &mut game.depth,
                budget,
                &mut params,
            )
            .map(|mv: Move| mv.to_uci())
            .unwrap_or_else(|| "0000".to_string());

            for (depth, score, elapsed_ms, nodes) in &params.ctx.stats.depth_results {
                let nps = if *elapsed_ms > 0 { nodes * 1000 / elapsed_ms } else { 0 };
                println!(
                    "info depth {depth} score cp {score} nodes {nodes} nps {nps} time {elapsed_ms}"
                );
            }
            println!("bestmove {mv_str}");
            let _ = io::stdout().flush();
            if debug {
                if let Some((depth, _, elapsed_ms, nodes)) = params.ctx.stats.depth_results.last() {
                    eprintln!("[debug] bestmove={mv_str} depth={depth} time={elapsed_ms}ms nodes={nodes}");
                }
            }
        }));

        Ok(())
    }
}
