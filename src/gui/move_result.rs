use crate::Coord;
use crate::Color;
use crate::ChessApp;
use crate::cell::Piece::*;
use crate::validate_move;
use crate::mat_or_pat;

impl ChessApp {

    pub fn try_apply_move(&mut self, from: Coord, to: Coord) {
        if !self.current.board.is_legal_move(&from, &to, &self.current.active_player) {
            println!("Illegal move: {from:?} -> {to:?}");
            return ;
        }
        if validate_move::is_king_exposed(&from, &to, &self.current.active_player, &self.current.board) {
            println!("King is exposed: illegal move");
            return ;
        }
        self.from_move_to_pgn((from, to));
        self.undo.push(self.current.clone());
        self.current.board.update_board(&from, &to, &self.current.active_player);
        if let Some(piece) = self.current.board.get(&to).get_piece() {
            match piece {
                Rook =>  {
                    if to.col == 7 {
                        if self.current.active_player == Color::White {
                            // println!("switching little white castle to false");
                            self.current.board.white_castle.0 = false;
                        } else {
                            // println!("switching little black castle to false");
                            self.current.board.black_castle.0 = false;
                        }
                    } else if to.col == 0 {
                        if self.current.active_player == Color::White {
                            // println!("switching long white castle to false");
                            self.current.board.white_castle.1 = false;
                        } else {
                            self.current.board.black_castle.1 = false;
                        }
                    }
                },
                King => {
                    if self.current.active_player == Color::White {
                        self.current.board.white_castle.0 = false;
                        self.current.board.white_castle.1 = false;
                    } else {
                        self.current.board.black_castle.0 = false;
                        self.current.board.black_castle.1 = false;
                    }
                }
                _ => { }
            }
        }; 
        self.redo.clear();
        self.current.last_move = Some((from, to));
        if self.autoflip {
            self.flip = !self.flip;
        }
        if self.current.active_player == Color::Black {
            self.current.turn += 1;
        }
        self.current.active_player = match self.current.active_player { Color::White => Color::Black, Color::Black => Color::White };

        let (end, mate) = mat_or_pat(&mut self.current.board, &self.current.active_player);
        if end {
            if mate {
                self.current.checkmate = true;
            } else {
                self.current.pat = true;
            }
        }

        println!("{:?} to move", self.current.active_player);
        if let Some(k) = self.current.board.get_king(&self.current.active_player) {
            if self.current.board.threaten_cells.contains(&k) {
                self.current.board.check = true;
                println!("Check !");
            }
        }
    }

}
