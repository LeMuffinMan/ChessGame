use crate::Board;
use crate::Color;
use crate::Coord;
use crate::board::cell::Piece::*;
use crate::board::legals_moves::piece_case::update_bishop_legals_moves;
use crate::board::legals_moves::piece_case::update_king_legals_moves;
use crate::board::legals_moves::piece_case::update_knight_legals_moves;
use crate::board::legals_moves::piece_case::update_pawn_legals_moves;
use crate::board::legals_moves::piece_case::update_queen_legals_moves;
use crate::board::legals_moves::piece_case::update_rook_legals_moves;
use crate::validate_move::is_legal_move::is_king_exposed;

impl Board {
    pub fn update_legals_moves(&mut self, color: &Color) {
        self.legals_moves.clear();
        for x in 0..8 {
            for y in 0..8 {
                if self.grid[x][y].is_color(color) {
                    let from = Coord {
                        row: x as u8,
                        col: y as u8,
                    };
                    if let Some(piece) = self.get(&from).get_piece() {
                        match piece {
                            Pawn => {
                                let vec = update_pawn_legals_moves(&from, color, self);
                                self.legals_moves.extend(vec);
                            }
                            Rook => {
                                let vec = update_rook_legals_moves(&from, color, self);
                                self.legals_moves.extend(vec);
                            }
                            Knight => {
                                let vec = update_knight_legals_moves(&from, color, self);
                                self.legals_moves.extend(vec);
                            }
                            Bishop => {
                                let vec = update_bishop_legals_moves(&from, color, self);
                                self.legals_moves.extend(vec);
                            }
                            Queen => {
                                let vec = update_queen_legals_moves(&from, color, self);
                                self.legals_moves.extend(vec);
                            }
                            King => {
                                let vec = update_king_legals_moves(&from, color, self);
                                self.legals_moves.extend(vec);
                            }
                        }
                    }
                }
            }
        }
        // println!("Legals moves : ");
        // for (from, to) in &self.legals_moves {
        //     println!(
        //         "from: ({}, {}), to: ({}, {})",
        //         from.row, from.col, to.row, to.col
        //     );
        // }
    }

    pub fn test_and_push(
        &mut self,
        from: &Coord,
        to: &Coord,
        color: &Color,
    ) -> Option<(Coord, Coord)> {
        if self.is_legal_move(from, to, color) {
            if !is_king_exposed(from, to, color, self) {
                // println!("pushing from: ({}, {}), to: ({}, {})", from.row, from.col, to.row, to.col);
                return Some((*from, *to));
                // self.legals_moves.push((*from, *to));
            }
            // println!("king exposed: from: ({}, {}), to: ({}, {})", from.row, from.col, to.row, to.col);
        }
        // println!("illegal move: from: ({}, {}), to: ({}, {})", from.row, from.col, to.row, to.col);
        return None;
    }

    pub fn checked_coord(row: i8, col: i8) -> Option<Coord> {
        if (0..8).contains(&row) && (0..8).contains(&col) {
            Some(Coord {
                row: row as u8,
                col: col as u8,
            })
        } else {
            None
        }
    }
}
