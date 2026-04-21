use crate::Board;
use crate::board::cell::Cell::*;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::is_king_exposed::is_king_exposed;
use crate::board::move_gen::Move;
use crate::board::move_gen::generate_moves;
use crate::engine::evaluator::Evaluator;
use crate::engine::evaluator::PositionalEvaluator;
use crate::engine::evaluator::{
    BISHOP_VALUE, KING_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE,
};
use crate::gui::bot_difficulty::BotDifficulty::*;
use crate::gui::player_type::PlayerType;
use crate::gui::player_type::PlayerType::*;
use js_sys::Math;

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

    let mut moves = generate_moves(board, &active_player);

    if moves.is_empty() {
        return if is_king_exposed(board, &active_player) {
            -MATE_SCORE + depth as i32
        } else {
            0
        };
    }

    moves.sort_by_key(|m| {
        move_order_score(
            &m,
            board.grid[m.origin.row as usize][m.origin.col as usize].get_piece(),
        )
    });

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

pub fn find_best_move<E: Evaluator>(
    board: &mut Board,
    active_player: Color,
    eval: &E,
    depth: u8,
) -> Option<Move> {
    let mut moves = generate_moves(board, &active_player);
    moves.sort_by_key(|m| {
        move_order_score(
            &m,
            board.grid[m.origin.row as usize][m.origin.col as usize].get_piece(),
        )
    });
    let opponent = match active_player {
        White => Black,
        Black => White,
    };
    let mut best_move = None;
    let mut best_score = i32::MIN;
    for m in moves {
        board.apply_move(&m, active_player);
        let score = -minimax(board, depth - 1, opponent, eval, -MATE_SCORE, MATE_SCORE);
        board.undo_move(m, active_player);
        if score > best_score {
            best_score = score;
            best_move = Some(m);
        }
    }
    best_move
}

pub fn move_order_score(mv: &Move, attacker: Option<&Piece>) -> i32 {
    let score;
    let attack_score = match attacker {
        Some(Pawn) => PAWN_VALUE / 10,
        Some(Knight) => KNIGHT_VALUE / 10,
        Some(Bishop) => BISHOP_VALUE / 10,
        Some(Rook) => ROOK_VALUE / 10,
        Some(Queen) => QUEEN_VALUE / 10,
        Some(King) => KING_VALUE / 10,
        None => unreachable!(),
    };
    match mv.capture {
        Occupied(piece, _) => match piece {
            Pawn => score = PAWN_VALUE - attack_score,
            Knight => score = KNIGHT_VALUE - attack_score,
            Bishop => score = BISHOP_VALUE - attack_score,
            Rook => score = ROOK_VALUE - attack_score,
            Queen => score = QUEEN_VALUE - attack_score,
            King => score = KING_VALUE - attack_score,
        },
        Free => score = 0,
    };
    -score
}

pub fn get_bot_move(
    difficulty: &PlayerType,
    board: &mut Board,
    active_player: Color,
) -> Option<Move> {
    match difficulty {
        Bot(Hard) => find_best_move(board, active_player, &PositionalEvaluator, 3),
        Bot(Medium) => find_best_move(board, active_player, &PositionalEvaluator, 2),
        Bot(Easy) => {
            let moves = generate_moves(board, &active_player);
            let index = (Math::random() * moves.len() as f64).floor() as usize;
            Some(moves[index])
        }
        _ => None,
    }
}
