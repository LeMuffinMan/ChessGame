use crate::ChessApp;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::End::*;
use crate::gui::chessapp_struct::GameState;
use crate::gui::chessapp_struct::PromoteInfo;
use crate::validate_move;

impl ChessApp {
    pub fn try_move(&mut self, from: Coord, to: Coord) {
        if !self
            .current
            .board
            .is_legal_move(&from, &to, &self.current.active_player)
        {
            println!("Illegal move: {from:?} -> {to:?}");
            return;
        }
        if validate_move::is_king_exposed(
            &from,
            &to,
            &self.current.active_player,
            &self.current.board,
        ) {
            println!("King is exposed: illegal move");
            return;
        }
        if self.history.is_empty() {
            self.history.push(self.current.clone());
            self.widgets.replay_index += 1;
        }
        //si ya un timer active, et que c'est le premier ocup joue
        self.fifty_moves_draw_check(&from, &to);
        self.current
            .board
            .update_board(&from, &to, &self.current.active_player);
        if self.impossible_mate_check() {
            self.current.end = Some(Draw);
        }
        self.update_castles(&to);
        self.add_hash();
        self.current.last_move = Some((from, to));
        if self.widgets.autoflip {
            self.widgets.flip = !self.widgets.flip;
        }
        self.incremente_turn();
        self.events_check();
        let prev_board = self.history[self.widgets.replay_index - 1].board.clone();
        if self.current.board.pawn_to_promote.is_some() {
            self.promoteinfo = Some(PromoteInfo {
                from,
                to,
                prev_board: prev_board.clone(),
            });
        } else {
            self.history.push(self.current.clone());
            self.widgets.replay_index += 1;
            self.encode_move_to_san(&from, &to, &prev_board);
        }
        log::info!("end try move {} {}", self.widgets.replay_index, self.history.len());
    }

    fn incremente_turn(&mut self) {
        if self.current.active_player == Black {
            self.current.turn += 1;
        }
    }

    fn update_castles(&mut self, to: &Coord) {
        if let Some(piece) = self.current.board.get(to).get_piece() {
            match piece {
                Rook => {
                    match to.col {
                        7 => self.current.switch_castle(false, true),
                        0 => self.current.switch_castle(true, false),
                        _ => {}
                    };
                }
                King => {
                    self.current.switch_castle(false, false);
                }
                _ => {}
            }
        };
    }

    pub fn check_endgame(&mut self) {
        self.current
            .board
            .update_threatens_cells(&self.current.active_player);
        self.current
            .board
            .update_legals_moves(&self.current.active_player);
        // for coord in &self.current.board.threaten_cells {
        //     println!("Cell threaten : ({}, {})", coord.row, coord.col);
        // }
        if self.current.board.legals_moves.is_empty() {
            self.current.board.print();
            let king_cell = self.current.board.get_king(&self.current.active_player);
            if let Some(coord) = king_cell {
                if self.current.board.threaten_cells.contains(&coord) {
                    self.current.end = Some(Checkmate);
                } else {
                    self.current.end = Some(Pat);
                }
            }
        }
    }

    fn events_check(&mut self) {
        self.current.board.promote_pawn(&self.current.active_player);
        self.current.switch_players_color();
        self.check_endgame();
        // println!("{:?} to move", self.current.active_player);
        let active_player = if self.current.active_player == White {
            White
        } else {
            Black
        };
        let opponent = if self.current.active_player != White {
            White
        } else {
            Black
        };

        if let Some(k) = self.current.board.get_king(&active_player)
            && self.current.board.threaten_cells.contains(&k)
            && let Some(k) = self.current.board.get_king(&opponent)
        {
            self.current.board.check = Some(k);
            // println!("Check !");
        }
    }
}

impl GameState {
    pub fn switch_players_color(&mut self) {
        self.active_player = match self.active_player {
            White => Black,
            Black => White,
        };
        self.opponent = match self.opponent {
            White => Black,
            Black => White,
        };
    }

    pub fn switch_castle(&mut self, long: bool, short: bool) {
        let castle_tuple = if self.active_player == White {
            &mut self.board.white_castle
        } else {
            &mut self.board.black_castle
        };
        castle_tuple.0 = long;
        castle_tuple.1 = short;
    }
}
