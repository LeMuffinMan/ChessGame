use chess_game::board::moves::move_structs::Move;
use chess_game::engine::bot::BotDifficulty::*;
use chess_game::engine::bot::PlayerType::*;
use chess_game::engine::bot::get_bot_move;
use chess_game::engine::search_context::SearchContext;
use chess_game::game::Game;
use std::error::Error;
use std::io;
use std::io::{BufRead, Write};
use std::sync::atomic::Ordering;
use std::thread;

// todo : enums errors
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone)]
struct Engine {
    wtime: f64,
    btime: f64,
    winc: f64,
    binc: f64,
    depth: usize,
    nodes: usize,
    infinite: bool,
    game: Game,
    search_ctx: SearchContext,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    fn new() -> Self {
        Self {
            wtime: 0.0,
            btime: 0.0,
            winc: 0.0,
            binc: 0.0,
            depth: 0,
            nodes: 0,
            infinite: false,
            game: Game::new(),
            search_ctx: SearchContext::new(),
        }
    }
    fn search(&mut self) -> Option<Move> {
        get_bot_move(
            &Bot(Depth(11)),
            &mut self.game.board,
            self.game.active_player,
            &mut self.search_ctx,
            &self.game.draw.board_hashs,
            self.game.draw.draw_moves_count,
            &mut self.game.depth,
        )
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    // * The engine should boot and wait for input from the GUI,
    //   the engine should wait for the "isready" or "setoption" command to set up its internal parameters
    //   as the boot process should be as quick as possible.
    let mut engine = Engine::new();
    for line in stdin.lock().lines() {
        let line = line?;
        //   Example: "debug on\n" and  "   debug     on  \n" and "\t  debug \t  \t\ton\t  \n"
        //   all set the debug mode of the engine on.
        let words: Vec<&str> = line.split_whitespace().collect();

        match words.get(0).copied() {
            Some("uci") => engine.cmd_uci()?,
            Some("quit") => break,
            Some("setoption") => engine.cmd_setoption()?,
            Some("position") => engine.cmd_position(words)?,
            Some("ucinewgame") => engine.cmd_ucinewgame()?,
            Some("isready") => engine.cmd_isready()?,
            Some("go") => engine.cmd_go(words)?,
            Some("stop") => engine.cmd_stop()?,
            Some("ponderhit") => engine.cmd_ponderhit()?,
            Some("debug") => engine.cmd_debug(words)?,
            Some("register") => engine.cmd_register()?,
            Some(_) => engine.cmd_unknown(&line)?,
            None => continue,
        }
    }

    Ok(())
}

impl Engine {
    fn cmd_setoption(&self) -> Result<()> {
        // * setoption name <id> [value <x>]
        // this is sent to the engine when the user wants to change the internal parameters
        // of the engine. For the "button" type no value is needed.
        // One string will be sent for each parameter and this will only be sent when the engine is waiting.
        // The name and value of the option in <id> should not be case sensitive and can inlude spaces.
        // The substrings "value" and "name" should be avoided in <id> and <x> to allow unambiguous parsing,
        // for example do not use <name> = "draw value".
        // Here are some strings for the example below:
        //    "setoption name Nullmove value true\n"
        //    "setoption name Selectivity value 3\n"
        //    "setoption name Style value Risky\n"
        //    "setoption name Clear Hash\n"
        //    "setoption name NalimovPath value c:\chess\tb\4;c:\chess\tb\5\n"
        Ok(())
    }

