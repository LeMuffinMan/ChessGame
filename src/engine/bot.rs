use crate::Board;
use crate::ChessApp;
use crate::Color;
use crate::board::cell::Cell;
use crate::board::cell::Color::*;
use crate::board::moves::move_gen::generate_moves;
use crate::board::moves::move_structs::Move;
use crate::board::moves::move_structs::MoveList;
use crate::board::moves::move_structs::MoveType::Promotion;
use crate::engine::bot::BotDifficulty::*;
use crate::engine::bot::PlayerType::*;
use crate::engine::minimax::iterative_deepening;
use crate::engine::search_context::{SearchContext, SearchParams};
use crate::gui::chessapp::AppMode::*;
use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        * 1000.0
}

#[cfg(target_arch = "wasm32")]
fn random_index(len: usize) -> usize {
    (js_sys::Math::random() * len as f64).floor() as usize
}

#[cfg(not(target_arch = "wasm32"))]
fn random_index(len: usize) -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as usize;
    nanos % len
}

const BOT_TIMEOUT: f64 = 150.0;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum BotDifficulty {
    Random,
    Depth(u8),
    Adaptive,
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PlayerType {
    Human,
    Bot(BotDifficulty),
}

pub fn get_bot_move(
    difficulty: &PlayerType,
    board: &mut Board,
    active_player: Color,
    ctx: &mut SearchContext,
    game_history: &HashMap<u64, usize>,
    fifty_count: u32,
    depth: &mut u8,
) -> Option<Move> {
    match difficulty {
        Bot(Depth(d)) => {
            let mut params = SearchParams::new(ctx, game_history, fifty_count);
            iterative_deepening(board, active_player, *d, depth, 0.0, &mut params)
        }
        Bot(Adaptive) => {
            let mut params = SearchParams::new(ctx, game_history, fifty_count);
            iterative_deepening(board, active_player, 11, depth, BOT_TIMEOUT, &mut params)
        }
        Bot(Random) => {
            let mut move_list = MoveList::new();
            generate_moves(board, &active_player, &mut move_list, false);
            let moves = &mut move_list.moves[..move_list.count];
            let index = random_index(moves.len());
            Some(moves[index])
        }
        _ => None,
    }
}

impl ChessApp {
    pub fn get_depth(&self) -> u8 {
        if let Bot(diff) = match self.game.active_player {
            White => self.settings.black_bot,
            Black => self.settings.white_bot,
        } {
            match diff {
                BotDifficulty::Depth(d) => d,
                BotDifficulty::Random => 0,
                BotDifficulty::Adaptive => self.game.depth,
            }
        } else {
            0
        }
    }
    pub fn is_bot_turn(&self) -> bool {
        match self.game.active_player {
            White => matches!(self.settings.white_bot, Bot(_)),
            Black => matches!(self.settings.black_bot, Bot(_)),
        }
    }

    pub fn start_bot_game(&mut self) {
        self.app_mode = Versus(None);
        self.timer.active = true;
        self.timer.start_of_turn.1 = Some(White);
        self.bot_pending = true;
        self.search_ctx.reset_for_new_game();
    }

    pub fn play_bot_turn(&mut self) {
        let difficulty = match self.game.active_player {
            White => &self.settings.white_bot,
            Black => &self.settings.black_bot,
        };
        self.search_ctx.reset_search_stats();
        let start = now_ms();
        let bot_move = get_bot_move(
            difficulty,
            &mut self.game.board,
            self.game.active_player,
            &mut self.search_ctx,
            &self.game.draw.board_hashs,
            self.game.draw.draw_moves_count,
            &mut self.game.depth,
        );
        let end = now_ms();
        self.search_ctx.stats.bot_time_thinking = end - start;
        self.search_ctx.stats.nps();
        if let Some(m) = bot_move {
            let bot_color = self.game.active_player;
            self.try_move(m.origin, m.dest);
            if let Promotion(piece) = m.move_type {
                self.game.board[(m.dest.row as usize, m.dest.col as usize)] =
                    Cell::Occupied(piece, bot_color);
                self.promoteinfo = None;
                self.win = None;
                if self.game.end.is_none() && self.is_bot_turn() {
                    self.bot_pending = true;
                }
            }
        }
    }
}
