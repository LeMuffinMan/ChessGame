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
    BISHOP_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE, evaluate,
};
use crate::engine::move_ordering::move_order_score;
use crate::engine::search_context::SearchContext;
use crate::engine::ttentry::{TtEntry, TtFlag};
use crate::engine::zobris_table::zobrist;
use std::collections::HashMap;
use web_sys::window;

const MATE_SCORE: i32 = 1_000_000;
const MATE_THRESHOLD: i32 = 990_000;
const BOT_TIME_LIMIT: f64 = 250.0;

pub fn minimax(
    board: &mut Board,
    depth: u8,
    active_player: Color,
    mut alpha: i32,
    mut beta: i32,
    ctx: &mut SearchContext,
    null_move_allowed: bool,
    game_history: &HashMap<u64, usize>,
    fifty_count: u32,
    ply: u8,
) -> i32 {
    ctx.incremente_node();

    if ctx.stats.max_nodes > 0 && ctx.stats.nodes >= ctx.stats.max_nodes {
        ctx.stats.aborted = true;
        return 0;
    }

    if game_history.get(&board.hash).copied().unwrap_or(0) >= 2 {
        return 0;
    }

    if fifty_count >= 100 {
        return 0;
    }

    if depth == 0 {
        ctx.stats.leafs += 1;
        return quiescence_minimax(board, alpha, beta, active_player, ctx, 4);
    }

    let orig_alpha = alpha;
    let orig_beta = beta;

    // we probe the tt first
    let mut tt_move: Option<Move> = None;
    if let Some(entry) = ctx.tt.get(&board.hash) {
        tt_move = entry.best_move;
        if entry.depth >= depth {
            match entry.flag {
                TtFlag::Exact => {
                    ctx.stats.cutoffs += 1;
                    ctx.stats.cutoffs_per_depth[ctx.stats.depth] += 1;
                    ctx.stats.total_cutoffs_depth += ctx.stats.depth;
                    ctx.stats.tt_hits += 1;
                    let score = score_from_tt(entry.score, ply as i32);
                    return score;
                }
                TtFlag::LowerBound => alpha = alpha.max(entry.score),
                TtFlag::UpperBound => beta = beta.min(entry.score),
            }
            if alpha >= beta {
                ctx.stats.cutoffs += 1;
                ctx.stats.cutoffs_per_depth[ctx.stats.depth] += 1;
                ctx.stats.total_cutoffs_depth += ctx.stats.depth;
                ctx.stats.tt_hits += 1;
                let score = score_from_tt(entry.score, ply as i32);
                return score;
            }
        }
    }

    let mut move_list = MoveList::new();
    generate_moves(board, &active_player, &mut move_list, false);
    let moves = &mut move_list.moves[..move_list.count];

    if moves.is_empty() {
        return if is_king_exposed(board, &active_player) {
            is_mate_or_pat(active_player, ply)
        } else {
            //we want a winning bot to see a pat as not a good option
            const STALEMATE_CONTEMPT: i32 = 50;
            if board.score > 300 {
                -STALEMATE_CONTEMPT
            } else if board.score < -300 {
                STALEMATE_CONTEMPT
            } else {
                0
            }
        };
    }

    let opponent = match active_player {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    //if we found a quite move which gave advantage at same depth but other subbranch, it could be a fork, or a near mate,
    // our moveordering could lead us to explore them later, so killers list is our "strong choices" among previous quiet moves tested
    // so we want to try in first these move in other sub branches or depth, to cut earlier
    let [killer1, killer2] = ctx.killers.get(depth as usize);

    // when we reached d=3 at least, and we are not in check situation and we are not in zugzwang (endgame)
    // benefiting of the alpha / beta pruning, we can simulate we pass our turn, then check deeper
    // if doing so, we are still in advantage, opponent would not play rationnaly
    // the move who led us at our initial depth so we want to cut this branch
    if null_move_allowed
        && depth >= 3
        && board.check.is_none()
        && has_non_pawn_material(board, active_player)
        && has_non_pawn_material(board, opponent)
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
            let null_score = minimax(
                board,
                depth - 3,
                opponent,
                beta.saturating_sub(1),
                beta,
                ctx,
                false,
                game_history,
                fifty_count,
                ply + 1,
            );
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
                alpha.saturating_add(1),
                ctx,
                false,
                game_history,
                fifty_count,
                ply + 1,
            );
            board.en_passant = prev_ep;
            board.hash = prev_hash;
            if null_score <= alpha {
                return alpha;
            }
        }
    }

    //to compare with selective sort
    if active_player == Color::White {
        moves.sort_unstable_by_key(|mv| {
            std::cmp::Reverse(move_order_score(
                mv,
                board[(mv.origin.row as usize, mv.origin.col as usize)].get_piece(),
                killer1,
                killer2,
                &ctx.history,
                tt_move,
            ))
        });
        let mut max_eval = i32::MIN;
        let mut best_move_found: Option<Move> = None;
        for (i, &m) in moves.iter().enumerate() {
            // futility pruning : near the leafes, in a quiet situation, we want to seek moves that would give
            // an advantage now, so we skip the quiet moves, except if it could give a queen advantage (estimation of a good gain)
            if depth == 1
                && m.capture == Free
                && !matches!(m.move_type, Promotion(_))
                && m.check.is_none()
                && board.score + QUEEN_VALUE < alpha
            {
                continue;
            }
            let new_fifty = update_fifty_count(board, &m, fifty_count);
            board.apply_move(&m, active_player);
            let gives_check = is_king_exposed(board, &opponent);

            // check extension : if we won't have the depth to evaluate the result of the answer to this check (d will be 0, then -1, in quiecence search),
            // we check now with an extended depth to guarantee to have at least an answer to the check position, before geting a score
            // depth == 1 : we don't want to generalise this check, only near the leafs
            let ext: u8 = u8::from(gives_check && depth == 1);
            ctx.stats.depth += 1;
            let score = if i == 0 {
                minimax(
                    board,
                    depth - 1 + ext,
                    opponent,
                    alpha,
                    beta,
                    ctx,
                    true,
                    game_history,
                    new_fifty,
                    ply + 1,
                )
            } else {
                //for (at first sight) bad moves, we prune as much as possible
                let is_quiet = m.capture == Free
                    && !matches!(m.move_type, Promotion(_))
                    && m.check.is_none()
                    && killer1 != Some(m)
                    && killer2 != Some(m);

                //Late move reduction: in a quiet situation, early in the tree, but after 3 iterations in available moves,
                // r works as ext, if we already iterated 3 times, this move might be bad or average
                // So, we want to search r deeper now, scouting if this move worth to explore or to skip
                let r: u8 = if is_quiet && !gives_check && depth >= 3 && i >= 3 {
                    if i >= 6 { 2 } else { 1 }
                } else {
                    0
                };
                //Primary variation search : we send a null window for the first moves and the subs ones.
                // It's betting it's bad, and reducing search in it, but still giving a chance to prove it could lead to something better
                let scout = minimax(
                    board,
                    (depth - 1 + ext).saturating_sub(r),
                    opponent,
                    alpha,
                    alpha.saturating_add(1),
                    ctx,
                    true,
                    game_history,
                    new_fifty,
                    ply + 1,
                );
                // scout <= alpha : the move is bad as we bet, we won't research and benefit of the scout economy
                // scout > alpha r == 0 : fail high: the cut is reliable, we return scout, and the cutoff will occure by the caller
                // scout > alpha && r == 0 && scout > beta : the score was in the window, but we didnt gave the full window to search
                //  So we want to research with full window to get the real value of this move
                // scout > alpha && r > 0 : we were near the leafs and a may be a fail-high, so we want to research to confirm it and cut by caller if yes
                if scout > alpha && (r > 0 || scout < beta) {
                    minimax(
                        board,
                        depth - 1 + ext,
                        opponent,
                        alpha,
                        beta,
                        ctx,
                        true,
                        game_history,
                        new_fifty,
                        ply + 1,
                    )
                } else {
                    scout
                }
            };
            ctx.stats.depth -= 1;
            board.undo_move(m, active_player);

            if score > max_eval {
                max_eval = score;
                best_move_found = Some(m);
            }
            alpha = alpha.max(score);
            if alpha >= beta {
                if m.capture == Free {
                    ctx.killers.update(depth as usize, m);
                    let from = m.origin.row as usize * 8 + m.origin.col as usize;
                    let to = m.dest.row as usize * 8 + m.dest.col as usize;
                    // history keep quiet moves which occured a cut, ponderated with depth : a high depth cut is more interesting to keep
                    ctx.history.update(from, to, depth);
                }
                ctx.stats.cutoffs += 1;
                ctx.stats.cutoffs_per_depth[ctx.stats.depth] += 1;
                ctx.stats.total_cutoffs_depth += ctx.stats.depth;
                break;
            }
        }
        //faire une fonction get_flag
        let flag = if max_eval <= orig_alpha {
            TtFlag::UpperBound
        } else if max_eval >= orig_beta {
            TtFlag::LowerBound
        } else {
            TtFlag::Exact
        };
        let should_store = max_eval != i32::MIN
            && ctx.tt.get(&board.hash).map_or(true, |e| depth >= e.depth);
        if should_store {
            ctx.tt.insert(
                board.hash,
                TtEntry {
                    score: score_to_tt(max_eval, ply as i32),
                    depth,
                    flag,
                    best_move: best_move_found,
                },
            );
            ctx.stats.tt_stores += 1;
        }
        max_eval
    } else {
        moves.sort_unstable_by_key(|mv| {
            std::cmp::Reverse(move_order_score(
                mv,
                board[(mv.origin.row as usize, mv.origin.col as usize)].get_piece(),
                killer1,
                killer2,
                &ctx.history,
                tt_move,
            ))
        });
        let mut min_eval = i32::MAX;
        let mut best_move_found: Option<Move> = None;
        for (i, &m) in moves.iter().enumerate() {
            if depth == 1
                && m.capture == Free
                && !matches!(m.move_type, Promotion(_))
                && m.check.is_none()
                && board.score - QUEEN_VALUE > beta
            {
                continue;
            }
            let new_fifty = update_fifty_count(board, &m, fifty_count);
            board.apply_move(&m, active_player);
            let gives_check = is_king_exposed(board, &opponent);
            let ext: u8 = u8::from(gives_check && depth == 1);
            ctx.stats.depth += 1;
            let score = if i == 0 {
                minimax(
                    board,
                    depth - 1 + ext,
                    opponent,
                    alpha,
                    beta,
                    ctx,
                    true,
                    game_history,
                    new_fifty,
                    ply + 1,
                )
            } else {
                let is_quiet = m.capture == Free
                    && !matches!(m.move_type, Promotion(_))
                    && m.check.is_none()
                    && killer1 != Some(m)
                    && killer2 != Some(m);
                let r: u8 = if is_quiet && !gives_check && depth >= 3 && i >= 3 {
                    if i >= 6 { 2 } else { 1 }
                } else {
                    0
                };
                let scout = minimax(
                    board,
                    (depth - 1 + ext).saturating_sub(r),
                    opponent,
                    beta.saturating_sub(1),
                    beta,
                    ctx,
                    true,
                    game_history,
                    new_fifty,
                    ply + 1,
                );
                if scout < beta && (r > 0 || scout > alpha) {
                    minimax(
                        board,
                        depth - 1 + ext,
                        opponent,
                        alpha,
                        beta,
                        ctx,
                        true,
                        game_history,
                        new_fifty,
                        ply + 1,
                    )
                } else {
                    scout
                }
            };
            ctx.stats.depth -= 1;
            board.undo_move(m, active_player);

            if score < min_eval {
                min_eval = score;
                best_move_found = Some(m);
            }
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
        let should_store = min_eval != i32::MAX
            && ctx.tt.get(&board.hash).map_or(true, |e| depth >= e.depth);
        if should_store {
            ctx.tt.insert(
                board.hash,
                TtEntry {
                    score: score_to_tt(min_eval, ply as i32),
                    depth,
                    flag,
                    best_move: best_move_found,
                },
            );
            ctx.stats.tt_stores += 1;
        }
        min_eval
    }
}