    fn cmd_position(&self, _words: Vec<&str>) -> Result<()> {
        // * Before the engine is asked to search on a position, there will always be a position command
        //   to tell the engine about the current position.
        // * position [fen <fenstring> | startpos ]  moves <move1> .... <movei>
        // set up the position described in fenstring on the internal board and
        // play the moves on the internal chess board.
        // if the game was played  from the start position the string "startpos" will be sent
        // Note: no "new" command is needed. However, if this position is from a different game than
        // the last position sent to the engine, the GUI should have sent a "ucinewgame" inbetween.
        // match words.get(1).copied() {
        //     Some("startpos") => { engine.game.board::new_board() },
        //     None => { },
        //     _ => { engine.game.board::from_fen() },
        // }
        // match words.get(2).copied() {
        //     Some("moves") => {
        //         let mut active_player = White;
        //         for m in words[3.. words.len()] {
        //             let mv = parse_move(m);
        //             engine.game.board.apply_move(mv, active_player);
        //             active_player = if active_player == White { Black } else { White };
        //         }
        //     }
        // }
        Ok(())
    }

    fn cmd_ucinewgame(&self) -> Result<()> {
        // this is sent to the engine when the next search (started with "position" and "go") will be from
        // a different game. This can be a new game the engine should play or a new game it should analyse but
        // also the next position from a testsuite with positions only.
        // If the GUI hasn't sent a "ucinewgame" before the first "position" command, the engine shouldn't
        // expect any further ucinewgame commands as the GUI is probably not supporting the ucinewgame command.
        // So the engine should not rely on this command even though all new GUIs should support it.
        // As the engine's reaction to "ucinewgame" can take some time the GUI should always send "isready"
        // after "ucinewgame" to wait for the engine to finish its operation.
        // engine.game = Game::new();
        Ok(())
    }

    fn cmd_isready(&self) -> Result<()> {
        let mut stdout = io::stdout();
        // this is used to synchronize the engine with the GUI. When the GUI has sent a command or
        // multiple commands that can take some time to complete,
        // this command can be used to wait for the engine to be ready again or
        // to ping the engine to find out if it is still alive.
        // E.g. this should be sent after setting the path to the tablebases as this can take some time.
        // This command is also required once before the engine is asked to do any search
        // to wait for the engine to finish initializing.
        // This command must always be answered with "readyok" and can be sent also when the engine is calculating
        // in which case the engine should also immediately answer with "readyok" without stopping the search.
        writeln!(stdout, "readyok")?;
        std::io::stdout().flush()?;
        Ok(())
    }

    fn cmd_go(&mut self, words: Vec<&str>) -> Result<()> {
        // * Before the engine is asked to search on a position, there will always be a position command
        //   to tell the engine about the current position.
        // if engine.game.board.is_none() {
        //     return Err()
        // }
        //  * the engine must always be able to process input from stdin, even while thinking.
        //  -> We open a thread for each search
        //
        // * The engine will always be in forced mode which means it should never start calculating
        //   or pondering without receiving a "go" command first.
        //
        // start calculating on the current position set up with the "position" command.
        // There are a number of commands that can follow this command, all will be sent in the same string.
        // If one command is not sent its value should be interpreted as it would not influence the search.
        //
        // * searchmoves <move1> .... <movei>
        // 	restrict search to this moves only
        // 	Example: After "position startpos" and "go infinite searchmoves e2e4 d2d4"
        // 	the engine should only search the two moves e2e4 and d2d4 in the initial position.
        //
        // * ponder
        // 	start searching in pondering mode.
        // 	Do not exit the search in ponder mode, even if it's mate!
        // 	This means that the last move sent in in the position string is the ponder move.
        // 	The engine can do what it wants to do, but after a "ponderhit" command
        // 	it should execute the suggested move to ponder on. This means that the ponder move sent by
        // 	the GUI can be interpreted as a recommendation about which move to ponder. However, if the
        // 	engine decides to ponder on a different move, it should not display any mainlines as they are
        // 	likely to be misinterpreted by the GUI because the GUI expects the engine to ponder
        //    on the suggested move.
        //
        // * wtime <x>
        // 	white has x msec left on the clock
        // * btime <x>
        // 	black has x msec left on the clock
        // * winc <x>
        // 	white increment per move in mseconds if x > 0
        // * binc <x>
        // 	black increment per move in mseconds if x > 0
        // * movestogo <x>
        //  there are x moves to the next time control,
        // 	this will only be sent if x > 0,
        // 	if you don't get this and get the wtime and btime it's sudden death
        // * depth <x>
        // 	search x plies only.
        // * nodes <x>
        //    search x nodes only,
        // * mate <x>
        // 	search for a mate in x moves
        // * movetime <x>
        // 	search exactly x mseconds
        // * infinite
        // 	search until the "stop" command. Do not exit the search without being told so in this mode!
        //
        for w in &words[1..words.len()] {
            match *w {
                // "searchmoves" => {}
                // "pounder" => {}
                "wtime" => {}
                "btime" => {}
                "winc" => {}
                "binc" => {}
                // "movetogo" => {}
                "depth" => {}
                "nodes" => {}
                // "movetime" => {}
                // "mate" => {}
                "infinite" => {}
                _ => continue,
            }
        }
        let mut engine_handle = self.clone();
        engine_handle
            .search_ctx
            .stop
            .store(false, Ordering::Relaxed);
        thread::spawn(move || {
            let mv_str = engine_handle
                .search()
                .map(|mv| mv.to_uci())
                .unwrap_or_else(|| "0000".to_string());
            println!("bestmove {}", mv_str);
            eprintln!("la");
            eprintln!("{}", format!("ici: {}", mv_str));
            let _ = std::io::stdout().flush();
        });
        Ok(())
    }

