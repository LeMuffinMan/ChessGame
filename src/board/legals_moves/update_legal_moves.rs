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
use crate::board::validate_move::is_king_exposed;

impl Board {
    //For each cell, we test each active player color piece possible moves
    //each piece case fct return a vec of the piece tested possible moves
    //We push this vec in the legals moves vec
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
    }

    //we want to know if the move is piece-legal
    //and if it does not leave the active player king threaten
    pub fn test_and_push(
        &mut self,
        from: &Coord,
        to: &Coord,
        color: &Color,
    ) -> Option<(Coord, Coord)> {
        if self.is_legal_move(from, to, color) && !is_king_exposed(from, to, color, self) {
            return Some((*from, *to));
        }
        None
    }

    //util fct to prevent over/underflow
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
