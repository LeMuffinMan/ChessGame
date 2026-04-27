use crate::Board;
use crate::board::cell::Cell::Occupied;
use crate::board::cell::Color;
use crate::board::cell::Color::{Black, White};
use crate::board::cell::Coord;
use crate::board::cell::Piece;
use crate::board::cell::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};

pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 320;
pub const BISHOP_VALUE: i32 = 330;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;
pub const KING_VALUE: i32 = 20000;

// Matière brute d'une pièce non-pion non-roi (les deux camps, valeur positive).
pub fn non_pawn_raw(piece: &Piece) -> i32 {
    match piece {
        Knight => KNIGHT_VALUE,
        Bishop => BISHOP_VALUE,
        Rook => ROOK_VALUE,
        Queen => QUEEN_VALUE,
        _ => 0,
    }
}

// [0.0 = endgame pur, 1.0 = opening/middlegame plein]
fn game_phase(board: &Board) -> f32 {
    const MAX_NPM: i32 = 2 * (QUEEN_VALUE + 2 * ROOK_VALUE + 2 * BISHOP_VALUE + 2 * KNIGHT_VALUE);
    (board.non_pawn_material as f32 / MAX_NPM as f32).clamp(0.0, 1.0)
}

// Bonus de sécurité du roi d'une couleur (positif = bon, négatif = exposé).
fn king_safety(board: &Board, color: Color) -> i32 {
    let king = if color == White { board.white_king } else { board.black_king };
    let shield_dir: i8 = if color == White { 1 } else { -1 };
    let mut score = 0i32;

    // Pawn shield : 3 cases devant le roi
    for dc in [-1i8, 0, 1] {
        let r = king.row as i8 + shield_dir;
        let c = king.col as i8 + dc;
        if (0..8).contains(&r) && (0..8).contains(&c)
            && board.grid[r as usize][c as usize] == Occupied(Pawn, color)
        {
            score += 10;
        }
    }

    // Attacker proximity : pièces adverses dans l'anneau 3×3
    for dr in -1i8..=1 {
        for dc in -1i8..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let r = king.row as i8 + dr;
            let c = king.col as i8 + dc;
            if (0..8).contains(&r) && (0..8).contains(&c)
                && board.grid[r as usize][c as usize].is_opponent_color(&color)
            {
                score -= 20;
            }
        }
    }
    score
}

// Ajustement du PST roi : interpolation MG → EG selon la phase.
// board.score inclut déjà KING_PST (MG) ; on ajoute le delta vers EG.
fn king_pst_delta(board: &Board, phase: f32) -> i32 {
    let eg_weight = 1.0 - phase;
    if eg_weight < 0.01 {
        return 0;
    }
    let w_idx = ((7 - board.white_king.row) * 8 + board.white_king.col) as usize;
    let b_idx = (board.black_king.row * 8 + board.black_king.col) as usize;
    let w_delta = KING_PST_EG[w_idx] - KING_PST[w_idx];
    let b_delta = KING_PST_EG[b_idx] - KING_PST[b_idx];
    ((w_delta - b_delta) as f32 * eg_weight) as i32
}

pub fn evaluate(board: &Board) -> i32 {
    let phase = game_phase(board);
    let safety = king_safety(board, White) - king_safety(board, Black);
    // 60% max en plein middlegame, 0% en endgame
    let safety_weight = (phase * 60.0) as i32;
    board.score + safety * safety_weight / 100 + king_pst_delta(board, phase)
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

// En endgame le roi doit se centraliser et aider aux pions
#[rustfmt::skip]
const KING_PST_EG: [i32; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -50,-40,-30,-20,-20,-30,-40,-50,
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

