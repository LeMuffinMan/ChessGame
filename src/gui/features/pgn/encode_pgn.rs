use crate::Board;
use crate::ChessApp;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::moves::move_gen::generate_moves;
use crate::board::moves::move_structs::MoveList;
use crate::game::End::*;

impl ChessApp {
    pub fn encode_move_to_san(&mut self, from: &Coord, to: &Coord, prev_board: &Board) {
        //on ecrit le dernier coup une fois les checks du tour suivant faits
        if self.game.active_player == Black {
            self.history_san
                .push_str(self.game.turn.to_string().as_str());
            self.history_san.push_str(". ");
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
        } else {
            '?'
        };

        if piece == 'p' {
            self.pawn_san(from, to, prev_board);
        } else if piece == 'K' && (to.col as i8 - from.col as i8).abs() > 1 {
            if (to.col as i8 - from.col as i8) < 0 {
                self.history_san.push_str("O-O-O");
            } else {
                self.history_san.push_str("O-O");
            }
        } else {
            self.history_san.push(piece);
            if let Some(piece) = prev_board.get(from).get_piece() {
                match piece {
                    Pawn => {}
                    King => {}
                    _ => self.is_ambiguous_move(piece, from, to),
                };
            }

            if !prev_board.get(to).is_empty() {
                self.history_san.push('x');
            }
            self.history_san.push((b'a' + to.col) as char);
            self.history_san.push((b'0' + to.row + 1) as char);
        }

        //endgame and checks
        if let Some(end) = &self.game.end {
            match end {
                Resign | TimeOut => {
                    match self.game.opponent() {
                        White => self.history_san.push_str("0-1"),
                        Black => self.history_san.push_str("1-0"),
                    };
                }
                Checkmate => {
                    self.history_san.push_str("# ");
                    match self.game.active_player {
                        White => self.history_san.push_str("0-1"),
                        Black => self.history_san.push_str("1-0"),
                    };
                }
                Pat => self.history_san.push_str(" 1/2 - 1/2"),
                Draw => self.history_san.push_str(" 1/2 - 1/2"),
            };
        }
        if self.game.board.check.is_some() && self.game.end.is_none() {
            self.history_san.push('+');
        }
        self.history_san.push(' ');
    }

    fn is_en_passant_take(&mut self, from: &Coord, to: &Coord, prev_board: &Board) -> bool {
        let row_en_passant = if self.game.active_player == White {
            5
        } else {
            4
        };
        let diff: i8 = if self.game.active_player == White {
            -1
        } else {
            1
        };
        if from.row == row_en_passant {
            let new_row = from.row as i8 + diff;
            let coord = Coord {
                row: new_row as u8,
                col: to.col,
            };
            if self.game.board.get(&coord).get_piece() != prev_board.get(&coord).get_piece() {
                return true;
            }
        }
        false
    }

    fn pawn_san(&mut self, from: &Coord, to: &Coord, prev_board: &Board) {
        if !prev_board.get(to).is_empty() || self.is_en_passant_take(from, to, prev_board) {
            self.history_san.push((b'a' + from.col) as char);
            self.history_san.push('x');
        }
        self.history_san.push((b'a' + to.col) as char);
        self.history_san.push((b'0' + to.row + 1) as char);

        if let Some(piece) = self.game.board.get(to).get_piece()
            && *piece != Pawn
        {
            self.history_san.push('=');
            let p = match piece {
                Rook => 'R',
                Knight => 'N',
                Bishop => 'B',
                Queen => 'Q',
                _ => '_',
            };
            self.history_san.push(p);
        }
    }
    // ui.label("♔ ♕ ♖ ♗ ♘ ♙");
    // ui.label("♚ ♛ ♜ ♝ ♞ ♟")

    pub fn is_ambiguous_move(&mut self, piece: &Piece, origin: &Coord, dest: &Coord) {
        if !self.game.history.is_empty() && self.replay_infos.index > 0 {
            let mut prev_board = self.game.board_at(self.replay_infos.index - 1);
            let prev_player = if (self.replay_infos.index - 1).is_multiple_of(2) {
                White
            } else {
                Black
            };
            let mut move_list = MoveList::new();
            generate_moves(&mut prev_board, &prev_player, &mut move_list, false);
            let prev_legal_moves = move_list.moves[..move_list.count].to_vec();
            for m in prev_legal_moves.iter() {
                if &m.dest == dest
                    && let Some(p) = self.game.board.get(&m.origin).get_piece()
                    && p == piece
                {
                    if origin.col != m.origin.col {
                        self.history_san.push((b'a' + origin.col) as char);
                    } else if origin.row != m.origin.row {
                        self.history_san.push((b'0' + origin.row + 1) as char);
                    } else {
                        self.history_san.push((b'a' + origin.col) as char);
                        self.history_san.push((b'0' + origin.row + 1) as char);
                    }
                }
            }
        }
    }
}
