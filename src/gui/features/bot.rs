<<<<<<< HEAD
use crate::board::cell::Cell;
use crate::board::cell::Color::*;
use crate::board::move_gen::MoveType::Promotion;
use crate::engine::minimax::get_bot_move;
use crate::gui::chessapp::AppMode::*;
use crate::gui::chessapp::ChessApp;
use crate::gui::features::bot::BotDifficulty::*;
use crate::gui::features::bot::PlayerType::*;
use web_sys::window;

#[derive(PartialEq, Debug)]
=======
#[derive(PartialEq, Debug, Copy, Clone)]
>>>>>>> 2b166a4 (feat: display depth on panel)
pub enum BotDifficulty {
    Easy,
    Medium,
    Hard,
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PlayerType {
    Human,
    Bot(BotDifficulty),
}

impl ChessApp {
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
    }

    pub fn play_bot_turn(&mut self) {
        let difficulty = match self.current.active_player {
            White => &self.settings.white_bot,
            Black => &self.settings.black_bot,
        };
        let performance = window().unwrap().performance().unwrap();
        self.stats.nodes = 0;
        self.stats.cutoffs = 0;
        self.stats.killer_moves = [[None; 2]; 64];
        let start = performance.now();
        let bot_move = get_bot_move(
            difficulty,
            &mut self.current.board,
            self.current.active_player,
            &mut self.stats,
        );
        let end = performance.now();
        self.stats.bot_time_thinking = end - start;
        self.stats.nps();
        if let Some(m) = bot_move {
            match difficulty {
                Bot(Easy) => {
                    let snapshot = self.current.clone();
                    self.apply_move(&m);
                    self.commit_move(snapshot, m, m.origin, m.dest);
                    if let Promotion(piece) = m.move_type {
                        self.current.board.grid[m.dest.row as usize][m.dest.col as usize] =
                            Cell::Occupied(piece, self.current.active_player);
                    }
                    self.switch_turn();
                    if self.current.end.is_none() && self.is_bot_turn() {
                        self.bot_pending = true;
                    }
                }
                Bot(Medium) | Bot(Hard) => {
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
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }
}
