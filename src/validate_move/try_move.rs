use crate::ChessApp;
use crate::Color;
use crate::Color::*;
use crate::Board;
use crate::Coord;
use crate::board::cell::Piece::*;
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
        self.undo.push(self.current.clone());
        self.current
            .board
            .update_board(&from, &to, &self.current.active_player);
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
                self.promoteinfo = Some(PromoteInfo { from: from, to: to, prev_board: prev_board.clone() });
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
        self.current.active_player = match self.current.active_player {
            White => Black,
            Black => White,
        };
        let (end, mate) = mat_or_pat(&mut self.current.board, &self.current.active_player);
        if end {
            if mate {
                self.current.checkmate = true;
            } else {
                self.current.pat = true;
            }
        }

        // println!("{:?} to move", self.current.active_player);
        let active_player = if self.current.active_player == White { White } else { Black };
        let opponent = if self.current.active_player != White { White } else { Black };

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

pub fn mat_or_pat(board: &mut Board, color: &Color) -> (bool, bool) {
    board.update_threatens_cells(color);
    board.update_legals_moves(color);
    // for coord in &board.threaten_cells {
    //     println!("Cell threaten : ({}, {})", coord.row, coord.col);
    // }
    if board.legals_moves.is_empty() {
        board.print();
        let king_cell = board.get_king(color);
        if let Some(coord) = king_cell {
            if board.threaten_cells.contains(&coord) {
                let winner = if *color == White {
                    Black
                } else {
                    White
                };
                println!("Checkmate ! {:?} win", winner);
                return (true, true);
            } else {
                println!("Pat");
                return (true, false);
            }
        }
    }
    (false, false)
}
