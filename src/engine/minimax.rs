use crate::Board;
use crate::board::cell::Cell::*;
use crate::board::cell::Color;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::moves::move_gen::generate_moves;
use crate::board::moves::move_structs::Move;
use crate::board::moves::move_structs::MoveList;
use crate::board::moves::move_structs::MoveType::Promotion;
use crate::engine::evaluator::Evaluator;
use crate::engine::ttentry::TtEntry;
use crate::engine::ttentry::TtFlag;
use std::collections::HashMap;
// use crate::engine::evaluator::PositionalEvaluator;
// use crate::engine::evaluator::*;
use crate::engine::search_stats::{HistoryTable, KillerTable, SearchContext, SearchStats};
#[cfg(target_arch = "wasm32")]
use js_sys::Math;

const MATE_SCORE: i32 = 1_000_000;
pub const EASY_DEPTH: u8 = 5;
pub const MEDIUM_DEPTH: u8 = 6;
pub const HARD_DEPTH: u8 = 7;

pub fn minimax<E: Evaluator>(
    board: &mut Board,
    depth: u8,
    active_player: Color,
    eval: &E,
    mut alpha: i32,
    mut beta: i32,
    stats: &mut SearchStats,
    killers: &mut KillerTable,
    history: &mut HistoryTable,
    tt: &mut HashMap<u64, TtEntry>,
) -> i32 {
    stats.nodes_per_depth[stats.depth] += 1;
    stats.total_node_depth += stats.depth;
    stats.nodes += 1;

    if stats.max_nodes > 0 && stats.nodes >= stats.max_nodes {
        stats.aborted = true;
        return 0;
    }

    if depth == 0 {
        stats.leafs += 1;
        // return match active_player {
        //     Color::White => quiescence(board, alpha, beta, eval, active_player, stats, 99),
        //     Color::Black => -quiescence(board, -beta, -alpha, eval, active_player, stats,99),
        // };
        // return quiescence(board, alpha, beta, eval, active_player, stats, 99);
        return board.score;
    }

    let orig_alpha = alpha;
    let orig_beta = beta;

    // --- Probe ---
    if let Some(entry) = tt.get(&board.hash) {
        if entry.depth >= depth {
            match entry.flag {
                TtFlag::Exact => {
                    stats.cutoffs += 1;
                    stats.cutoffs_per_depth[stats.depth] += 1;
                    stats.total_cutoffs_depth += stats.depth;
                    stats.tt_hits += 1;
                    return entry.score;
                }
                TtFlag::LowerBound => alpha = alpha.max(entry.score),
                TtFlag::UpperBound => beta = beta.min(entry.score),
            }
            if alpha >= beta {
                stats.cutoffs += 1;
                stats.cutoffs_per_depth[stats.depth] += 1;
                stats.total_cutoffs_depth += stats.depth;
                stats.tt_hits += 1;
                return entry.score;
            }
        }
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

    let [killer1, killer2] = killers.get(depth as usize);

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
                    killer1,
                    killer2,
                    history,
                );
                if score > best_score {
                    best_score = score;
                    best_idx = j;
                }
            }
            moves.swap(i, best_idx);
            let m = moves[i];
            board.apply_move(&m, active_player);
            stats.depth += 1;
            let score = minimax(
                board,
                depth - 1,
                opponent,
                eval,
                alpha,
                beta,
                stats,
                killers,
                history,
                tt,
            );
            stats.depth -= 1;
            board.undo_move(m, active_player);

            max_eval = max_eval.max(score);
            alpha = alpha.max(score);
            if alpha >= beta {
                if m.capture == Free {
                    killers.update(depth as usize, m);
                    let from = m.origin.row as usize * 8 + m.origin.col as usize;
                    let to = m.dest.row as usize * 8 + m.dest.col as usize;
                    history.update(from, to, depth);
                }
                stats.cutoffs += 1;
                stats.cutoffs_per_depth[stats.depth] += 1;
                stats.total_cutoffs_depth += stats.depth;
                break;
            }
        }
        // --- Store (replace only if new depth >= existing) ---
        let flag = if max_eval <= orig_alpha {
            TtFlag::UpperBound
        } else if max_eval >= orig_beta {
            TtFlag::LowerBound
        } else {
            TtFlag::Exact
        };
        let should_store = tt.get(&board.hash).map_or(true, |e| depth >= e.depth);
        if should_store {
            tt.insert(
                board.hash,
                TtEntry {
                    score: max_eval,
                    depth,
                    flag,
                },
            );
            stats.tt_stores += 1;
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
                    killer1,
                    killer2,
                    history,
                );
                if score > best_score {
                    best_score = score;
                    best_idx = j;
                }
            }
            moves.swap(i, best_idx);
            let m = moves[i];
            board.apply_move(&m, active_player);
            stats.depth += 1;
            let score = minimax(
                board,
                depth - 1,
                opponent,
                eval,
                alpha,
                beta,
                stats,
                killers,
                history,
                tt,
            );
            stats.depth -= 1;
            board.undo_move(m, active_player);

            min_eval = min_eval.min(score);
            beta = beta.min(score);
            if alpha >= beta {
                if m.capture == Free {
                    killers.update(depth as usize, m);
                    let from = m.origin.row as usize * 8 + m.origin.col as usize;
                    let to = m.dest.row as usize * 8 + m.dest.col as usize;
                    history.update(from, to, depth);
                }
                stats.cutoffs += 1;
                stats.cutoffs_per_depth[stats.depth] += 1;
                stats.total_cutoffs_depth += stats.depth;
                break;
            }
        }

        // --- Store (replace only if new depth >= existing) ---
        let flag = if min_eval <= orig_alpha {
            TtFlag::UpperBound
        } else if min_eval >= orig_beta {
            TtFlag::LowerBound
        } else {
            TtFlag::Exact
        };
        let should_store = tt.get(&board.hash).map_or(true, |e| depth >= e.depth);
        if should_store {
            tt.insert(
                board.hash,
                TtEntry {
                    score: min_eval,
                    depth,
                    flag,
                },
            );
            stats.tt_stores += 1;
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
    ctx: &mut SearchContext,
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
    let [killer1, killer2] = ctx.killers.get(depth as usize);

    if active_player == Color::White {
        let mut best_score = i32::MIN;
        let mut alpha = i32::MIN;
        for i in 0..moves.len() {
            let mut best_idx = i;
            let mut current_max = i32::MIN;
            for j in i..moves.len() {
                let score = move_order_score(
                    &moves[j],
                    board.grid[moves[j].origin.row as usize][moves[j].origin.col as usize]
                        .get_piece(),
                    killer1,
                    killer2,
                    &ctx.history,
                );
                if score > current_max {
                    current_max = score;
                    best_idx = j;
                }
            }
            moves.swap(i, best_idx);
            let m = moves[i];

            board.apply_move(&m, active_player);
            ctx.stats.depth += 1;
            let score = if best_move.is_none() {
                minimax(board, depth - 1, opponent, eval, alpha, i32::MAX, &mut ctx.stats, &mut ctx.killers, &mut ctx.history, &mut ctx.tt)
            } else {
                let null_score = minimax(board, depth - 1, opponent, eval, alpha, alpha + 1, &mut ctx.stats, &mut ctx.killers, &mut ctx.history, &mut ctx.tt);
                if null_score > alpha {
                    minimax(board, depth - 1, opponent, eval, alpha, i32::MAX, &mut ctx.stats, &mut ctx.killers, &mut ctx.history, &mut ctx.tt)
                } else {
                    null_score
                }
            };
            ctx.stats.depth -= 1;
            board.undo_move(m, active_player);

            if score > best_score {
                best_score = score;
                alpha = score;
                best_move = Some(m);
            }
        }
    } else {
        let mut best_score = i32::MAX;
        let mut beta = i32::MAX;
        for i in 0..moves.len() {
            let mut best_idx = i;
            let mut current_max = i32::MIN;
            for j in i..moves.len() {
                let score = move_order_score(
                    &moves[j],
                    board.grid[moves[j].origin.row as usize][moves[j].origin.col as usize]
                        .get_piece(),
                    killer1,
                    killer2,
                    &ctx.history,
                );
                if score > current_max {
                    current_max = score;
                    best_idx = j;
                }
            }
            moves.swap(i, best_idx);
            let m = moves[i];

            board.apply_move(&m, active_player);
            ctx.stats.depth += 1;
            let score = if best_move.is_none() {
                minimax(board, depth - 1, opponent, eval, i32::MIN, beta, &mut ctx.stats, &mut ctx.killers, &mut ctx.history, &mut ctx.tt)
            } else {
                let null_score = minimax(board, depth - 1, opponent, eval, beta - 1, beta, &mut ctx.stats, &mut ctx.killers, &mut ctx.history, &mut ctx.tt);
                if null_score < beta {
                    minimax(board, depth - 1, opponent, eval, i32::MIN, beta, &mut ctx.stats, &mut ctx.killers, &mut ctx.history, &mut ctx.tt)
                } else {
                    null_score
                }
            };
            ctx.stats.depth -= 1;
            board.undo_move(m, active_player);

            if score < best_score {
                best_score = score;
                beta = score;
                best_move = Some(m);
            }
        }
    }

    best_move
}

