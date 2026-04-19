use std::i32;

use crate::Board;
use crate::board::cell::Cell::{Free, Occupied};
use crate::board::cell::Color;
use crate::board::cell::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 20000;

pub trait Evaluator {
    fn evaluate(&self, board: &Board, active_player: Color) -> i32;
}

pub struct MaterialEvaluation;

impl Evaluator for MaterialEvaluation {
    fn evaluate(&self, board: &Board, active_player: &Color) -> i32 {
        let mut score = 0;
        for x in 0..8 {
            for y in 0..8 {
                let cell = board.grid[x][y].get_cell();
                match cell {
                    Occupied(piece, color) => {
                        let side = if color == active_player { 1 } else { -1 };
                        let mut value = 0;
                        match piece {
                            Pawn => value += PAWN_VALUE,
                            Knight => value += KNIGHT_VALUE,
                            Bishop => value += BISHOP_VALUE,
                            Rook => value += ROOK_VALUE,
                            Queen => value += QUEEN_VALUE,
                            King => value += KING_VALUE,
                        }
                        score += value * side;
                    }
                    Free => { /* we skip empty cell  */ }
                };
            }
        }
        score
    }
}
