use crate::Coord;
use crate::Color;
use crate::ChessApp;
use crate::cell::Piece::*;
use crate::validate_move;
use crate::mat_or_pat;

impl ChessApp {

    pub fn try_move(&mut self, from: Coord, to: Coord) {
        if !self.current.board.is_legal_move(&from, &to, &self.current.active_player) {
            println!("Illegal move: {from:?} -> {to:?}");
            return ;
        }
        if validate_move::is_king_exposed(&from, &to, &self.current.active_player, &self.current.board) {
            println!("King is exposed: illegal move");
            return ;
        }
        self.from_move_to_pgn((from, to));
        self.add_history();
        println!("{:?} : {:?}", self.current.active_player, self.current.last_move_pgn);
        self.undo.push(self.current.clone());
        self.current.board.update_board(&from, &to, &self.current.active_player);
        self.update_castles(&to);
        self.redo.clear();
        self.current.last_move = Some((from, to));
        if self.autoflip {
            self.flip = !self.flip;
        }
        self.incremente_turn();
        self.events_check();
    }

    fn incremente_turn(&mut self) {
        if self.current.active_player == Color::Black {
            self.current.turn += 1;
        }
    }
    
    fn add_history(&mut self) {
        if self.current.active_player == Color::White {
            let turn_str = self.current.turn.to_string();
            if self.current.history_pgn.is_empty() {
            } else {
                self.current.history_pgn.push(' ');
            }
            self.current
                .history_pgn
                .push_str(&format!("{}. {}", turn_str, self.current.last_move_pgn));
        } else {
            if !self.current.history_pgn.is_empty() {
                self.current.history_pgn.push(' ');
            }
            self.current.history_pgn.push_str(&self.current.last_move_pgn);
        }
    }

    fn update_castles(&mut self, to: &Coord) {
        let mut castle_tuple = self.current.board.get_castle_tuple(&self.current.active_player);
        if let Some(piece) = self.current.board.get(&to).get_piece() {
            match piece {
                Rook =>  {
                    match to.col {
                        7 => castle_tuple.0 = false,
                        0 => castle_tuple.1 = false,
                        _ => { } 
                    };
                },
                King => {
                    castle_tuple.0 = false;
                    castle_tuple.1 = false;
                }
                _ => { }
            }
        }; 
    }

    fn events_check(&mut self) {
        self.current.board.promote_pawn(&self.current.active_player);
        self.current.active_player = match self.current.active_player { Color::White => Color::Black, Color::Black => Color::White };
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

    pub fn from_move_to_pgn(&mut self, move_to_encode: (Coord, Coord)) {
        let (from, to) = move_to_encode;
        let piece_char = match self.current.board.get(&from).get_piece() {
            Some(Pawn)  => None,
            Some(Rook)  => Some('R'),
            Some(Knight)=> Some('N'),
            Some(Bishop)=> Some('B'),
            Some(Queen) => Some('Q'),
            Some(King)  => Some('K'),
            None        => Some('?'),
        };

        let is_capture = !self.current.board.get(&to).is_empty();

        let dest_col = (b'a' + to.col as u8) as char; 
        let dest_row = char::from_digit((to.row + 1) as u32, 10).unwrap(); 

        self.current.last_move_pgn = String::new();
        if let Some(pc) = piece_char {
            self.current.last_move_pgn.push(pc);
        } else if is_capture {
            let src_col = (b'a' + from.col as u8) as char;
            self.current.last_move_pgn.push(src_col);
        }

        if is_capture {
            self.current.last_move_pgn.push('x');
        }

        self.current.last_move_pgn.push(dest_col);
        self.current.last_move_pgn.push(dest_row);
        //ajouter le check
        //ajouter le mat / pat
        //ajouter la promotion
        //ajouter ambiguite
        //ajouter les roques
    } 
}
