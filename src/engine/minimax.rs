use crate::Board;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::move_gen::generate_moves;
use crate::engine::evaluator::Evaluator;
use std::i32;

const MATE_SCORE: i32 = 1_000_000;
const DEPTH_MAX: u8 = 5;

fn minimax<E: Evaluator>(
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
            board.apply_move(m, opponnent);
            score = minimax(board, depth - 1, true, active_player, eval);
            board.undo_move(*m, opponnent);
            if score < best {
                best = score;
            }
        }
    }
    return best;
}
