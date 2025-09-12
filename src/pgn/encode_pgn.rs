
use chrono::Utc;
use std::path::Path;
use std::fs;

pub fn export_pgn(san: &String, path: &Path) {
    let mut pgn = String::new();
    pgn.push_str("[Event \"ChessGame\"]\n[Site \"ChessGame\"]\n[Date \"");
    pgn.push_str(Utc::now().to_string().as_str());
    pgn.push_str("\"]\n[White \"White_player\"]\n[Black \"Black_player\"]\n");
    if let Some(result) = san.split_whitespace().last() {
        pgn.push_str("[Result : \"");
        if result == "0-1" || result == "1-0" || result == "1/2 - 1/2" {
            pgn.push_str(result);
        } else {
            pgn.push('*');
        }
        pgn.push_str("\"]\n\n");
        pgn.push_str(san);
        pgn.push('\n');
        match fs::write(path, &pgn) {
            Ok(_) => println!("File saved with success"),
            Err(e) => eprintln!("Error saving file : {}", e),
        }
        println!("{}", pgn);
    }
}
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