    fn cmd_stop(&self) -> Result<()> {
        // * if the engine receives a command which is not supposed to come, for example "stop" when the engine is
        //   not calculating, it should also just ignore it.
        //
        // stop calculating as soon as possible,
        // don't forget the "bestmove" and possibly the "ponder" token when finishing the search
        //
        self.search_ctx.stop.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn cmd_ponderhit(&self) -> Result<()> {
        // the user has played the expected move. This will be sent if the engine was told to ponder on the same move
        // the user has played. The engine should continue searching but switch from pondering to normal search.
        Ok(())
    }

    fn cmd_unknown(&self, line: &str) -> Result<()> {
        // * if the engine or the GUI receives an unknown command or token it should just ignore it and try to
        //   parse the rest of the string in this line.
        //   Examples: "joho debug on\n" should switch the debug mode on given that joho is not defined,
        //             "debug joho on\n" will be undefined however.
        // pour chaque line un splitword
        println!("cmd not found: {line}");
        std::io::stdout().flush()?;
        Ok(())
    }

    fn cmd_debug(&self, _words: Vec<&str>) -> Result<()> {
        // * debug [ on | off ]
        // switch the debug mode of the engine on and off.
        // In debug mode the engine should send additional infos to the GUI, e.g. with the "info string" command,
        // to help debugging, e.g. the commands that the engine has received etc.
        // This mode should be switched off by default and this command can be sent
        // any time, also when the engine is thinking.
        //
        // On stockfish example :
        println!("Unknown command: 'debug on'");
        std::io::stdout().flush()?;
        Ok(())
    }

    fn cmd_register(&self) -> Result<()> {
        // this is the command to try to register an engine or to tell the engine that registration
        // will be done later. This command should always be sent if the engine	has sent "registration error"
        // at program startup.
        // The following tokens are allowed:
        // * later
        //    the user doesn't want to register the engine now.
        // * name <x>
        //    the engine should be registered with the name <x>
        // * code <y>
        //    the engine should be registered with the code <y>
        // Example:
        //    "register later"
        //    "register name Stefan MK code 4359874324"
        Ok(())
    }

    fn cmd_uci(&self) -> Result<()> {
        // tell engine to use the uci (universal chess interface),
        // this will be sent once as a first command after program boot
        // to tell the engine to switch to uci mode.
        // After receiving the uci command the engine must identify itself with the "id" command
        // and send the "option" commands to tell the GUI which engine settings the engine supports if any.
        // After that the engine should send "uciok" to acknowledge the uci mode.
        // If no uciok is sent within a certain time period, the engine task will be killed by the GUI.
        println!("id name ChessGame");
        println!("id author Muffin");
        println!("uciok");
        std::io::stdout().flush()?;
        Ok(())
    }
}
