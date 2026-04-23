use crate::Board;
#[cfg(test)]
use crate::board::cell::Cell::Free;
use crate::board::cell::Cell::Occupied;
use crate::board::cell::Color;
use crate::board::cell::Coord;
use crate::board::cell::Piece;
use crate::board::cell::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};

pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 320;
pub const BISHOP_VALUE: i32 = 330;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;
pub const KING_VALUE: i32 = 20000;

pub trait Evaluator {
    fn evaluate(&self, board: &Board) -> i32;
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

pub fn get_piece_value_at(piece: &Piece, color: &Color, target: &Coord) -> i32 {
    let (val, pst) = match piece {
        Pawn => (PAWN_VALUE, PAWN_PST),
        Knight => (KNIGHT_VALUE, KNIGHT_PST),
        Bishop => (BISHOP_VALUE, BISHOP_PST),
        Rook => (ROOK_VALUE, ROOK_PST),
        Queen => (QUEEN_VALUE, QUEEN_PST),
        King => (KING_VALUE, KING_PST),
    };

    let pst_idx = if *color == Color::White {
        (7 - target.row) * 8 + target.col
    } else {
        target.row * 8 + target.col
    };
    let total = val + pst[pst_idx as usize];

    if *color == Color::White {
        total
    } else {
        -total
    }
}

pub struct PositionalEvaluator;

impl Evaluator for PositionalEvaluator {
    fn evaluate(&self, board: &Board) -> i32 {
        // let mut score = 0;
        // for x in 0..8 {
        //     for y in 0..8 {
        //         let target = Coord { row: x, col: y };
        //         if let Occupied(piece, color) = board.get(&target) {
        //             score += get_piece_value_at(&piece, &color, &target);
        //         }
        //     }
        // }
        // score

        return board.score;
        //     let mut score = 0;
        //     for row in 0..8 {
        //         for col in 0..8 {
        //             if let Occupied(piece, color) = board.grid[row][col] {
        //                 let side = if color == Color::White { 1 } else { -1 };

        //                 let pst_idx = if color == Color::White {
        //                     (7 - row) * 8 + col
        //                 } else {
        //                     row * 8 + col
        //                 };

        //                 let material = match piece {
        //                     Pawn => PAWN_VALUE,
        //                     Knight => KNIGHT_VALUE,
        //                     Bishop => BISHOP_VALUE,
        //                     Rook => ROOK_VALUE,
        //                     Queen => QUEEN_VALUE,
        //                     King => KING_VALUE,
        //                 };

        //                 let positional = match piece {
        //                     Pawn => PAWN_PST[pst_idx],
        //                     Knight => KNIGHT_PST[pst_idx],
        //                     Bishop => BISHOP_PST[pst_idx],
        //                     Rook => ROOK_PST[pst_idx],
        //                     Queen => QUEEN_PST[pst_idx],
        //                     King => KING_PST[pst_idx],
        //                 };
        //                 score += (material + positional) * side;
        //             }
        //         }
        //     }
        //     score
    }
}

// #[cfg(test)]
// pub struct MaterialEvaluator;

// #[cfg(test)]
// impl Evaluator for MaterialEvaluator {
//     fn evaluate(&self, board: &Board, active_player: Color) -> i32 {
//         let mut score = 0;
//         for x in 0..8 {
//             for y in 0..8 {
//                 let cell = board.grid[x][y].get_cell();
//                 match cell {
//                     Occupied(piece, color) => {
//                         let side = if *color == active_player { 1 } else { -1 };
//                         let mut value = 0;
//                         match piece {
//                             Pawn => value += PAWN_VALUE as i32,
//                             Knight => value += KNIGHT_VALUE as i32,
//                             Bishop => value += BISHOP_VALUE as i32,
//                             Rook => value += ROOK_VALUE as i32,
//                             Queen => value += QUEEN_VALUE as i32,
//                             King => value += KING_VALUE as i32,
//                         }
//                         score += value * side;
//                     }
//                     Free => {}
//                 };
//             }
//         }
//         score
//     }
// }
