// use chrono::Utc;
// use std::fs;
// use std::path::Path;

use crate::Board;
use crate::ChessApp;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::End::*;

// pub fn export_pgn(san: &str, path: &Path) {
//     let mut pgn = String::new();
//     pgn.push_str("[Event \"ChessGame\"]\n[Site \"ChessGame\"]\n[Date \"");
//     pgn.push_str(Utc::now().to_string().as_str());
//     pgn.push_str("\"]\n[White \"White_player\"]\n[Black \"Black_player\"]\n");
//     if let Some(result) = san.split_whitespace().last() {
//         pgn.push_str("[Result : \"");
//         if result == "0-1" || result == "1-0" || result == "1/2 - 1/2" {
//             pgn.push_str(result);
//         } else {
//             pgn.push('*');
//         }
//         pgn.push_str("\"]\n\n");
//         pgn.push_str(san);
//         pgn.push('\n');
//         match fs::write(path, &pgn) {
//             Ok(_) => println!("File saved with success"),
//             Err(e) => eprintln!("Error saving file : {}", e),
//         }
//         println!("{}", pgn);
//     }
// }

impl ChessApp {
    //a reacto
    pub fn encode_move_to_san(&mut self, from: &Coord, to: &Coord, prev_board: &Board) {
        //on ecrit le dernier coup une fois les checks du tour suivant faits
        if self.current.active_player == Black {
            self.history_san
                .push_str(self.current.turn.to_string().as_str());
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
        //
        if let Some(end) = &self.current.end {
            match end {
                Resign | TimeOut => {
                    match self.current.opponent {
                        White => self.history_san.push_str("0-1"),
                        Black => self.history_san.push_str("1-0"),
                    };
                }
                Checkmate => {
                    self.history_san.push_str("# ");
                    match self.current.active_player {
                        White => self.history_san.push_str("0-1"),
                        Black => self.history_san.push_str("1-0"),
                    };
                }
                Pat => self.history_san.push_str(" 1/2 - 1/2"),
                Draw => self.history_san.push_str(" 1/2 - 1/2"),
            };
        }
        if self.current.board.check.is_some() && self.current.end.is_none() {
            self.history_san.push('+');
        }
        self.history_san.push(' ');
    }

    fn is_en_passant_take(&mut self, from: &Coord, to: &Coord, prev_board: &Board) -> bool {
        let row_en_passant = if self.current.active_player == White {
            5
        } else {
            4
        };
        let diff: i8 = if self.current.active_player == White {
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
            if self.current.board.get(&coord).get_piece() != prev_board.get(&coord).get_piece() {
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

        if let Some(piece) = self.current.board.get(to).get_piece()
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

    pub fn is_ambiguous_move(&mut self, piece: &Piece, from: &Coord, to: &Coord) {
        if !self.history.is_empty() && self.widgets.replay_index > 0 {
            let prev_state = &self.history[self.widgets.replay_index - 1];
            let prev_legal_moves = prev_state.board.legals_moves.clone();
            for (f, t) in prev_legal_moves.iter() {
                if t == to
                    && let Some(p) = self.current.board.get(f).get_piece()
                    && p == piece
                {
                    if from.col != f.col {
                        self.history_san.push((b'a' + from.col) as char);
                    } else if from.row != f.row {
                        self.history_san.push((b'0' + from.row + 1) as char);
                    } else {
                        self.history_san.push((b'a' + from.col) as char);
                        self.history_san.push((b'0' + from.row + 1) as char);
                    }
                }
            }
        }
    }

    // fn load_pgn_datas() -> (Vec<String>, Vec<(Coord, Coord)>) {
    // let mut headers = Vec::new();
    // let mut move_list = Vec::new();
    //extraire le contenu d'un fichier recu
    //
    //
    //headers = get_headers(lines);
    //move_list = get_moves_list(line);
    // todo!();
    // (headers, move_list)

    // }

    // fn get_san_moves_list(san: String) -> Vec<String> {
    // let moves_list = Vec::new();

    //recuperer la ligne du san
    //accepter :
    //      abcdefghRNBQKO12345678.[]/-
    //split sur les espaces
    //evacuer les 1. si isdigit+.
    // todo!();
    // moves_list
    // }

    // fn load_san(move_list: Vec<String>) -> ChessApp {
    //for move in move_list
    //{
    //   if (from, to) = interpret_san {
    //      gerrer l'erreur
    //   } else {
    //      try_move(from, to);
    //      //gerer l'erreur
    //   }
    //   //success -> ChessApp
    //}
    // todo!();
    // }

    // fn interpret_san(move: String) -> Option<(Option<Coord>, Coord)> {
    // let piece: char = if move[0].is_lowercase {
    //     pawn
    // } else if move[0].is_uppercase() {
    //      match move[0] {
    //         R
    //         N
    //         B
    //         Q
    //         K
    //         O
    //         [
    //         1
    //         0
    //         _ { invalid }
    //      }
    // }
    //
    // let to = match piece {
    //      Pawn => {
    //          if is_capture {
    //              to = x +1 +2
    //              chercher indice origin
    //          }
    //      }
    //      Rook | Knight | Bishop | Queen | King => {
    //          if is_capture {
    //              to = x +1 +2
    //              chercher indice origin
    //          }
    //      }
    //      _ { edges cases or invalid }
    //
    // let from = if Some(Coord) = find_piece_origin(hint) { Coord } else { None }
    // Some((from, to))
    // }
    // }

    //as IMPL CHESSAPP
    // fn update_last_move_san(&mut self) {
    //     if self.current.checkmate {
    //         self.current.last_move_san.push_str("# ");
    //     } else if self.current.board.check.is_some() {
    //         self.current.last_move_san.push_str("+ ");
    //         if self.current.active_player == Color::White {
    //             self.current.last_move_san.push_str("1-0");
    //         } else {
    //             self.current.last_move_san.push_str("0-1");
    //             //     } else if self.current.pat {
    //         self.current.last_move_san.push_str("1/2 - 1/2");
    //     } else {
    //         self.current.last_move_san.push_str(" ");
    //     }
    // }
    //

    // fn pieces_move(san_move: &String, moves_list: Vec<(Piece, Coord, Option<char>, Option<char>)) {
    //     let piece = match san_move[0] {
    //         'R' => Rook,
    //         'N' => Knight,
    //         'B' => Bishop,
    //         'Q' => Queen,
    //         'K' => King,
    //     };
    //     let mut hint_col = None;
    //     let mut hint_row = None;
    //     if san_move.len() == 3 {
    //         let coord = Coord { row: san_move[2] - b'0' - 1, col: san_move[1] - b'a' };
    //     } else if san_move.len() >= 4 {
    //         // a finir
    //     } else { printlnt!("Inccorrect san code {}" san_move);
    //
    //     }
    // }
    //
    // fn pawn_move(san_move: &String, moves_list: Vec<(Piece, Coord, Option<char>, Option<char>)>) {
    //     //manque le hint
    //     if san_move.len() == 2 {
    //         let coord = Coord { row: san_move[1] - b'0' - 1, col: san_move[0] - b'a' };
    //     } else {
    //        let coord = Coord { row: san_move[3] - b'0' - 1, col: san_move[2] - b'a' };
    //     }
    //     moves_list.push((Pawn, Coord, None));
    // }
    //
    // fn decode_san(san: &String) -> Vec<(Piece, Coord, Option<char>, Option<char>)> {
    //     let mut moves_list = Vec::new();
    //     for san_move in san.split_whitespaces().iter() {
    //         if !san_move.contains('.') {
    //             if san_move[0].is_lowercase() {
    //                 pawn_move(san_move, moves_list);
    //             } else if san_move[0].is_uppercase() {
    //                 pieces_move(san_move, moves_list);
    //             } else if san_move[0] == 'O' {
    //
    //             } else if san_move == "1-0" || san_move == "0-1" || san_move == "1/2 - 1/2" {
    //                 break;
    //             } else { println!("Incorrect first char in move {}", san_move); }
    //         }
    //     }
    //     return moves_list;
    // }
}
