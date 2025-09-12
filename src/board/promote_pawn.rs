use crate::Board;
use crate::Color;
use crate::Coord;
use crate::board::cell::Piece;
// use std::io::{self, BufRead}; // en haut ou dans la fonction ?

impl Board {
    pub fn promote_pawn(&mut self, color: &Color) {
        // let stdin = io::stdin();
        // let mut line = String::new();

        let promote_row = if *color == Color::White { 7 } else { 0 };
        for y in 0..8 {
            if self.grid[promote_row][y].is_color(color) {
                if let Some(piece) = self.grid[promote_row][y].get_piece()
                    && *piece == Piece::Pawn
                {
                    let coord = Coord {
                        row: promote_row as u8,
                        col: y as u8,
                    };
                    self.pawn_to_promote = Some(coord);
                    // println!("Pawn promote : R/N/B/Q");
                    // line.clear();
                    // loop {
                    //     if stdin.lock().read_line(&mut line).unwrap() == 0 {
                    //         println!("EOF");
                    //         break;
                    //     }
                    //     let input = line.trim();
                    //     if input.len() != 1 {
                    //         println!(
                    //             "Incorrect input.\nAllowed inputs : R = Rook | N = Knight | B = Bishop | Q = Queen"
                    //         );
                    //         continue;
                    //     }
                    //     self.grid[promote_row][y] = match input {
                    //         "R" => Cell::Occupied(Piece::Rook, *color),
                    //         "N" => Cell::Occupied(Piece::Knight, *color),
                    //         "B" => Cell::Occupied(Piece::Bishop, *color),
                    //         "Q" => Cell::Occupied(Piece::Queen, *color),
                    //         _ => {
                    //             println!(
                    //                 "Incorrect input.\nAllowed inputs : R = Rook | N = Knight | B = Bishop | Q = Queen"
                    //             );
                    //             continue;
                    //         }
                    //     };
                    //     let from_row = if *color == Color::White { 6 } else { 1 };
                    //     self.grid[from_row][y] = Cell::Free;
                    //     break;
                    // }
                }
            }
        }
    }
}
