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
use crate::engine::evaluator::PositionalEvaluator;
use crate::engine::minimax::iterative_deepening;
use crate::engine::search_context::SearchContext;
use crate::gui::chessapp::AppMode::*;
use js_sys::Math;
use web_sys::window;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum BotDifficulty {
    Random,
    Depth(u8),
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
) -> Option<Move> {
    match difficulty {
        Bot(Depth(d)) => iterative_deepening(board, active_player, &PositionalEvaluator, *d, ctx),
        Bot(Random) => {
            let mut move_list = MoveList::new();
            generate_moves(board, &active_player, &mut move_list, false);
            let moves = &mut move_list.moves[..move_list.count];
            let index = (Math::random() * moves.len() as f64).floor() as usize;
            Some(moves[index])
        }
        _ => None,
    }
}

impl ChessApp {
    pub fn get_depth(&self) -> u8 {
        if let Bot(diff) = match self.current.active_player {
            White => self.settings.black_bot,
            Black => self.settings.white_bot,
        } {
            match diff {
                BotDifficulty::Depth(d) => d,
                BotDifficulty::Random => 0,
            }
        } else {
            0
        }
    }
    pub fn is_bot_turn(&self) -> bool {
        match self.current.active_player {
            White => matches!(self.settings.white_bot, Bot(_)),
            Black => matches!(self.settings.black_bot, Bot(_)),
        }
    }

    pub fn start_bot_game(&mut self) {
        let snapshot = self.current.clone();
        self.history.snapshots.push(snapshot);
        self.replay_infos.index += 1;
        self.app_mode = Versus(None);
        self.timer.active = true;
        self.timer.start_of_turn.1 = Some(White);
        self.bot_pending = true;
        self.search_ctx.reset_for_new_game();
    }

    pub fn play_bot_turn(&mut self) {
        let difficulty = match self.current.active_player {
            White => &self.settings.white_bot,
            Black => &self.settings.black_bot,
        };
        let performance = window().unwrap().performance().unwrap();
        self.search_ctx.reset_stats();
        let start = performance.now();
        let bot_move = get_bot_move(
            difficulty,
            &mut self.current.board,
            self.current.active_player,
            &mut self.search_ctx,
        );
        let end = performance.now();
        self.search_ctx.stats.bot_time_thinking = end - start;
        self.search_ctx.stats.nps();
        if let Some(m) = bot_move {
            let bot_color = self.current.active_player;
            self.try_move(m.origin, m.dest);
            if let Promotion(piece) = m.move_type {
                self.current.board.grid[m.dest.row as usize][m.dest.col as usize] =
                    Cell::Occupied(piece, bot_color);
                self.promoteinfo = None;
                self.win = None;
                if self.current.end.is_none() && self.is_bot_turn() {
                    self.bot_pending = true;
                }
            }
            // match difficulty {
            //     Bot(BotDifficulty::Easy) => {
            //         let snapshot = self.current.clone();
            //         self.apply_move(&m);
            //         self.commit_move(snapshot, m, m.origin, m.dest);
            //         if let Promotion(piece) = m.move_type {
            //             self.current.board.grid[m.dest.row as usize][m.dest.col as usize] =
            //                 Cell::Occupied(piece, self.current.active_player);
            //         }
            //         self.switch_turn();
            //         if self.current.end.is_none() && self.is_bot_turn() {
            //             self.bot_pending = true;
            //         }
            //     }
            //     Bot(BotDifficulty::Medium) | Bot(BotDifficulty::Hard) => {
            //         let bot_color = self.current.active_player;
            //         self.try_move(m.origin, m.dest);
            //         if let Promotion(piece) = m.move_type {
            //             self.current.board.grid[m.dest.row as usize][m.dest.col as usize] =
            //                 Cell::Occupied(piece, bot_color);
            //             self.promoteinfo = None;
            //             self.win = None;
            //             if self.current.end.is_none() && self.is_bot_turn() {
            //                 self.bot_pending = true;
            //             }
            //         }
            //     }
            //     _ => {
            //         unreachable!("Player can't reach this branch")
            //     }
            // }
        }
    }
}
