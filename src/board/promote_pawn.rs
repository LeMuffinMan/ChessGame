use crate::Board;
use crate::Color;
use crate::cell::Piece;
use crate::cell::Cell;

impl Board {
    pub fn promote_pawn(&mut self, color: &Color) {
        use std::io::{self, BufRead}; // en haut ou dans la fonction ?

        let stdin = io::stdin();
        let mut line = String::new();

        let promote_row = if *color == Color::White { 7 } else { 0 };
        for y in 0..8 {
            if self.grid[promote_row][y].is_color(color) {
                if let Some(piece) = self.grid[7][y].get_piece()
                    && *piece == Piece::Pawn
                {
                    println!("Pawn promote : R/N/B/Q");
                    line.clear();
                    loop {
                        if stdin.lock().read_line(&mut line).unwrap() == 0 {
                            println!("EOF");
                            break;
                        }
                        let input = line.trim();
                        if input.len() != 1 {
                            println!(
                                "Incorrect input.\nAllowed inputs : R = Rook | N = Knight | B = Bishop | Q = Queen"
                            );
                            continue;
                        }
                        self.grid[promote_row as usize][y as usize] = match input {
                            "R" => Cell::Occupied(Piece::Rook, *color),
                            "N" => Cell::Occupied(Piece::Knight, *color),
                            "B" => Cell::Occupied(Piece::Bishop, *color),
                            "Q" => Cell::Occupied(Piece::Queen, *color),
                            _ => {
                                println!(
                                    "Incorrect input.\nAllowed inputs : R = Rook | N = Knight | B = Bishop | Q = Queen"
                                );
                                continue;
                            }
                        };
                        let from_row = if *color == Color::White { 6 } else { 1 };
                        self.grid[from_row][y] = Cell::Free;
                        break;
                    }
                }
            }
        }
    }
}
