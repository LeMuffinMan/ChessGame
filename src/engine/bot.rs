use crate::Board;
use crate::ChessApp;
use crate::Color;
use crate::board::cell::Color::*;
use crate::board::moves::move_gen::generate_moves;
use crate::board::moves::move_structs::Move;
use crate::board::moves::move_structs::MoveList;
use crate::board::moves::move_structs::MoveType::Promotion;
use crate::engine::bot::BotDifficulty::*;
use crate::engine::bot::PlayerType::*;
use crate::engine::minimax::iterative_deepening;
use crate::engine::search_context::SearchParams;
use crate::engine::time_manager::{Budget, DEFAULT_MOVE_OVERHEAD_MS, TimeControl, plan};
use crate::gui::chessapp::AppMode::*;
use crate::gui::features::timer::{GameMode, Timer};

pub const MAX_DEPTH: u8 = 16;

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

const UI_RESPONSIVENESS_CAP_MS: f64 = 300.0;
const NO_CLOCK_BUDGET_MS: f64 = 300.0;

fn adaptive_budget(timer: &Timer, active_player: Color, legal_moves: usize) -> Budget {
    if timer.mode == GameMode::NoTime || !timer.active {
        return Budget::fixed(NO_CLOCK_BUDGET_MS);
    }
    let remaining_s = match active_player {
        White => timer.white_time,
        Black => timer.black_time,
    };
    plan(
        &TimeControl {
            remaining_ms: remaining_s * 1000.0,
            increment_ms: timer.increment * 1000.0,
            moves_to_go: None,
        },
        legal_moves,
        DEFAULT_MOVE_OVERHEAD_MS,
    )
    .capped_at(UI_RESPONSIVENESS_CAP_MS)
}

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
    params: &mut SearchParams,
    depth: &mut u8,
    timer: &Timer,
) -> Option<Move> {
    match difficulty {
        Bot(Depth(d)) => {
            iterative_deepening(board, active_player, *d, depth, Budget::UNLIMITED, params)
        }
        Bot(Adaptive) => {
            let mut move_list = MoveList::new();
            generate_moves(board, &active_player, &mut move_list, false);
            let budget = adaptive_budget(timer, active_player, move_list.count);
            iterative_deepening(board, active_player, MAX_DEPTH, depth, budget, params)
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
        let bot_move = {
            let mut params = SearchParams::new(
                &mut self.search_ctx,
                &self.game.draw.board_hashs,
                self.game.draw.draw_moves_count,
            );
            get_bot_move(
                difficulty,
                &mut self.game.board,
                self.game.active_player,
                &mut params,
                &mut self.game.depth,
                &self.timer,
            )
        };
        let end = now_ms();
        self.search_ctx.stats.bot_time_thinking = end - start;
        self.search_ctx.stats.nps();
        if let Some(m) = bot_move {
            if let Promotion(piece) = m.move_type {
                let prev_board = self.game.board.clone();
                if let Some(event) = self.game.try_move_promotion(m.origin, m.dest, piece) {
                    use crate::game::End;
                    use crate::game::GameEvent::*;
                    use crate::gui::chessapp::AppMode::Versus;

                    self.last_move = Some((m.origin, m.dest));
                    match event {
                        Checkmate => {
                            self.app_mode = Versus(Some(End::Checkmate));
                            self.timer.active = false;
                        }
                        Stalemate => {
                            self.app_mode = Versus(Some(End::Pat));
                            self.timer.active = false;
                        }
                        Draw => {
                            self.app_mode = Versus(Some(End::Draw));
                        }
                        _ => {}
                    }
                    self.add_history_san(&m.origin, &m.dest, &prev_board);
                    if self.game.end.is_none() && self.is_bot_turn() {
                        self.bot_pending = true;
                    }
                    self.hint_highlight = 0;
                    self.game.hint = None;
                }
            } else {
                self.try_move(m.origin, m.dest);
            }
        }
    }
}
