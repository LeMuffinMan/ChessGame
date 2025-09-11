use crate::ChessApp;
use crate::Color;
use crate::Board;
use crate::Coord;
use crate::cell::Piece::*;
use crate::mat_or_pat;
use crate::validate_move;
use crate::gui::chessapp_struct::PromoteInfo;

impl ChessApp {
    // fn update_last_move_pgn(&mut self) {
    //     if self.current.checkmate {
    //         self.current.last_move_pgn.push_str("# ");
    //     } else if self.current.board.check.is_some() {
    //         self.current.last_move_pgn.push_str("+ ");
    //         if self.current.active_player == Color::White {
    //             self.current.last_move_pgn.push_str("1-0");
    //         } else {
    //             self.current.last_move_pgn.push_str("0-1");
    //         }
    //     } else if self.current.pat {
    //         self.current.last_move_pgn.push_str("1/2 - 1/2");
    //     } else {
    //         self.current.last_move_pgn.push_str(" ");
    //     }
    // }
    //


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
        // self.add_history();
        // println!(
        //     "{:?} : {:?}",
        //     self.current.active_player, self.current.last_move_pgn
        // );
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
                self.from_move_to_pgn(&from, &to, &prev_board);
            }
        }
    }

    fn incremente_turn(&mut self) {
        if self.current.active_player == Color::Black {
            self.current.turn += 1;
        }
    }

    // fn add_history(&mut self) {
    //     if self.current.active_player == Color::White {
    //         let turn_str = self.current.turn.to_string();
    //         if self.current.history_pgn.is_empty() {
    //         } else {
    //             self.current.history_pgn.push(' ');
    //         }
    //         self.current
    //             .history_pgn
    //             .push_str(&format!("{}. {}", turn_str, self.current.last_move_pgn));
    //     } else {
    //         if !self.current.history_pgn.is_empty() {
    //             self.current.history_pgn.push(' ');
    //         }
    //         self.current
    //             .history_pgn
    //             .push_str(&self.current.last_move_pgn);
    //     }
    // }

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
            Color::White => Color::Black,
            Color::Black => Color::White,
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
        if let Some(k) = self.current.board.get_king(&self.current.active_player) {
            if self.current.board.threaten_cells.contains(&k) {
                self.current.board.check = Some(k);
                // println!("Check !");
            }
        }
    }
    
    fn is_en_passant_take(&mut self, from: &Coord, to: &Coord, prev_board: &Board) -> bool {
        let row_en_passant = if self.current.active_player == Color::White { 5 } else { 4 };
        let diff: i8 = if self.current.active_player == Color::White { -1 } else { 1 };
        if from.row == row_en_passant {
            let new_row = from.row as i8 + diff;
            let coord = Coord { row: new_row as u8, col: to.col };
            if self.current.board.get(&coord).get_piece() != prev_board.get(&coord).get_piece() {
                return true;
            }
        }
        return false;
    }

    fn pawn_pgn(&mut self, from: &Coord, to: &Coord, prev_board: &Board) {
        if  !prev_board.get(&to).is_empty() 
            || self.is_en_passant_take(from, to, prev_board) {
            self.current.history_pgn.push((b'a' + from.col as u8) as char);
            self.current.history_pgn.push('x');
        } 
        self.current.history_pgn.push((b'a' + to.col as u8) as char);
        self.current.history_pgn.push((b'0' + to.row + 1) as char);

        if let Some(piece) = self.current.board.get(&to).get_piece() {
            if *piece != Pawn {
                self.current.history_pgn.push('=');
                let p = match piece {
                    Rook => 'R',
                    Knight => 'N',
                    Bishop => 'B',
                    Queen => 'Q',
                    _ => '_',
                };
                self.current.history_pgn.push(p);
            }
        }
    }

    pub fn from_move_to_pgn(&mut self, from: &Coord, to: &Coord, prev_board: &Board) {

        //on ecrit le dernier coup une fois les checks du tour suivant faits
        if self.current.active_player == Color::Black {
            self.current.history_pgn.push_str(self.current.turn.to_string().as_str());
            self.current.history_pgn.push_str(". ");
        } 

        let piece: char = if let Some(p) = prev_board.get(from).get_piece() {
            match p {
                Pawn => 'p',
                Rook => 'R',
                Knight => 'N',
                Bishop => 'B',
                Queen => 'Q',
                King => 'K',
            }
        } else { '?' };

        if piece == 'p' {
            self.pawn_pgn(from, to, prev_board);
        } else if piece == 'K' && (to.col as i8 - from.col as i8).abs() > 1 {
            if (to.col as i8 - from.col as i8) < 0 {
                self.current.history_pgn.push_str("O-O-O");
            } else {
                self.current.history_pgn.push_str("O-O");
            }
        } else {
            self.current.history_pgn.push(piece);

            //ambiguite ici
            
            if !self.current.board.get(&to).is_empty() {
                self.current.history_pgn.push('x');
            }
            self.current.history_pgn.push((b'a' + to.col) as char);
            self.current.history_pgn.push((b'0' + to.row + 1) as char);
        }

        //endgame and checks
        if self.current.checkmate {
            self.current.history_pgn.push_str("# ");
            if self.current.active_player == Color::White {
                self.current.history_pgn.push_str("0-1");
            } else {
                self.current.history_pgn.push_str("1-0");
            }
        } else if self.current.pat {
            self.current.history_pgn.push_str(" 1/2 - 1/2");

        } else if self.current.board.check.is_some() {
            self.current.history_pgn.push_str("+ ");
        } else {
            self.current.history_pgn.push(' ');
        }
    }

    // pub fn from_move_to_pgn(&mut self, move_to_encode: (Coord, Coord)) {
    //     let (from, to) = move_to_encode;
    //     let piece_char = match self.current.board.get(&from).get_piece() {
    //         Some(Pawn) => None,
    //         Some(Rook) => Some('R'),
    //         Some(Knight) => Some('N'),
    //         Some(Bishop) => Some('B'),
    //         Some(Queen) => Some('Q'),
    //         Some(King) => Some('K'),
    //         None => Some('?'),
    //     };
    //
    //     let is_capture = !self.current.board.get(&to).is_empty();
    //
    //     let dest_col = (b'a' + to.col as u8) as char;
    //     let dest_row = char::from_digit((to.row + 1) as u32, 10).unwrap();
    //
    //     self.current.last_move_pgn = String::new();
    //     if let Some(pc) = piece_char {
    //         self.current.last_move_pgn.push(pc);
    //     } else if is_capture {
    //         let src_col = (b'a' + from.col as u8) as char;
    //         self.current.last_move_pgn.push(src_col);
    //     }
    //
    //     if is_capture {
    //         self.current.last_move_pgn.push('x');
    //     }
    //
    //     self.current.last_move_pgn.push(dest_col);
    //     self.current.last_move_pgn.push(dest_row);
        //ajouter le check
        //ajouter le mat / pat
        //ajouter la promotion
        //ajouter ambiguite
        //ajouter les roques
    // }
}