pub fn iterative_deepening<E: Evaluator>(
    board: &mut Board,
    active_player: Color,
    eval: &E,
    max_depth: u8,
    ctx: &mut SearchContext,
) -> Option<Move> {
    let mut best_move = None;
    for depth in 1..=max_depth {
        ctx.stats.reset();
        let candidate = find_best_move(board, active_player, eval, depth, ctx);
        if ctx.stats.aborted {
            break;
        }
        if candidate.is_some() {
            best_move = candidate;
        }
    }
    best_move
}

pub fn move_order_score(
    mv: &Move,
    attacker: Option<&Piece>,
    killer1: Option<Move>,
    killer2: Option<Move>,
    history: &HistoryTable,
) -> i32 {
    if killer1 == Some(*mv) {
        return 1_000_000;
    }

    if killer2 == Some(*mv) {
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

    // Captures always rank above quiet moves — offset ensures the hierarchy:
    // killers (1M, 900K) > captures (200K+) > quiet+history (0–199K)
    if capture_score > 0 {
        return 200_000 + mvv_lva + promotion_bonus + check_bonus;
    }

    let history_bonus = {
        let from = mv.origin.row as usize * 8 + mv.origin.col as usize;
        let to = mv.dest.row as usize * 8 + mv.dest.col as usize;
        history.get(from, to) as i32
    };

    promotion_bonus + check_bonus + history_bonus
}

pub fn quiescence<E: Evaluator>(
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    eval: &E,
    active_player: Color,
    stats: &mut SearchStats,
    depth: i8,
) -> i32 {
    if stats.max_nodes > 0 && stats.nodes >= stats.max_nodes {
        stats.aborted = true;
        return 0;
    }

    let stand_pat = if active_player == Color::White {
        board.score
    } else {
        -board.score
    };

    if stand_pat >= beta {
        return beta;
    }
    if depth <= 0 {
        return alpha.max(stand_pat);
    }
    if stand_pat + 900 < alpha {
        return alpha;
    }
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut move_list = MoveList::new();
    generate_moves(board, &active_player, &mut move_list, true);

    let opponent = match active_player {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    for i in 0..move_list.count {
        let m = move_list.moves[i];
        board.apply_move(&m, active_player);
        let score = -quiescence(board, -beta, -alpha, eval, opponent, stats, depth - 1);
        board.undo_move(m, active_player);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    alpha
}
