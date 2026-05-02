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
use crate::engine::search_context::{SearchContext, SearchParams, TT_SIZE};
use crate::engine::ttentry::{TtEntry, TtFlag};
use crate::engine::zobrist::zobrist;

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        * 1000.0
}

const MATE_SCORE: i32 = 1_000_000;
const MATE_THRESHOLD: i32 = 990_000;

pub fn minimax(
    board: &mut Board,
    depth: u8,
    active_player: Color,
    mut alpha: i32,
    mut beta: i32,
    ply: u8,
    params: &mut SearchParams,
) -> i32 {
    params.ctx.incremente_node();

    if params.ctx.stats.max_nodes > 0 && params.ctx.stats.nodes >= params.ctx.stats.max_nodes {
        params.ctx.stats.aborted = true;
        return 0;
    }

    if params.game_history.get(&board.hash).copied().unwrap_or(0) >= 2 {
        return 0;
    }

    if params.fifty_count >= 100 {
        return 0;
    }

    if depth == 0 {
        params.ctx.stats.leafs += 1;
        return quiescence_minimax(board, alpha, beta, active_player, params.ctx, 4, ply + 1);
    }

    let orig_alpha = alpha;
    let orig_beta = beta;

    let mut tt_move: Option<Move> = None;
    {
        let idx = (board.hash as usize) & (TT_SIZE - 1);
        let entry = params.ctx.tt[idx];
        if entry.key == board.hash {
            tt_move = entry.best_move;
            if entry.generation == params.ctx.tt_generation && entry.depth >= depth {
                match entry.flag {
                    TtFlag::Exact => {
                        params.ctx.stats.cutoffs += 1;
                        params.ctx.stats.cutoffs_per_depth[params.ctx.stats.depth] += 1;
                        params.ctx.stats.total_cutoffs_depth += params.ctx.stats.depth;
                        params.ctx.stats.tt_hits += 1;
                        let score = score_from_tt(entry.score, ply as i32);
                        return score;
                    }
                    TtFlag::LowerBound => alpha = alpha.max(entry.score),
                    TtFlag::UpperBound => beta = beta.min(entry.score),
                }
                if alpha >= beta {
                    params.ctx.stats.cutoffs += 1;
                    params.ctx.stats.cutoffs_per_depth[params.ctx.stats.depth] += 1;
                    params.ctx.stats.total_cutoffs_depth += params.ctx.stats.depth;
                    params.ctx.stats.tt_hits += 1;
                    let score = score_from_tt(entry.score, ply as i32);
                    return score;
                }
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

    let [killer1, killer2] = params.ctx.killers.get(depth as usize);

    let original_null = params.null_move_allowed;
    let original_fifty = params.fifty_count;

    if params.null_move_allowed
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
        params.null_move_allowed = false;
        if active_player == Color::White {
            let null_score = minimax(
                board,
                depth - 3,
                opponent,
                beta.saturating_sub(1),
                beta,
                ply + 1,
                params,
            );
            board.en_passant = prev_ep;
            board.hash = prev_hash;
            params.null_move_allowed = original_null;
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
                ply + 1,
                params,
            );
            board.en_passant = prev_ep;
            board.hash = prev_hash;
            params.null_move_allowed = original_null;
            if null_score <= alpha {
                return alpha;
            }
        }
    }

    params.null_move_allowed = true;

    if active_player == Color::White {
        moves.sort_unstable_by_key(|mv| {
            std::cmp::Reverse(move_order_score(
                mv,
                board[(mv.origin.row as usize, mv.origin.col as usize)].get_piece(),
                killer1,
                killer2,
                &params.ctx.history,
                tt_move,
            ))
        });
        let mut max_eval = i32::MIN;
        let mut best_move_found: Option<Move> = None;
        for (i, &m) in moves.iter().enumerate() {
            if depth == 1
                && m.capture == Free
                && !matches!(m.move_type, Promotion(_))
                && m.check.is_none()
                && board.score + QUEEN_VALUE < alpha
            {
                continue;
            }
            params.fifty_count = update_fifty_count(board, &m, original_fifty);
            board.apply_move(&m, active_player);
            let gives_check = is_king_exposed(board, &opponent);

            let ext: u8 = u8::from(gives_check && depth == 1);
            params.ctx.stats.depth += 1;
            let score = if i == 0 {
                minimax(
                    board,
                    depth - 1 + ext,
                    opponent,
                    alpha,
                    beta,
                    ply + 1,
                    params,
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
                    alpha,
                    alpha.saturating_add(1),
                    ply + 1,
                    params,
                );
                if scout > alpha && (r > 0 || scout < beta) {
                    minimax(
                        board,
                        depth - 1 + ext,
                        opponent,
                        alpha,
                        beta,
                        ply + 1,
                        params,
                    )
                } else {
                    scout
                }
            };
            params.ctx.stats.depth -= 1;
            params.fifty_count = original_fifty;
            board.undo_move(m, active_player);

            if score > max_eval {
                max_eval = score;
                best_move_found = Some(m);
            }
            alpha = alpha.max(score);
            if alpha >= beta {
                if m.capture == Free {
                    params.ctx.killers.update(depth as usize, m);
                    let from = m.origin.row as usize * 8 + m.origin.col as usize;
                    let to = m.dest.row as usize * 8 + m.dest.col as usize;
                    params.ctx.history.update(from, to, depth);
                }
                params.ctx.stats.cutoffs += 1;
                params.ctx.stats.cutoffs_per_depth[params.ctx.stats.depth] += 1;
                params.ctx.stats.total_cutoffs_depth += params.ctx.stats.depth;
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
        if max_eval != i32::MIN {
            let idx = (board.hash as usize) & (TT_SIZE - 1);
            let slot = &params.ctx.tt[idx];
            if slot.key == 0 || slot.generation != params.ctx.tt_generation || depth >= slot.depth {
                params.ctx.tt[idx] = TtEntry {
                    key: board.hash,
                    score: score_to_tt(max_eval, ply as i32),
                    depth,
                    generation: params.ctx.tt_generation,
                    flag,
                    best_move: best_move_found,
                };
                params.ctx.stats.tt_stores += 1;
            }
        }
        params.null_move_allowed = original_null;
        max_eval
    } else {
        moves.sort_unstable_by_key(|mv| {
            std::cmp::Reverse(move_order_score(
                mv,
                board[(mv.origin.row as usize, mv.origin.col as usize)].get_piece(),
                killer1,
                killer2,
                &params.ctx.history,
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
            params.fifty_count = update_fifty_count(board, &m, original_fifty);
            board.apply_move(&m, active_player);
            let gives_check = is_king_exposed(board, &opponent);
            let ext: u8 = u8::from(gives_check && depth == 1);
            params.ctx.stats.depth += 1;
            let score = if i == 0 {
                minimax(
                    board,
                    depth - 1 + ext,
                    opponent,
                    alpha,
                    beta,
                    ply + 1,
                    params,
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
                    ply + 1,
                    params,
                );
                if scout < beta && (r > 0 || scout > alpha) {
                    minimax(
                        board,
                        depth - 1 + ext,
                        opponent,
                        alpha,
                        beta,
                        ply + 1,
                        params,
                    )
                } else {
                    scout
                }
            };
            params.ctx.stats.depth -= 1;
            params.fifty_count = original_fifty;
            board.undo_move(m, active_player);

            if score < min_eval {
                min_eval = score;
                best_move_found = Some(m);
            }
            beta = beta.min(score);
            if alpha >= beta {
                if m.capture == Free {
                    params.ctx.killers.update(depth as usize, m);
                    let from = m.origin.row as usize * 8 + m.origin.col as usize;
                    let to = m.dest.row as usize * 8 + m.dest.col as usize;
                    params.ctx.history.update(from, to, depth);
                }
                params.ctx.stats.cutoffs += 1;
                params.ctx.stats.cutoffs_per_depth[params.ctx.stats.depth] += 1;
                params.ctx.stats.total_cutoffs_depth += params.ctx.stats.depth;
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
        if min_eval != i32::MAX {
            let idx = (board.hash as usize) & (TT_SIZE - 1);
            let slot = &params.ctx.tt[idx];
            if slot.key == 0 || slot.generation != params.ctx.tt_generation || depth >= slot.depth {
                params.ctx.tt[idx] = TtEntry {
                    key: board.hash,
                    score: score_to_tt(min_eval, ply as i32),
                    depth,
                    generation: params.ctx.tt_generation,
                    flag,
                    best_move: best_move_found,
                };
                params.ctx.stats.tt_stores += 1;
            }
        }
        params.null_move_allowed = original_null;
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
            if let Occupied(piece, c) = board[(row, col)]
                && c == color
                && !matches!(piece, Pawn | King)
            {
                return true;
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
    alpha: i32,
    beta: i32,
    params: &mut SearchParams,
) -> (Option<Move>, i32) {
    let mut move_list = MoveList::new();
    generate_moves(board, &active_player, &mut move_list, false);
    let moves = &mut move_list.moves[..move_list.count];

    if moves.is_empty() {
        return (None, 0);
    }

    let opponent = match active_player {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let tt_move = {
        let idx = (board.hash as usize) & (TT_SIZE - 1);
        let e = params.ctx.tt[idx];
        if e.key == board.hash {
            e.best_move
        } else {
            None
        }
    };
    let mut best_move = None;
    let mut best_score;
    let [killer1, killer2] = params.ctx.killers.get(depth as usize);
    let original_fifty = params.fifty_count;
    params.null_move_allowed = true;

    if active_player == Color::White {
        moves.sort_unstable_by_key(|mv| {
            std::cmp::Reverse(move_order_score(
                mv,
                board[(mv.origin.row as usize, mv.origin.col as usize)].get_piece(),
                killer1,
                killer2,
                &params.ctx.history,
                tt_move,
            ))
        });
        best_score = i32::MIN;
        let mut alpha = alpha;
        for (i, &m) in moves.iter().enumerate() {
            params.fifty_count = update_fifty_count(board, &m, original_fifty);
            board.apply_move(&m, active_player);
            let gives_check = is_king_exposed(board, &opponent);
            let ext: u8 = u8::from(gives_check && depth == 1);
            params.ctx.stats.depth += 1;
            let score = if i == 0 {
                minimax(board, depth - 1 + ext, opponent, alpha, beta, 0, params)
            } else {
                let scout = minimax(
                    board,
                    depth - 1 + ext,
                    opponent,
                    alpha,
                    alpha.saturating_add(1),
                    0,
                    params,
                );
                if scout > alpha {
                    minimax(board, depth - 1 + ext, opponent, alpha, beta, 0, params)
                } else {
                    scout
                }
            };
            params.ctx.stats.depth -= 1;
            params.fifty_count = original_fifty;
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
                &params.ctx.history,
                tt_move,
            ))
        });
        best_score = i32::MAX;
        let mut beta = beta;
        for (i, &m) in moves.iter().enumerate() {
            params.fifty_count = update_fifty_count(board, &m, original_fifty);
            board.apply_move(&m, active_player);
            let gives_check = is_king_exposed(board, &opponent);
            let ext: u8 = u8::from(gives_check && depth == 1);
            params.ctx.stats.depth += 1;
            let score = if i == 0 {
                minimax(board, depth - 1 + ext, opponent, alpha, beta, 0, params)
            } else {
                let scout = minimax(
                    board,
                    depth - 1 + ext,
                    opponent,
                    beta.saturating_sub(1),
                    beta,
                    0,
                    params,
                );
                if scout < beta {
                    minimax(board, depth - 1 + ext, opponent, alpha, beta, 0, params)
                } else {
                    scout
                }
            };
            params.ctx.stats.depth -= 1;
            params.fifty_count = original_fifty;
            board.undo_move(m, active_player);
            if score < best_score {
                best_score = score;
                beta = score;
                best_move = Some(m);
            }
        }
    }

    (best_move, best_score)
}

fn aspiration_search(
    board: &mut Board,
    active_player: Color,
    depth: u8,
    prev_score: i32,
    params: &mut SearchParams,
) -> (Option<Move>, i32) {
    const INIT_DELTA: i32 = 50;
    let mut delta = INIT_DELTA;
    let mut alpha = prev_score.saturating_sub(delta);
    let mut beta = prev_score.saturating_add(delta);

    loop {
        params.ctx.stats.reset();
        let (mv, score) = find_best_move(board, active_player, depth, alpha, beta, params);

        if params.ctx.stats.aborted {
            return (mv, score);
        }

        if score <= alpha {
            alpha = score.saturating_sub(delta);
            delta = delta.saturating_mul(2);
        } else if score >= beta {
            beta = score.saturating_add(delta);
            delta = delta.saturating_mul(2);
        } else {
            return (mv, score);
        }

        if delta >= MATE_SCORE {
            params.ctx.stats.reset();
            return find_best_move(board, active_player, depth, i32::MIN, i32::MAX, params);
        }
    }
}

pub fn timed_out_iterative_deepening(
    board: &mut Board,
    active_player: Color,
    max_depth: u8,
    reached_depth: &mut u8,
    timeout: f64,
    params: &mut SearchParams,
) -> Option<Move> {
    let mut best_move = None;
    let mut prev_score = 0i32;
    let start = now_ms();
    for depth in 1..=max_depth {
        let (candidate, score) = if depth <= 2 {
            params.ctx.stats.reset();
            find_best_move(board, active_player, depth, i32::MIN, i32::MAX, params)
        } else {
            aspiration_search(board, active_player, depth, prev_score, params)
        };
        if params.ctx.stats.aborted {
            break;
        }
        if candidate.is_some() {
            *reached_depth = depth;
            best_move = candidate;
            prev_score = score;
        }
        if now_ms() - start > timeout {
            break;
        }
    }
    best_move
}

pub fn iterative_deepening(
    board: &mut Board,
    active_player: Color,
    max_depth: u8,
    params: &mut SearchParams,
) -> Option<Move> {
    let mut best_move = None;
    let mut prev_score = 0i32;
    for depth in 1..=max_depth {
        let (candidate, score) = if depth <= 2 {
            params.ctx.stats.reset();
            find_best_move(board, active_player, depth, i32::MIN, i32::MAX, params)
        } else {
            aspiration_search(board, active_player, depth, prev_score, params)
        };
        if params.ctx.stats.aborted {
            break;
        }
        if candidate.is_some() {
            best_move = candidate;
            prev_score = score;
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
    ply: u8,
) -> i32 {
    ctx.stats.quiescence_nodes += 1;

    let orig_alpha = alpha;
    let orig_beta = beta;
    let q_depth = depth.max(0) as u8;

    {
        let idx = (board.hash as usize) & (TT_SIZE - 1);
        let entry = ctx.tt[idx];
        if entry.key == board.hash
            && entry.generation == ctx.tt_generation
            && entry.depth >= q_depth
        {
            let s = score_from_tt(entry.score, ply as i32);
            match entry.flag {
                TtFlag::Exact => return s,
                TtFlag::LowerBound => alpha = alpha.max(s),
                TtFlag::UpperBound => beta = beta.min(s),
            }
            if alpha >= beta {
                return s;
            }
        }
    }

    let stand_pat = evaluate(board);
    let mut best_move_found: Option<Move> = None;

    if active_player == Color::White {
        if stand_pat >= beta {
            {
                let idx = (board.hash as usize) & (TT_SIZE - 1);
                let slot = &ctx.tt[idx];
                if slot.key == 0 || slot.generation != ctx.tt_generation || q_depth >= slot.depth {
                    ctx.tt[idx] = TtEntry {
                        key: board.hash,
                        score: score_to_tt(stand_pat, ply as i32),
                        depth: q_depth,
                        generation: ctx.tt_generation,
                        flag: TtFlag::LowerBound,
                        best_move: None,
                    };
                }
            }
            return beta;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }
    } else {
        if stand_pat <= alpha {
            {
                let idx = (board.hash as usize) & (TT_SIZE - 1);
                let slot = &ctx.tt[idx];
                if slot.key == 0 || slot.generation != ctx.tt_generation || q_depth >= slot.depth {
                    ctx.tt[idx] = TtEntry {
                        key: board.hash,
                        score: score_to_tt(stand_pat, ply as i32),
                        depth: q_depth,
                        generation: ctx.tt_generation,
                        flag: TtFlag::UpperBound,
                        best_move: None,
                    };
                }
            }
            return alpha;
        }
        if stand_pat < beta {
            beta = stand_pat;
        }
    }

    if depth <= 0 {
        let result = if active_player == Color::White {
            alpha
        } else {
            beta
        };
        let flag = if result <= orig_alpha {
            TtFlag::UpperBound
        } else if result >= orig_beta {
            TtFlag::LowerBound
        } else {
            TtFlag::Exact
        };
        {
            let idx = (board.hash as usize) & (TT_SIZE - 1);
            let slot = &ctx.tt[idx];
            if slot.key == 0 || slot.generation != ctx.tt_generation || q_depth >= slot.depth {
                ctx.tt[idx] = TtEntry {
                    key: board.hash,
                    score: score_to_tt(result, ply as i32),
                    depth: q_depth,
                    generation: ctx.tt_generation,
                    flag,
                    best_move: None,
                };
            }
        }
        return result;
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
        let score = quiescence_minimax(board, alpha, beta, opponent, ctx, depth - 1, ply + 1);
        board.undo_move(m, active_player);

        if active_player == Color::White {
            if score > alpha {
                alpha = score;
                best_move_found = Some(m);
            }
            if alpha >= beta {
                {
                    let idx = (board.hash as usize) & (TT_SIZE - 1);
                    let slot = &ctx.tt[idx];
                    if slot.key == 0
                        || slot.generation != ctx.tt_generation
                        || q_depth >= slot.depth
                    {
                        ctx.tt[idx] = TtEntry {
                            key: board.hash,
                            score: score_to_tt(alpha, ply as i32),
                            depth: q_depth,
                            generation: ctx.tt_generation,
                            flag: TtFlag::LowerBound,
                            best_move: best_move_found,
                        };
                    }
                }
                return beta;
            }
        } else {
            if score < beta {
                beta = score;
                best_move_found = Some(m);
            }
            if alpha >= beta {
                {
                    let idx = (board.hash as usize) & (TT_SIZE - 1);
                    let slot = &ctx.tt[idx];
                    if slot.key == 0
                        || slot.generation != ctx.tt_generation
                        || q_depth >= slot.depth
                    {
                        ctx.tt[idx] = TtEntry {
                            key: board.hash,
                            score: score_to_tt(beta, ply as i32),
                            depth: q_depth,
                            generation: ctx.tt_generation,
                            flag: TtFlag::UpperBound,
                            best_move: best_move_found,
                        };
                    }
                }
                return alpha;
            }
        }
    }

    let result = if active_player == Color::White {
        alpha
    } else {
        beta
    };
    let flag = if result <= orig_alpha {
        TtFlag::UpperBound
    } else if result >= orig_beta {
        TtFlag::LowerBound
    } else {
        TtFlag::Exact
    };
    {
        let idx = (board.hash as usize) & (TT_SIZE - 1);
        let slot = &ctx.tt[idx];
        if slot.key == 0 || slot.generation != ctx.tt_generation || q_depth >= slot.depth {
            ctx.tt[idx] = TtEntry {
                key: board.hash,
                score: score_to_tt(result, ply as i32),
                depth: q_depth,
                generation: ctx.tt_generation,
                flag,
                best_move: best_move_found,
            };
        }
    }
    result
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
