use crate::Board;
use crate::board::cell::Cell::Occupied;
use crate::board::cell::Color;
use crate::board::cell::Color::{Black, White};
use crate::board::cell::Coord;
use crate::board::cell::Piece;
use crate::board::cell::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::engine::pst_maps::*;

pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 320;
pub const BISHOP_VALUE: i32 = 330;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;
pub const KING_VALUE: i32 = 20000;

pub fn evaluate(board: &Board) -> i32 {
    let phase = game_phase(board);
    let safety = king_safety(board, White) - king_safety(board, Black);
    let safety_weight = (phase * 60.0) as i32;
    board.score + safety * safety_weight / 100 + king_pst_delta(board, phase) + mop_up_eval(board)
}

pub fn non_pawn_raw(piece: &Piece) -> i32 {
    match piece {
        Knight => KNIGHT_VALUE,
        Bishop => BISHOP_VALUE,
        Rook => ROOK_VALUE,
        Queen => QUEEN_VALUE,
        _ => 0,
    }
}

// 0.0 is an endamge / 1.0 is an opening
fn game_phase(board: &Board) -> f32 {
    const MAX_NPM: i32 = 2 * (QUEEN_VALUE + 2 * ROOK_VALUE + 2 * BISHOP_VALUE + 2 * KNIGHT_VALUE);
    (board.non_pawn_material as f32 / MAX_NPM as f32).clamp(0.0, 1.0)
}

fn king_safety(board: &Board, color: Color) -> i32 {
    let king = if color == White {
        board.white_king
    } else {
        board.black_king
    };
    let shield_dir: i8 = if color == White { 1 } else { -1 };
    let mut score = 0;

    //seeking 3 protective pawns
    for dc in [-1, 0, 1] {
        let r = king.row as i8 + shield_dir;
        let c = king.col as i8 + dc;
        if (0..8).contains(&r)
            && (0..8).contains(&c)
            && board[(r as usize, c as usize)] == Occupied(Pawn, color)
        {
            score += 10;
        }
    }

    //seeking attacker near
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let r = king.row as i8 + dr;
            let c = king.col as i8 + dc;
            if (0..8).contains(&r)
                && (0..8).contains(&c)
                && board[(r as usize, c as usize)].is_opponent_color(&color)
            {
                score -= 20;
            }
        }
    }
    score
}

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

// Mop-up eval : active when weak side has only a king and strong side has at least one R or Q.
// reward if the weak king is in a corner, and if the two kings are close to each other
fn mop_up_eval(board: &Board) -> i32 {
    let (white_npm, black_npm) = per_side_npm(board);

    let (sign, weak_king, strong_king, strong_color) =
        if black_npm == 0 && has_rook_or_queen(board, White) {
            (1, board.black_king, board.white_king, White)
        } else if white_npm == 0 && has_rook_or_queen(board, Black) {
            (-1, board.white_king, board.black_king, Black)
        } else {
            return 0;
        };

    let cmd = king_corner_pressure(weak_king);
    let kd = king_manhattan_dist(strong_king, weak_king);
    let cut = rook_cut_bonus(board, strong_color, weak_king);
    let bcut = bishop_cut_bonus(board, strong_color, weak_king);
    let freedom = king_freedom(weak_king, board, strong_color); // disabled for comparison
    sign * (80 * cmd + 30 * (14 - kd) + cut + bcut - 10 * freedom)
}

fn per_side_npm(board: &Board) -> (i32, i32) {
    let mut white = 0;
    let mut black = 0;
    for row in &board.grid {
        for cell in row {
            if let Occupied(piece, color) = cell {
                let npm = non_pawn_raw(piece);
                if npm > 0 {
                    if *color == White {
                        white += npm;
                    } else {
                        black += npm;
                    }
                }
            }
        }
    }
    (white, black)
}

// -- ladder mate --

// +25 per rook sharing a line with the weak king
fn rook_cut_bonus(board: &Board, strong_color: Color, weak_king: Coord) -> i32 {
    let mut bonus = 0;
    for r in 0..8 {
        for c in 0..8 {
            if let Occupied(piece, color) = board[(r, c)]
                && color == strong_color
                && (piece == Rook || piece == Queen)
            {
                let r_dist = (r - weak_king.row as i32).abs();
                let c_dist = (c - weak_king.col as i32).abs();

                if r_dist == 0 || c_dist == 0 {
                    bonus += if piece == Rook { 40 } else { 20 };
                }

                if r_dist == 1 || c_dist == 1 {
                    bonus += 25;
                }
            }
        }
    }
    bonus
}

// +35 per bishop sharing a diag with the weak king
fn bishop_cut_bonus(board: &Board, strong_color: Color, weak_king: Coord) -> i32 {
    let mut bonus = 0;
    let k_diag = weak_king.row as i8 - weak_king.col as i8;
    let k_anti = weak_king.row as i8 + weak_king.col as i8;
    for r in 0..8usize {
        for c in 0..8 {
            if let Occupied(Bishop, color) = board[(r, c)]
                && color == strong_color
            {
                let b_diag = r as i8 - c as i8;
                let b_anti = r as i8 + c as i8;
                if b_diag == k_diag || b_anti == k_anti {
                    bonus += 35;
                }
            }
        }
    }
    bonus
}

fn has_rook_or_queen(board: &Board, color: Color) -> bool {
    board
        .grid
        .iter()
        .flatten()
        .any(|cell| matches!(cell, Occupied(Rook, c) | Occupied(Queen, c) if *c == color))
}

// 0 = center  6 = corner
fn king_corner_pressure(king: Coord) -> i32 {
    let r = king.row as i32;
    let c = king.col as i32;
    (3 - r).max(r - 4).max(0) + (3 - c).max(c - 4).max(0)
}

// the less  mobility the weak king has, the better the score is
fn king_freedom(weak_king: Coord, board: &Board, strong_color: Color) -> i32 {
    let mut count = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let r = weak_king.row as i8 + dr;
            let c = weak_king.col as i8 + dc;
            if (0..8).contains(&r)
                && (0..8).contains(&c)
                && !matches!(board[(r as usize, c as usize)], Occupied(_, color) if color == strong_color)
            {
                count += 1;
            }
        }
    }
    count
}

fn king_manhattan_dist(origin: Coord, dest: Coord) -> i32 {
    (origin.row as i32 - dest.row as i32).abs() + (origin.col as i32 - dest.col as i32).abs()
}

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
