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

// PST : rank 8 first (index 0) / rank 1 last (index 56).
// White → (7-row)*8+col  |  Black → row*8+col
#[rustfmt::skip]
const PAWN_PST: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0,
];
#[rustfmt::skip]
const KNIGHT_PST: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];
#[rustfmt::skip]
const BISHOP_PST: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];
#[rustfmt::skip]
const ROOK_PST: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0,
];
#[rustfmt::skip]
const QUEEN_PST: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];
#[rustfmt::skip]
const KING_PST: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20,
];

pub struct PositionalEvaluator;

impl Evaluator for PositionalEvaluator {
    fn evaluate(&self, board: &Board, active_player: Color) -> i32 {
        let mut score = 0;
        for row in 0..8 {
            for col in 0..8 {
                if let Occupied(piece, color) = board.grid[row][col] {
                    let side = if color == active_player { 1 } else { -1 };
                    let pst_idx = if color == Color::White {
                        (7 - row) * 8 + col
                    } else {
                        row * 8 + col
                    };
                    let material = match piece {
                        Pawn => PAWN_VALUE,
                        Knight => KNIGHT_VALUE,
                        Bishop => BISHOP_VALUE,
                        Rook => ROOK_VALUE,
                        Queen => QUEEN_VALUE,
                        King => KING_VALUE,
                    };
                    let positional = match piece {
                        Pawn => PAWN_PST[pst_idx],
                        Knight => KNIGHT_PST[pst_idx],
                        Bishop => BISHOP_PST[pst_idx],
                        Rook => ROOK_PST[pst_idx],
                        Queen => QUEEN_PST[pst_idx],
                        King => KING_PST[pst_idx],
                    };
                    score += (material + positional) * side;
                }
            }
        }
        score
    }
}

pub struct MaterialEvaluator;

impl Evaluator for MaterialEvaluator {
    fn evaluate(&self, board: &Board, active_player: Color) -> i32 {
        let mut score = 0;
        for x in 0..8 {
            for y in 0..8 {
                let cell = board.grid[x][y].get_cell();
                match cell {
                    Occupied(piece, color) => {
                        let side = if *color == active_player { 1 } else { -1 };
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
