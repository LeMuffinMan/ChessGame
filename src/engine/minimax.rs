use crate::Board;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::move_gen::Move;
use crate::board::move_gen::generate_moves;
use crate::engine::evaluator::{Evaluator, MaterialEvaluation};

const MATE_SCORE: i32 = 1_000_000;

pub(crate) fn minimax<E: Evaluator>(
    board: &mut Board,
    depth: u8,
    active_player: Color,
    eval: &E,
    mut alpha: i32,
    beta: i32,
) -> i32 {
    if depth == 0 {
        return eval.evaluate(board, active_player);
    }

    let moves = generate_moves(board, &active_player);

    if moves.is_empty() {
        return if board.check.is_some() {
            // mated: worse at lower depth (prefer fast mates)
            -MATE_SCORE + depth as i32
        } else {
            0
        };
    }

    let opponent = match active_player {
        White => Color::Black,
        Black => Color::White,
    };

    for m in moves.iter() {
        board.apply_move(m, active_player);
        let score = -minimax(board, depth - 1, opponent, eval, -beta, -alpha);
        board.undo_move(*m, active_player);
        if score > alpha {
            alpha = score;
        }
        if alpha >= beta {
            return alpha;
        }
    }

    alpha
}

pub fn find_best_move(board: &mut Board, active_player: Color, depth: u8) -> Option<Move> {
    let moves = generate_moves(board, &active_player);
    let opponent = match active_player {
        White => Black,
        Black => White,
    };
    let mut best_move = None;
    let mut best_score = i32::MIN;
    for m in moves {
        board.apply_move(&m, active_player);
        let score = -minimax(board, depth - 1, opponent, &MaterialEvaluation, -MATE_SCORE, MATE_SCORE);
        board.undo_move(m, active_player);
        if score > best_score {
            best_score = score;
            best_move = Some(m);
        }
    }
    best_move
}
