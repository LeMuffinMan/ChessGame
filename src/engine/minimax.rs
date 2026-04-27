use crate::Board;
use crate::board::cell::Cell::*;
use crate::board::cell::Color;
use crate::board::cell::Piece::*;
use crate::board::is_king_exposed::is_king_exposed;
use crate::board::moves::move_gen::generate_moves;
use crate::board::moves::move_structs::Move;
use crate::board::moves::move_structs::MoveList;
use crate::board::moves::move_structs::MoveType::Promotion;
use crate::engine::evaluator::{
    evaluate, BISHOP_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE,
};
use crate::engine::move_ordering::move_order_score;
use crate::engine::search_context::SearchContext;
use crate::engine::ttentry::{TtEntry, TtFlag};
use crate::engine::zobris_table::zobrist;

const MATE_SCORE: i32 = 1_000_000;

pub fn minimax(
    board: &mut Board,
    depth: u8,
    active_player: Color,
    mut alpha: i32,
    mut beta: i32,
    ctx: &mut SearchContext,
    null_move_allowed: bool,
) -> i32 {
    ctx.incremente_node();

    if ctx.stats.max_nodes > 0 && ctx.stats.nodes >= ctx.stats.max_nodes {
        ctx.stats.aborted = true;
        return 0;
    }

    if depth == 0 {
        ctx.stats.leafs += 1;
        return quiescence_minimax(board, alpha, beta, active_player, ctx, 4);
    }

    let orig_alpha = alpha;
    let orig_beta = beta;

    // --- Probe TT ---
    if let Some(entry) = ctx.tt.get(&board.hash) {
        if entry.depth >= depth {
            match entry.flag {
                TtFlag::Exact => {
                    ctx.stats.cutoffs += 1;
                    ctx.stats.cutoffs_per_depth[ctx.stats.depth] += 1;
                    ctx.stats.total_cutoffs_depth += ctx.stats.depth;
                    ctx.stats.tt_hits += 1;
                    return entry.score;
                }
                TtFlag::LowerBound => alpha = alpha.max(entry.score),
                TtFlag::UpperBound => beta = beta.min(entry.score),
            }
            if alpha >= beta {
                ctx.stats.cutoffs += 1;
                ctx.stats.cutoffs_per_depth[ctx.stats.depth] += 1;
                ctx.stats.total_cutoffs_depth += ctx.stats.depth;
                ctx.stats.tt_hits += 1;
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
            // Pat : pénalise le camp gagnant qui a créé le pat
            const STALEMATE_CONTEMPT: i32 = 50;
            if board.score > 300 { -STALEMATE_CONTEMPT }
            else if board.score < -300 { STALEMATE_CONTEMPT }
            else { 0 }
        };
    }

    let opponent = match active_player {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let [killer1, killer2] = ctx.killers.get(depth as usize);

    // --- Null Move Pruning ---
    if null_move_allowed
        && depth >= 3
        && board.check.is_none()
        && has_non_pawn_material(board, active_player)
    {
        let zt = zobrist();
        let prev_ep = board.en_passant;
        let prev_hash = board.hash;
        board.hash ^= zt.side_to_move;
        if let Some(ep) = prev_ep {
            board.hash ^= zt.en_passant[ep.col as usize];
        }
        board.en_passant = None;
        if active_player == Color::White {
            let null_score = minimax(board, depth - 3, opponent, beta - 1, beta, ctx, false);
            board.en_passant = prev_ep;
            board.hash = prev_hash;
            if null_score >= beta {
                return beta;
            }
        } else {
            let null_score = minimax(
                board,
                depth - 3,
                opponent,
                alpha,
                alpha + 1,
                ctx,
                false,
            );
            board.en_passant = prev_ep;
            board.hash = prev_hash;
            if null_score <= alpha {
                return alpha;
            }
        }
    }

    if active_player == Color::White {
        moves.sort_unstable_by_key(|mv| {
            std::cmp::Reverse(move_order_score(
                mv,
                board.grid[mv.origin.row as usize][mv.origin.col as usize].get_piece(),
                killer1,
                killer2,
                &ctx.history,
            ))
        });
        let mut max_eval = i32::MIN;
        for (i, &m) in moves.iter().enumerate() {
            // Futility Pruning
            if depth == 1
                && m.capture == Free
                && !matches!(m.move_type, Promotion(_))
                && m.check.is_none()
                && board.score + QUEEN_VALUE < alpha
            {
                continue;
            }
            board.apply_move(&m, active_player);
            let gives_check = is_king_exposed(board, &opponent);
            let ext: u8 = u8::from(gives_check && depth == 1);
            ctx.stats.depth += 1;
            let score = if i == 0 {
                minimax(board, depth - 1 + ext, opponent, alpha, beta, ctx, true)
            } else {
                let is_quiet = m.capture == Free
                    && !matches!(m.move_type, Promotion(_))
                    && m.check.is_none()
                    && killer1 != Some(m)
                    && killer2 != Some(m);
                let r: u8 = if is_quiet && depth >= 3 && i >= 3 {
                    if i >= 6 { 2 } else { 1 }
                } else {
                    0
                };
                let scout = minimax(
                    board,
                    (depth - 1 + ext).saturating_sub(r),
                    opponent,
                    alpha,
                    alpha + 1,
                    ctx,
                    true,
                );
                if scout > alpha && (r > 0 || scout < beta) {
                    minimax(board, depth - 1 + ext, opponent, alpha, beta, ctx, true)
                } else {
                    scout
                }
            };
            ctx.stats.depth -= 1;
            board.undo_move(m, active_player);

            max_eval = max_eval.max(score);
            alpha = alpha.max(score);
            if alpha >= beta {
                if m.capture == Free {
                    ctx.killers.update(depth as usize, m);
                    let from = m.origin.row as usize * 8 + m.origin.col as usize;
                    let to = m.dest.row as usize * 8 + m.dest.col as usize;
                    ctx.history.update(from, to, depth);
                }
                ctx.stats.cutoffs += 1;
                ctx.stats.cutoffs_per_depth[ctx.stats.depth] += 1;
                ctx.stats.total_cutoffs_depth += ctx.stats.depth;
                break;
            }
        }
        let flag = if max_eval <= orig_alpha {
            TtFlag::UpperBound
        } else if max_eval >= orig_beta {
            TtFlag::LowerBound
        } else {
            TtFlag::Exact
        };
        let should_store = ctx.tt.get(&board.hash).map_or(true, |e| depth >= e.depth);
        if should_store {
            ctx.tt.insert(
                board.hash,
                TtEntry {
                    score: max_eval,
                    depth,
                    flag,
                },
            );
            ctx.stats.tt_stores += 1;
        }
        max_eval
    } else {
        moves.sort_unstable_by_key(|mv| {
            std::cmp::Reverse(move_order_score(
                mv,
                board.grid[mv.origin.row as usize][mv.origin.col as usize].get_piece(),
                killer1,
                killer2,
                &ctx.history,
            ))
        });
        let mut min_eval = i32::MAX;
        for (i, &m) in moves.iter().enumerate() {
            // Futility Pruning
            if depth == 1
                && m.capture == Free
                && !matches!(m.move_type, Promotion(_))
                && m.check.is_none()
                && board.score - QUEEN_VALUE > beta
            {
                continue;
            }
            board.apply_move(&m, active_player);
            let gives_check = is_king_exposed(board, &opponent);
            let ext: u8 = u8::from(gives_check && depth == 1);
            ctx.stats.depth += 1;
            let score = if i == 0 {
                minimax(board, depth - 1 + ext, opponent, alpha, beta, ctx, true)
            } else {
                let is_quiet = m.capture == Free
                    && !matches!(m.move_type, Promotion(_))
                    && m.check.is_none()
                    && killer1 != Some(m)
                    && killer2 != Some(m);
                let r: u8 = if is_quiet && depth >= 3 && i >= 3 {
                    if i >= 6 { 2 } else { 1 }
                } else {
                    0
                };
                let scout = minimax(
                    board,
                    (depth - 1 + ext).saturating_sub(r),
                    opponent,
                    beta - 1,
                    beta,
                    ctx,
                    true,
                );
                if scout < beta && (r > 0 || scout > alpha) {
                    minimax(board, depth - 1 + ext, opponent, alpha, beta, ctx, true)
                } else {
                    scout
                }
            };
            ctx.stats.depth -= 1;
            board.undo_move(m, active_player);

            min_eval = min_eval.min(score);
            beta = beta.min(score);
            if alpha >= beta {
                if m.capture == Free {
                    ctx.killers.update(depth as usize, m);
                    let from = m.origin.row as usize * 8 + m.origin.col as usize;
                    let to = m.dest.row as usize * 8 + m.dest.col as usize;
                    ctx.history.update(from, to, depth);
                }
                ctx.stats.cutoffs += 1;
                ctx.stats.cutoffs_per_depth[ctx.stats.depth] += 1;
                ctx.stats.total_cutoffs_depth += ctx.stats.depth;
                break;
            }
        }
        let flag = if min_eval <= orig_alpha {
            TtFlag::UpperBound
        } else if min_eval >= orig_beta {
            TtFlag::LowerBound
        } else {
            TtFlag::Exact
        };
        let should_store = ctx.tt.get(&board.hash).map_or(true, |e| depth >= e.depth);
        if should_store {
            ctx.tt.insert(
                board.hash,
                TtEntry {
                    score: min_eval,
                    depth,
                    flag,
                },
            );
            ctx.stats.tt_stores += 1;
        }
        min_eval
    }
}

fn has_non_pawn_material(board: &Board, color: Color) -> bool {
    for row in 0..8usize {
        for col in 0..8usize {
            if let Occupied(piece, c) = board.grid[row][col] {
                if c == color && !matches!(piece, Pawn | King) {
                    return true;
                }
            }
        }
    }
    false
}

fn is_mate_or_pat(active_player: Color, depth: u8) -> i32 {
    if active_player == Color::White {
        -MATE_SCORE + depth as i32
    } else {
        MATE_SCORE - depth as i32
    }
}

pub fn find_best_move(
    board: &mut Board,
    active_player: Color,
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
        moves.sort_unstable_by_key(|mv| {
            std::cmp::Reverse(move_order_score(
                mv,
                board.grid[mv.origin.row as usize][mv.origin.col as usize].get_piece(),
                killer1,
                killer2,
                &ctx.history,
            ))
        });
        let mut best_score = i32::MIN;
        let mut alpha = i32::MIN;
        for (i, &m) in moves.iter().enumerate() {
            board.apply_move(&m, active_player);
            let gives_check = is_king_exposed(board, &opponent);
            let ext: u8 = u8::from(gives_check && depth == 1);
            ctx.stats.depth += 1;
            let score = if i == 0 {
                minimax(board, depth - 1 + ext, opponent, alpha, i32::MAX, ctx, true)
            } else {
                let scout = minimax(
                    board,
                    depth - 1 + ext,
                    opponent,
                    alpha,
                    alpha + 1,
                    ctx,
                    true,
                );
                if scout > alpha {
                    minimax(board, depth - 1 + ext, opponent, alpha, i32::MAX, ctx, true)
                } else {
                    scout
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
        moves.sort_unstable_by_key(|mv| {
            std::cmp::Reverse(move_order_score(
                mv,
                board.grid[mv.origin.row as usize][mv.origin.col as usize].get_piece(),
                killer1,
                killer2,
                &ctx.history,
            ))
        });
        let mut best_score = i32::MAX;
        let mut beta = i32::MAX;
        for (i, &m) in moves.iter().enumerate() {
            board.apply_move(&m, active_player);
            let gives_check = is_king_exposed(board, &opponent);
            let ext: u8 = u8::from(gives_check && depth == 1);
            ctx.stats.depth += 1;
            let score = if i == 0 {
                minimax(board, depth - 1 + ext, opponent, i32::MIN, beta, ctx, true)
            } else {
                let scout = minimax(board, depth - 1 + ext, opponent, beta - 1, beta, ctx, true);
                if scout < beta {
                    minimax(board, depth - 1 + ext, opponent, i32::MIN, beta, ctx, true)
                } else {
                    scout
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

pub fn iterative_deepening(
    board: &mut Board,
    active_player: Color,
    max_depth: u8,
    ctx: &mut SearchContext,
) -> Option<Move> {
    let mut best_move = None;
    for depth in 1..=max_depth {
        ctx.stats.reset();
        let candidate = find_best_move(board, active_player, depth, ctx);
        if ctx.stats.aborted {
            break;
        }
        if candidate.is_some() {
            best_move = candidate;
        }
    }
    best_move
}

pub fn quiescence_minimax(
    board: &mut Board,
    mut alpha: i32,
    mut beta: i32,
    active_player: Color,
    ctx: &mut SearchContext,
    depth: i8,
) -> i32 {
    ctx.stats.quiescence_nodes += 1;

    let stand_pat = evaluate(board);

    if active_player == Color::White {
        if stand_pat >= beta {
            return beta;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }
    } else {
        if stand_pat <= alpha {
            return alpha;
        }
        if stand_pat < beta {
            beta = stand_pat;
        }
    }

    if depth <= 0 {
        return if active_player == Color::White {
            alpha
        } else {
            beta
        };
    }

    let mut move_list = MoveList::new();
    generate_moves(board, &active_player, &mut move_list, true);
    let opponent = if active_player == Color::White {
        Color::Black
    } else {
        Color::White
    };

    for i in 0..move_list.count {
        let m = move_list.moves[i];
        // Delta Pruning
        let capture_value = match m.capture {
            Occupied(piece, _) => match piece {
                Pawn => PAWN_VALUE,
                Knight => KNIGHT_VALUE,
                Bishop => BISHOP_VALUE,
                Rook => ROOK_VALUE,
                Queen => QUEEN_VALUE,
                King => 0,
            },
            Free => 0,
        };
        const DELTA: i32 = 200;
        if active_player == Color::White && stand_pat + capture_value + DELTA < alpha {
            continue;
        }
        if active_player == Color::Black && stand_pat - capture_value - DELTA > beta {
            continue;
        }

        board.apply_move(&m, active_player);
        let score = quiescence_minimax(board, alpha, beta, opponent, ctx, depth - 1);
        board.undo_move(m, active_player);

        if active_player == Color::White {
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                return beta;
            }
        } else {
            if score < beta {
                beta = score;
            }
            if alpha >= beta {
                return alpha;
            }
        }
    }

    if active_player == Color::White {
        alpha
    } else {
        beta
    }
}
