use crate::Board;
use crate::ChessApp;
use crate::Color::*;
use crate::Coord;
use crate::game::End;
use crate::game::GameEvent::*;
use crate::gui::chessapp::AppMode::*;

impl ChessApp {
    pub fn try_move(&mut self, from: Coord, to: Coord) {
        let mover = self.game.active_player;
        let is_first = self.game.history.is_empty();

        if let Some(event) = self.game.try_move(from, to) {
            if is_first {
                self.app_mode = Versus(None);
                self.init_timer();
            }

            if self.game.draw.draw_option.is_some() && self.is_bot_turn() {
                self.game.end = Some(End::Draw);
            }

            self.last_move = Some((from, to));
            if self.settings.autoflip {
                self.settings.flip = !self.settings.flip;
            }

            match mover {
                White => self.white_last_score = self.game.board.score,
                Black => self.black_last_score = self.game.board.score,
            }

            match event {
                PromotionPending(coord) => {
                    let prev_board = self.game.board_at(self.game.history.len() - 1);
                    self.promoteinfo = Some(crate::gui::hooks::promote::PromoteInfo {
                        from,
                        to,
                        prev_board,
                        pawn_to_promote: Some(coord),
                        promote: None,
                    });
                }
                Checkmate => {
                    self.app_mode = Versus(Some(End::Checkmate));
                    self.timer.active = false;
                    self.add_history_san(
                        &from,
                        &to,
                        &self.game.board_at(self.game.history.len() - 1).clone(),
                    );
                }
                Stalemate => {
                    self.app_mode = Versus(Some(End::Pat));
                    self.timer.active = false;
                    self.add_history_san(
                        &from,
                        &to,
                        &self.game.board_at(self.game.history.len() - 1).clone(),
                    );
                }
                Draw => {
                    self.app_mode = Versus(Some(End::Draw));
                    self.add_history_san(
                        &from,
                        &to,
                        &self.game.board_at(self.game.history.len() - 1).clone(),
                    );
                }
                Ok | Check => {
                    let prev_board = self.game.board_at(self.game.history.len() - 1);
                    self.add_history_san(&from, &to, &prev_board);
                    // if self.game.end.is_none() && self.is_bot_turn() {
                    //     self.bot_pending = true;
                    // }
                }
            }
        }
        if self.is_bot_turn() {
            self.bot_pending = true;
        }
    }

    pub fn add_history_san(&mut self, from: &Coord, to: &Coord, prev_board: &Board) {
        self.replay_infos.index += 1;
        self.encode_move_to_san(from, to, prev_board);
    }

    pub fn init_timer(&mut self) {
        self.timer.active = true;
        self.timer.start_of_turn.1 = Some(White);
    }

    pub fn update_threaten_cells(&mut self) {
        self.game.threaten_cells = self
            .game
            .board
            .update_threatens_cells(&self.game.active_player);
    }

    pub fn update_legals_moves(&mut self) {
        use crate::board::moves::move_gen::generate_moves;
        use crate::board::moves::move_structs::MoveList;
        let mut move_list = MoveList::new();
        generate_moves(
            &mut self.game.board,
            &self.game.active_player,
            &mut move_list,
            false,
        );
        self.game.legals_moves = move_list.moves[..move_list.count].to_vec();
    }
}
