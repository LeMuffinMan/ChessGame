use crate::Board;
use crate::board::cell::Cell::*;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::is_king_exposed::is_king_exposed;
use crate::board::move_gen::Move;
use crate::board::move_gen::MoveList;
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
const MEDIUM_DEPTH: u8 = 2;
const HARD_DEPTH: u8 = 3;

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
    let mut move_list = MoveList::new();
    generate_moves(board, &active_player, &mut move_list);
    let moves = &mut move_list.moves[..move_list.count];
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
    let mut move_list = MoveList::new();
    generate_moves(board, &active_player, &mut move_list);
    let moves = &mut move_list.moves[..move_list.count];

    moves.sort_by_key(|m| {
        move_order_score(
            m,
            board.grid[m.origin.row as usize][m.origin.col as usize].get_piece(),
        )
    });

    let opponent = match active_player {
        White => Black,
        Black => White,
    };

    let mut best_move = None;
    let mut alpha = i32::MIN;

    for m in moves.iter() {
        board.apply_move(m, active_player);
        let score = -minimax(board, depth - 1, opponent, eval, -MATE_SCORE, MATE_SCORE);
        board.undo_move(*m, active_player);

        if score > alpha {
            alpha = score;
            best_move = Some(*m);
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
        Bot(Hard) => find_best_move(board, active_player, &PositionalEvaluator, HARD_DEPTH),
        Bot(Medium) => find_best_move(board, active_player, &PositionalEvaluator, MEDIUM_DEPTH),
        Bot(Easy) => {
            let mut move_list = MoveList::new();
            generate_moves(board, &active_player, &mut move_list);
            let moves = &mut move_list.moves[..move_list.count];
            let index = (Math::random() * moves.len() as f64).floor() as usize;
            Some(moves[index])
        }
        _ => None,
    }
}
