use crate::ChessApp;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::PromoteInfo;
use crate::validate_move;
use crate::gui::chessapp_struct::GameState;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::gui::chessapp_struct::End::*;

impl ChessApp {
    pub fn add_hash(&mut self) {
        let mut hasher = DefaultHasher::new();
        self.current.board.grid.hash(&mut hasher);
        let hash_value = hasher.finish();
        let count = self.board_hashs.entry(hash_value).or_insert(0);
        *count += 1;
        if *count >= 3 {
            self.current.end = Some(Draw);
        }
    }

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
        self.undo.push(self.current.clone());
        self.current
            .board
            .update_board(&from, &to, &self.current.active_player);
        self.add_hash();
        self.update_castles(&to);
        self.redo.clear();
        self.current.last_move = Some((from, to));
        if self.autoflip {
            self.flip = !self.flip;
        }
        self.incremente_turn();
        self.events_check();
        if let Some(prev_state) = self.undo.last() {
            let prev_board = &prev_state.board.clone();
            if self.current.board.pawn_to_promote.is_some() {
                self.promoteinfo = Some(PromoteInfo {
                    from: from,
                    to: to,
                    prev_board: prev_board.clone(),
                });
            } else {
                self.from_move_to_san(&from, &to, &prev_board);
            }
        }
    }

    fn incremente_turn(&mut self) {
        if self.current.active_player == Black {
            self.current.turn += 1;
        }
    }

    fn update_castles(&mut self, to: &Coord) {
        let mut castle_tuple = self
            .current
            .board
            .get_castle_tuple(&self.current.active_player);
        if let Some(piece) = self.current.board.get(&to).get_piece() {
            match piece {
                Rook => {
                    match to.col {
                        7 => castle_tuple.0 = false,
                        0 => castle_tuple.1 = false,
                        _ => {}
                    };
                }
                King => {
                    castle_tuple.0 = false;
                    castle_tuple.1 = false;
                }
                _ => {}
            }
        };
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

        if let Some(k) = self.current.board.get_king(&active_player) {
            if self.current.board.threaten_cells.contains(&k) {
                if let Some(k) = self.current.board.get_king(&opponent) {
                    self.current.board.check = Some(k);
                }
                // println!("Check !");
            }
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
}


