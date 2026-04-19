use crate::Board;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::move_gen::Move;
use crate::board::move_gen::generate_moves;
use crate::engine::evaluator::{Evaluator, MaterialEvaluation};
use std::i32;

const MATE_SCORE: i32 = 1_000_000;

pub(crate) fn minimax<E: Evaluator>(
    board: &mut Board,
    depth: u8,
    maximising: bool,
    active_player: Color,
    eval: &E,
) -> i32 {
    if depth == 0 {
        return eval.evaluate(board, active_player);
    }
    //en faire une impl de board
    let moves = generate_moves(board, &active_player);

    if moves.is_empty() {
        if board.check.is_some() {
            return if maximising {
                -MATE_SCORE - depth as i32
            } else {
                MATE_SCORE - depth as i32
            };
        } else {
            return 0;
        }
    };

    let opponnent = match active_player {
        White => Color::Black,
        Black => Color::White,
    };
    let mut score;
    let mut best = if maximising { -MATE_SCORE } else { MATE_SCORE };
    if maximising {
        for m in moves.iter() {
            board.apply_move(m, active_player);
            score = minimax(board, depth - 1, false, opponnent, eval);
            board.undo_move(*m, active_player);
            if score > best {
                best = score;
            }
        }
    } else {
        for m in moves.iter() {
            board.apply_move(m, active_player);
            score = minimax(board, depth - 1, true, opponnent, eval);
            board.undo_move(*m, active_player);
            if score < best {
                best = score;
            }
        }
    }
    return best;
}

pub fn find_best_move(board: &mut Board, active_player: Color, depth: u8) -> Option<Move> {
    let moves = generate_moves(board, &active_player);
    let opponent = match active_player { White => Black, Black => White };
    let mut best_move = None;
    let mut best_score = i32::MIN;
    for m in moves {
        board.apply_move(&m, active_player);
        let score = minimax(board, depth - 1, false, opponent, &MaterialEvaluation);
        board.undo_move(m, active_player);
        if score > best_score {
            best_score = score;
            best_move = Some(m);
        }
    }
    best_move
}