fn update_fifty_count(board: &Board, m: &Move, fifty_count: u32) -> u32 {
    let is_pawn = board[(m.origin.row as usize, m.origin.col as usize)].get_piece() == Some(&Pawn);
    if m.capture != Free || is_pawn {
        0
    } else {
        fifty_count + 1
    }
}

fn has_non_pawn_material(board: &Board, color: Color) -> bool {
    for row in 0..8usize {
        for col in 0..8usize {
            if let Occupied(piece, c) = board[(row, col)] {
                if c == color && !matches!(piece, Pawn | King) {
                    return true;
                }
            }
        }
    }
    false
}

fn is_mate_or_pat(active_player: Color, ply: u8) -> i32 {
    if active_player == Color::White {
        -MATE_SCORE + ply as i32
    } else {
        MATE_SCORE - ply as i32
    }
}

pub fn find_best_move(
    board: &mut Board,
    active_player: Color,
    depth: u8,
    ctx: &mut SearchContext,
    game_history: &HashMap<u64, usize>,
    fifty_count: u32,
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

    let tt_move = ctx.tt.get(&board.hash).and_then(|e| e.best_move);
    let mut best_move = None;
    let [killer1, killer2] = ctx.killers.get(depth as usize);

    if active_player == Color::White {
        moves.sort_unstable_by_key(|mv| {
            std::cmp::Reverse(move_order_score(
                mv,
                board[(mv.origin.row as usize, mv.origin.col as usize)].get_piece(),
                killer1,
                killer2,
                &ctx.history,
                tt_move,
            ))
        });
        let mut best_score = i32::MIN;
        let mut alpha = i32::MIN;
        for (i, &m) in moves.iter().enumerate() {
            let new_fifty = update_fifty_count(board, &m, fifty_count);
            board.apply_move(&m, active_player);
            let gives_check = is_king_exposed(board, &opponent);
            let ext: u8 = u8::from(gives_check && depth == 1);
            ctx.stats.depth += 1;
            let score = if i == 0 {
                minimax(
                    board,
                    depth - 1 + ext,
                    opponent,
                    alpha,
                    i32::MAX,
                    ctx,
                    true,
                    game_history,
                    new_fifty,
                    0,
                )
            } else {
                let scout = minimax(
                    board,
                    depth - 1 + ext,
                    opponent,
                    alpha,
                    alpha.saturating_add(1),
                    ctx,
                    true,
                    game_history,
                    new_fifty,
                    0,
                );
                if scout > alpha {
                    minimax(
                        board,
                        depth - 1 + ext,
                        opponent,
                        alpha,
                        i32::MAX,
                        ctx,
                        true,
                        game_history,
                        new_fifty,
                        0,
                    )
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
                board[(mv.origin.row as usize, mv.origin.col as usize)].get_piece(),
                killer1,
                killer2,
                &ctx.history,
                tt_move,
            ))
        });
        let mut best_score = i32::MAX;
        let mut beta = i32::MAX;
        for (i, &m) in moves.iter().enumerate() {
            let new_fifty = update_fifty_count(board, &m, fifty_count);
            board.apply_move(&m, active_player);
            let gives_check = is_king_exposed(board, &opponent);
            let ext: u8 = u8::from(gives_check && depth == 1);
            ctx.stats.depth += 1;
            let score = if i == 0 {
                minimax(
                    board,
                    depth - 1 + ext,
                    opponent,
                    i32::MIN,
                    beta,
                    ctx,
                    true,
                    game_history,
                    new_fifty,
                    0,
                )
            } else {
                let scout = minimax(
                    board,
                    depth - 1 + ext,
                    opponent,
                    beta.saturating_sub(1),
                    beta,
                    ctx,
                    true,
                    game_history,
                    new_fifty,
                    0,
                );
                if scout < beta {
                    minimax(
                        board,
                        depth - 1 + ext,
                        opponent,
                        i32::MIN,
                        beta,
                        ctx,
                        true,
                        game_history,
                        new_fifty,
                        0,
                    )
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

pub fn timed_out_iterative_deepening(
    board: &mut Board,
    active_player: Color,
    max_depth: u8,
    ctx: &mut SearchContext,
    game_history: &HashMap<u64, usize>,
    fifty_count: u32,
    reached_depth: &mut u8,
) -> Option<Move> {
    let mut best_move = None;
    let performance = window().unwrap().performance().unwrap();
    let start = performance.now();
    for depth in 1..=max_depth {
        ctx.stats.reset();
        let candidate = find_best_move(board, active_player, depth, ctx, game_history, fifty_count);
        if ctx.stats.aborted {
            break;
        }
        if candidate.is_some() {
            *reached_depth = depth;
            best_move = candidate;
        }
        if performance.now() - start > BOT_TIME_LIMIT {
            break;
        }
    }
    best_move
}

pub fn iterative_deepening(
    board: &mut Board,
    active_player: Color,
    max_depth: u8,
    ctx: &mut SearchContext,
    game_history: &HashMap<u64, usize>,
    fifty_count: u32,
) -> Option<Move> {
    let mut best_move = None;
    for depth in 1..=max_depth {
        ctx.stats.reset();
        let candidate = find_best_move(board, active_player, depth, ctx, game_history, fifty_count);
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

fn score_to_tt(score: i32, ply: i32) -> i32 {
    if score > MATE_THRESHOLD {
        score + ply
    } else if score < -MATE_THRESHOLD {
        score - ply
    } else {
        score
    }
}

fn score_from_tt(score: i32, ply: i32) -> i32 {
    if score > MATE_THRESHOLD {
        score - ply
    } else if score < -MATE_THRESHOLD {
        score + ply
    } else {
        score
    }
}
