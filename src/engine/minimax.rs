use crate::Board;
use crate::board::cell::Cell::*;
use crate::board::cell::Color;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::move_gen::Move;
use crate::board::move_gen::MoveList;
use crate::board::move_gen::MoveType::Promotion;
use crate::board::move_gen::generate_moves;
use crate::engine::evaluator::Evaluator;
use crate::engine::evaluator::PositionalEvaluator;
// use crate::engine::evaluator::*;
use crate::engine::search_stats::SearchStats;
use crate::gui::features::bot::BotDifficulty::*;
use crate::gui::features::bot::PlayerType;
use crate::gui::features::bot::PlayerType::*;
use js_sys::Math;

const MATE_SCORE: i32 = 1_000_000;
pub const MEDIUM_DEPTH: u8 = 3;
pub const HARD_DEPTH: u8 = 4;

pub fn minimax<E: Evaluator>(
    board: &mut Board,
    depth: u8,
    active_player: Color,
    eval: &E,
    mut alpha: i32,
    mut beta: i32,
    stats: &mut SearchStats,
) -> i32 {
    stats.nodes += 1;

    if depth == 0 {
        // return quiescence(board, alpha, beta, eval, active_player, stats);
        return board.score;
    }

    let mut move_list = MoveList::new();
    generate_moves(board, &active_player, &mut move_list, false);
    let moves = &mut move_list.moves[..move_list.count];

    if moves.is_empty() {
        return if board.check.is_some() {
            is_mate_or_pat(active_player, depth)
        } else {
            0
        };
    }

    let opponent = match active_player {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    if active_player == Color::White {
        let mut max_eval = i32::MIN;
        for i in 0..moves.len() {
            let mut best_idx = i;
            let mut best_score = i32::MIN;
            for j in i..moves.len() {
                let score = move_order_score(
                    &moves[j],
                    board.grid[moves[j].origin.row as usize][moves[j].origin.col as usize]
                        .get_piece(),
                    stats.killer_moves[depth as usize]
                        .get(0)
                        .and_then(|x| x.as_ref()),
                    stats.killer_moves[depth as usize]
                        .get(1)
                        .and_then(|x| x.as_ref()),
                );
                if score > best_score {
                    best_score = score;
                    best_idx = j;
                }
            }
            moves.swap(i, best_idx);
            let m = moves[i];
            board.apply_move(&m, active_player);
            let score = minimax(board, depth - 1, opponent, eval, alpha, beta, stats);
            board.undo_move(m, active_player);

            max_eval = max_eval.max(score);
            alpha = alpha.max(score);
            if alpha >= beta {
                if m.capture == Free {
                    let d = depth as usize;
                    if stats.killer_moves[d][0] != Some(m) {
                        stats.killer_moves[d][1] = stats.killer_moves[d][0];
                        stats.killer_moves[d][0] = Some(m);
                    }
                }
                stats.cutoffs += 1;
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for i in 0..moves.len() {
            let mut best_idx = i;
            let mut best_score = i32::MIN;
            for j in i..moves.len() {
                let score = move_order_score(
                    &moves[j],
                    board.grid[moves[j].origin.row as usize][moves[j].origin.col as usize]
                        .get_piece(),
                    stats.killer_moves[depth as usize]
                        .get(0)
                        .and_then(|x| x.as_ref()),
                    stats.killer_moves[depth as usize]
                        .get(1)
                        .and_then(|x| x.as_ref()),
                );
                if score > best_score {
                    best_score = score;
                    best_idx = j;
                }
            }
            moves.swap(i, best_idx);
            let m = moves[i];
            board.apply_move(&m, active_player);
            let score = minimax(board, depth - 1, opponent, eval, alpha, beta, stats);
            board.undo_move(m, active_player);

            min_eval = min_eval.min(score);
            beta = beta.min(score);
            if alpha >= beta {
                if m.capture == Free {
                    let d = depth as usize;
                    if stats.killer_moves[d][0] != Some(m) {
                        stats.killer_moves[d][1] = stats.killer_moves[d][0];
                        stats.killer_moves[d][0] = Some(m);
                    }
                }
                stats.cutoffs += 1;
                break;
            }
        }
        min_eval
    }
}

fn is_mate_or_pat(active_player: Color, depth: u8) -> i32 {
    if active_player == Color::White {
        -MATE_SCORE + depth as i32
    } else {
        MATE_SCORE - depth as i32
    }
}

pub fn find_best_move<E: Evaluator>(
    board: &mut Board,
    active_player: Color,
    eval: &E,
    depth: u8,
    stats: &mut SearchStats,
) -> Option<Move> {
    let mut move_list = MoveList::new();
    generate_moves(board, &active_player, &mut move_list, false);
    let moves = &mut move_list.moves[..move_list.count];

    if moves.is_empty() {
        return None;
    }

    let opponent = match active_player {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let mut best_move = None;

    if active_player == Color::White {
        let mut best_score = i32::MIN;
        for i in 0..moves.len() {
            // Tri identique
            let mut best_idx = i;
            let mut current_max = i32::MIN;
            for j in i..moves.len() {
                let score = move_order_score(
                    &moves[j],
                    board.grid[moves[j].origin.row as usize][moves[j].origin.col as usize]
                        .get_piece(),
                    stats.killer_moves[depth as usize]
                        .get(0)
                        .and_then(|x| x.as_ref()),
                    stats.killer_moves[depth as usize]
                        .get(1)
                        .and_then(|x| x.as_ref()),
                );
                if score > current_max {
                    current_max = score;
                    best_idx = j;
                }
            }
            moves.swap(i, best_idx);
            let m = moves[i];

            board.apply_move(&m, active_player);
            let score = minimax(board, depth - 1, opponent, eval, i32::MIN, i32::MAX, stats);
            board.undo_move(m, active_player);

            if score > best_score {
                best_score = score;
                best_move = Some(m);
            }
        }
    } else {
        let mut best_score = i32::MAX;
        for i in 0..moves.len() {
            // Tri identique
            let mut best_idx = i;
            let mut current_max = i32::MIN;
            for j in i..moves.len() {
                let score = move_order_score(
                    &moves[j],
                    board.grid[moves[j].origin.row as usize][moves[j].origin.col as usize]
                        .get_piece(),
                    stats.killer_moves[depth as usize]
                        .get(0)
                        .and_then(|x| x.as_ref()),
                    stats.killer_moves[depth as usize]
                        .get(1)
                        .and_then(|x| x.as_ref()),
                );
                if score > current_max {
                    current_max = score;
                    best_idx = j;
                }
            }
            moves.swap(i, best_idx);
            let m = moves[i];

            board.apply_move(&m, active_player);
            let score = minimax(board, depth - 1, opponent, eval, i32::MIN, i32::MAX, stats);
            board.undo_move(m, active_player);

            if score < best_score {
                best_score = score;
                best_move = Some(m);
            }
        }
    }

    best_move
}

pub fn move_order_score(
    mv: &Move,
    attacker: Option<&Piece>,
    killer1: Option<&Move>,
    killer2: Option<&Move>,
) -> i32 {
    if Some(mv) == killer1 {
        return 1_000_000;
    }

    if Some(mv) == killer2 {
        return 900_000;
    }
    let capture_score = match mv.capture {
        Occupied(piece, _) => match piece {
            Pawn => 10,
            Knight => 30,
            Bishop => 30,
            Rook => 50,
            Queen => 90,
            King => 10000,
        },
        Free => 0,
    };

    let attacker_penalty = match attacker {
        Some(Pawn) => 1,
        Some(Knight) => 3,
        Some(Bishop) => 3,
        Some(Rook) => 5,
        Some(Queen) => 9,
        Some(King) => 100,
        None => 0,
    };
    let promotion_bonus = match mv.move_type {
        Promotion(_) => 800,
        _ => 0,
    };
    let check_bonus = if mv.check.is_some() { 50 } else { 0 };
    let mvv_lva = capture_score * 10 - attacker_penalty;

    mvv_lva + promotion_bonus + check_bonus
}

pub fn get_bot_move(
    difficulty: &PlayerType,
    board: &mut Board,
    active_player: Color,
    stats: &mut SearchStats,
) -> Option<Move> {
    match difficulty {
        Bot(Hard) => find_best_move(
            board,
            active_player,
            &PositionalEvaluator,
            HARD_DEPTH,
            stats,
        ),
        Bot(Medium) => find_best_move(
            board,
            active_player,
            &PositionalEvaluator,
            MEDIUM_DEPTH,
            stats,
        ),
        Bot(Easy) => {
            let mut move_list = MoveList::new();
            generate_moves(board, &active_player, &mut move_list, false);
            let moves = &mut move_list.moves[..move_list.count];
            let index = (Math::random() * moves.len() as f64).floor() as usize;
            Some(moves[index])
        }
        _ => None,
    }
}

pub fn quiescence<E: Evaluator>(
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    eval: &E,
    active_player: Color,
    stats: &mut SearchStats,
) -> i32 {
    let current_board_score = eval.evaluate(board);

    if current_board_score >= beta {
        return beta;
    }
    if current_board_score > alpha {
        alpha = current_board_score;
    }

    let mut move_list = MoveList::new();
    generate_moves(board, &active_player, &mut move_list, true);
    let moves = &mut move_list.moves[..move_list.count];

    let opponent = match active_player {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    for i in 0..moves.len() {
        board.apply_move(&moves[i], active_player);
        let score = -quiescence(board, -beta, -alpha, eval, opponent, stats);
        board.undo_move(moves[i], active_player);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    alpha
}
