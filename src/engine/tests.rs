use crate::Board;
use crate::board::CastleRights;
use crate::board::cell::Cell::{Free, Occupied};
use crate::board::cell::Color;
use crate::board::cell::Color::{Black, White};
use crate::board::cell::Coord;
use crate::board::cell::Piece::{King, Pawn, Queen, Rook};
use crate::board::moves::move_gen::generate_moves;
use crate::board::moves::move_structs::MoveList;
use crate::engine::bench::{KIWIPETE_FEN, PAWN_ENDING_FEN};
use crate::engine::evaluator::{evaluate, get_piece_value_at, non_pawn_raw};
use crate::engine::minimax::{find_best_move, iterative_deepening, minimax, quiescence_minimax};
use crate::engine::search_context::{SearchContext, SearchParams, TT_SIZE};
use crate::engine::time_manager::{Budget, TimeControl, plan};
use crate::engine::ttentry::{TtEntry, TtFlag};
use std::collections::HashMap;

const POSITION_4_FEN: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
const POSITION_5_FEN: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

fn coord(row: u8, col: u8) -> Coord {
    Coord { row, col }
}

fn empty_board(white_king: Coord, black_king: Coord) -> Board {
    let mut board = Board::init_board();
    for r in 0..8 {
        for c in 0..8 {
            board[(r, c)] = Free;
        }
    }
    board[(white_king.row as usize, white_king.col as usize)] = Occupied(King, White);
    board[(black_king.row as usize, black_king.col as usize)] = Occupied(King, Black);
    board.white_king = white_king;
    board.black_king = black_king;
    board.white_castle = CastleRights {
        long: false,
        short: false,
    };
    board.black_castle = CastleRights {
        long: false,
        short: false,
    };
    board
}

fn test_ctx() -> SearchContext {
    SearchContext::new()
}

fn recompute_score(board: &mut Board) {
    board.score = 0;
    board.non_pawn_material = 0;
    for r in 0..8 {
        for c in 0..8 {
            if let Occupied(piece, color) = board[(r, c)] {
                board.score += get_piece_value_at(
                    &piece,
                    &color,
                    &Coord {
                        row: r as u8,
                        col: c as u8,
                    },
                );
                board.non_pawn_material += non_pawn_raw(&piece);
            }
        }
    }
}

pub fn perft(board: &mut Board, color: Color, depth: u8) -> u64 {
    let mut res = 0;
    if depth == 0 {
        return 1;
    }

    let mut move_list = MoveList::new();
    generate_moves(board, &color, &mut move_list, false);
    let moves = &move_list.moves[..move_list.count];

    let opponent = match color {
        White => Black,
        Black => White,
    };

    for m in moves {
        board.apply_move(m, color);
        res += perft(board, opponent, depth - 1);
        board.undo_move(*m, color);
    }
    res
}

#[test]
fn perft_start_d1() {
    let mut board = Board::init_board();
    assert_eq!(perft(&mut board, White, 1), 20);
}

#[test]
fn perft_start_d2() {
    let mut board = Board::init_board();
    assert_eq!(perft(&mut board, White, 2), 400);
}

#[test]
fn perft_start_d3() {
    let mut board = Board::init_board();
    assert_eq!(perft(&mut board, White, 3), 8902);
}

#[test]
fn perft_start_d4() {
    let mut board = Board::init_board();
    assert_eq!(perft(&mut board, White, 4), 197281);
}

#[test]
fn perft_kiwipete_d1() {
    let fen = Board::board_from_fen(KIWIPETE_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 1), 48);
}

#[test]
fn perft_kiwipete_d2() {
    let fen = Board::board_from_fen(KIWIPETE_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 2), 2039);
}

#[test]
fn perft_kiwipete_d3() {
    let fen = Board::board_from_fen(KIWIPETE_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 3), 97862);
}

#[test]
fn perft_pawn_ending_d1() {
    let fen = Board::board_from_fen(PAWN_ENDING_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 1), 14);
}

#[test]
fn perft_pawn_ending_d2() {
    let fen = Board::board_from_fen(PAWN_ENDING_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 2), 191);
}

#[test]
fn perft_pawn_ending_d3() {
    let fen = Board::board_from_fen(PAWN_ENDING_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 3), 2812);
}

#[test]
fn perft_pawn_ending_d4() {
    let fen = Board::board_from_fen(PAWN_ENDING_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 4), 43238);
}

#[test]
fn perft_position_4_d1() {
    let fen = Board::board_from_fen(POSITION_4_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 1), 6);
}

#[test]
fn perft_position_4_d2() {
    let fen = Board::board_from_fen(POSITION_4_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 2), 264);
}

#[test]
fn perft_position_4_d3() {
    let fen = Board::board_from_fen(POSITION_4_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 3), 9467);
}

#[test]
fn perft_position_4_d4() {
    let fen = Board::board_from_fen(POSITION_4_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 4), 422333);
}

#[test]
fn perft_position_5_d1() {
    let fen = Board::board_from_fen(POSITION_5_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 1), 44);
}

#[test]
fn perft_position_5_d2() {
    let fen = Board::board_from_fen(POSITION_5_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 2), 1486);
}

#[test]
fn perft_position_5_d3() {
    let fen = Board::board_from_fen(POSITION_5_FEN);
    let mut board = fen.board;
    assert_eq!(perft(&mut board, fen.active_color, 3), 62379);
}

#[test]
fn test_evaluate_equal_material() {
    let mut board = empty_board(coord(0, 0), coord(7, 7));
    recompute_score(&mut board);
    assert_eq!(evaluate(&board), 0);
}

#[test]
fn test_evaluate_white_queen_advantage() {
    let mut board = empty_board(coord(0, 0), coord(7, 7));
    board[(3, 3)] = Occupied(Queen, White);
    recompute_score(&mut board);
    assert_eq!(evaluate(&board), 1355);
}

// White rook on d4 and black queen on d5 not defended : bot should take
#[test]
fn test_captures_free_queen() {
    let mut board = empty_board(coord(0, 0), coord(7, 7));
    board[(3, 3)] = Occupied(Rook, White);
    board[(4, 3)] = Occupied(Queen, Black);
    recompute_score(&mut board);
    board.sync_hash(White);

    let mut ctx = test_ctx();
    let history = HashMap::new();
    let mut params = SearchParams::new(&mut ctx, &history, 0);
    let (mv, _) = find_best_move(&mut board, White, 2, i32::MIN, i32::MAX, &mut params);
    let mv = mv.expect("should find a move");
    assert_eq!(mv.origin, coord(3, 3));
    assert_eq!(mv.dest, coord(4, 3));
}

// The bot should not take pawn with rook, as opponent rook is protecting the pawn
#[test]
fn test_avoids_losing_rook_depth2() {
    let mut board = empty_board(coord(0, 0), coord(7, 7));
    board[(3, 3)] = Occupied(Rook, White);
    board[(3, 4)] = Occupied(Pawn, Black);
    board[(7, 7)] = Free;
    board[(7, 4)] = Occupied(King, Black);
    board.black_king = coord(7, 4);
    board[(6, 4)] = Occupied(Rook, Black);
    recompute_score(&mut board);
    board.sync_hash(White);

    let mut ctx = test_ctx();
    let history = HashMap::new();
    let mut params = SearchParams::new(&mut ctx, &history, 0);
    let (mv, _) = find_best_move(&mut board, White, 2, i32::MIN, i32::MAX, &mut params);
    let mv = mv.expect("should find a move");
    let is_bad_capture = mv.origin == coord(3, 3) && mv.dest == coord(3, 4);
    assert!(
        !is_bad_capture,
        "bot should not take a pawn defended by a rook"
    );
}

// Kink stucked in a8 with white queen in b6
#[test]
fn test_stalemate_returns_zero() {
    let mut board = empty_board(coord(5, 0), coord(7, 0));
    board[(5, 1)] = Occupied(Queen, White);
    recompute_score(&mut board);
    board.sync_hash(Black);

    let mut ctx = test_ctx();
    let history = HashMap::new();
    let mut params = SearchParams::new(&mut ctx, &history, 0);
    let score = minimax(&mut board, 1, Black, -1_000_000, 1_000_000, 0, &mut params);
    assert_eq!(
        score, -50,
        "stalemate caused by winning side should return contempt penalty"
    );
}

#[test]
fn test_checkmate_returns_mate_score() {
    let mut board = empty_board(coord(5, 5), coord(7, 7));
    board[(6, 6)] = Occupied(Queen, White);
    board[(0, 7)] = Occupied(Rook, White);
    recompute_score(&mut board);
    board.sync_hash(Black);

    let mut ctx = test_ctx();
    let history = HashMap::new();
    let mut params = SearchParams::new(&mut ctx, &history, 0);
    let score = minimax(&mut board, 1, Black, -1_000_000, 1_000_000, 0, &mut params);
    assert!(
        score > 100_000,
        "checkmate should return a large positive score (white wins), got {score}"
    );
}

fn legal_moves_uci(fen: &str) -> Vec<String> {
    let mut fen_info = Board::board_from_fen(fen);
    let mut list = MoveList::new();
    generate_moves(
        &mut fen_info.board,
        &fen_info.active_color,
        &mut list,
        false,
    );
    let mut uci: Vec<String> = list.moves[..list.count]
        .iter()
        .map(|m| m.to_uci())
        .collect();
    uci.sort();
    uci
}

#[test]
fn castle_not_generated_while_in_check() {
    let cases = [
        (
            "r1bqk2r/p4pp1/1p1p4/2pP3B/1b6/2NnP3/PP1B1PPP/R2QK2R w KQkq - 1 5",
            "e1g1",
            vec!["e1e2", "e1f1"],
        ),
        (
            "r3k2r/p6p/2p1p2B/b1ppPn1Q/8/2P5/P7/4N2K b kq - 4 19",
            "e8c8",
            vec!["e8d7", "e8d8", "e8e7"],
        ),
        (
            "r2qk2r/pp2bppp/3p1nn1/2pPp3/Q1P1P3/2N2NPb/PP2BP1P/1RB1K2R b Kkq - 2 2",
            "e8g8",
            vec!["b7b5", "d8d7", "e8f8", "f6d7", "h3d7"],
        ),
        (
            "r3k1r1/5p2/pqp2np1/4Q2p/2p1P3/2N2P1N/PP4PP/2K2R2 b q - 0 13",
            "e8c8",
            vec!["e8d7", "e8d8", "e8f8"],
        ),
    ];
    for (fen, castle, expected) in cases {
        let got = legal_moves_uci(fen);
        assert!(
            !got.contains(&castle.to_string()),
            "{fen} generated {castle}"
        );
        assert_eq!(got, expected, "{fen}");
    }
}

#[test]
fn aborted_search_keeps_a_move_and_leaves_tt_untouched() {
    let mut board = Board::init_board();
    let history = HashMap::new();

    let mut ctx = test_ctx();
    ctx.stats.max_nodes = 1;
    let aborted_move = {
        let mut params = SearchParams::new(&mut ctx, &history, 0);
        find_best_move(&mut board, White, 6, i32::MIN, i32::MAX, &mut params).0
    };
    assert!(ctx.stats.aborted, "the search should have been aborted");
    assert!(
        aborted_move.is_some(),
        "an aborted search must still return a legal move"
    );
    assert_eq!(
        ctx.stats.tt_stores, 0,
        "an aborted search must not write to the transposition table"
    );

    let mut ctx = test_ctx();
    let full_move = {
        let mut params = SearchParams::new(&mut ctx, &history, 0);
        find_best_move(&mut board, White, 6, i32::MIN, i32::MAX, &mut params).0
    };
    assert!(!ctx.stats.aborted);
    assert!(full_move.is_some());
    assert!(
        ctx.stats.tt_stores > 0,
        "a complete search is expected to fill the transposition table"
    );
}

#[test]
fn a_past_deadline_aborts_the_search_and_keeps_a_move() {
    let mut board = Board::init_board();
    let history = HashMap::new();

    let mut ctx = test_ctx();
    ctx.stats.deadline = 1.0;
    let aborted_move = {
        let mut params = SearchParams::new(&mut ctx, &history, 0);
        find_best_move(&mut board, White, 8, i32::MIN, i32::MAX, &mut params).0
    };
    let aborted_nodes = ctx.stats.cumulative_nodes;
    assert!(
        ctx.stats.aborted,
        "an expired deadline should abort the search"
    );
    assert!(
        aborted_move.is_some(),
        "a search stopped by the deadline must still return a legal move"
    );

    let mut ctx = test_ctx();
    let full_move = {
        let mut params = SearchParams::new(&mut ctx, &history, 0);
        find_best_move(&mut board, White, 8, i32::MIN, i32::MAX, &mut params).0
    };
    let full_nodes = ctx.stats.cumulative_nodes;
    assert!(!ctx.stats.aborted);
    assert!(full_move.is_some());
    assert!(
        aborted_nodes * 10 < full_nodes,
        "deadline stopped after {aborted_nodes} nodes, full search took {full_nodes}"
    );
}

#[test]
fn iterative_deepening_arms_the_deadline_from_the_timeout() {
    let mut board = Board::init_board();
    let history = HashMap::new();
    let mut ctx = test_ctx();
    let mut reached = 0;

    {
        let mut params = SearchParams::new(&mut ctx, &history, 0);
        iterative_deepening(
            &mut board,
            White,
            2,
            &mut reached,
            Budget::UNLIMITED,
            &mut params,
        );
    }
    assert_eq!(
        ctx.stats.deadline, 0.0,
        "a search without timeout must stay unbounded"
    );

    {
        let mut params = SearchParams::new(&mut ctx, &history, 0);
        iterative_deepening(
            &mut board,
            White,
            2,
            &mut reached,
            Budget::fixed(50.0),
            &mut params,
        );
    }
    assert!(
        ctx.stats.deadline > 0.0,
        "a timeout must arm the in-search deadline"
    );
}

fn tc(remaining_ms: f64, increment_ms: f64, moves_to_go: Option<u32>) -> TimeControl {
    TimeControl {
        remaining_ms,
        increment_ms,
        moves_to_go,
    }
}

#[test]
fn plan_never_commits_more_than_the_clock_holds() {
    let overheads = [0.0, 50.0, 100.0, 300.0];
    let remainings = [200.0, 1_000.0, 10_000.0, 60_000.0, 600_000.0];
    let increments = [0.0, 100.0, 600.0, 1_000.0];
    let to_go = [None, Some(1), Some(5), Some(40)];

    for &overhead in &overheads {
        for &remaining in &remainings {
            if remaining <= overhead {
                continue;
            }
            for &inc in &increments {
                for &mtg in &to_go {
                    let Budget { soft_ms, hard_ms } = plan(&tc(remaining, inc, mtg), 30, overhead);
                    assert!(
                        hard_ms + overhead <= remaining,
                        "remaining={remaining} inc={inc} mtg={mtg:?} overhead={overhead} \
                         would commit {hard_ms} + {overhead} overhead"
                    );
                    assert!(soft_ms <= hard_ms, "soft {soft_ms} above hard {hard_ms}");
                    assert!(soft_ms > 0.0, "soft budget must stay positive");
                }
            }
        }
    }
}

#[test]
fn plan_reserves_the_move_overhead() {
    let control = tc(10_000.0, 100.0, None);
    let generous = plan(&control, 30, 0.0);
    let careful = plan(&control, 30, 500.0);
    assert!(
        careful.hard_ms < generous.hard_ms,
        "a larger overhead must shrink the budget"
    );
}

#[test]
fn plan_returns_the_minimum_for_a_single_legal_move() {
    let budget = plan(&tc(60_000.0, 1_000.0, None), 1, 100.0);
    assert_eq!(budget.soft_ms, budget.hard_ms);
    assert!(
        budget.hard_ms < 100.0,
        "a forced move must not consume a full budget, got {}",
        budget.hard_ms
    );
}

#[test]
fn moves_to_go_spends_faster_than_sudden_death() {
    let control = tc(60_000.0, 0.0, None);
    let sudden_death = plan(&control, 30, 100.0);
    let ten_left = plan(&tc(60_000.0, 0.0, Some(10)), 30, 100.0);
    assert!(
        ten_left.soft_ms > sudden_death.soft_ms,
        "a known move count must allow a larger share"
    );
}

#[test]
fn the_increment_raises_the_budget() {
    let without = plan(&tc(10_000.0, 0.0, None), 30, 100.0);
    let with = plan(&tc(10_000.0, 1_000.0, None), 30, 100.0);
    assert!(with.soft_ms > without.soft_ms);
}

#[test]
fn plan_survives_a_nearly_flagged_clock() {
    let budget = plan(&tc(120.0, 100.0, None), 30, 100.0);
    assert!(budget.soft_ms > 0.0);
    assert!(budget.hard_ms <= 20.0, "got {}", budget.hard_ms);
}

const POISON_SCORE: i32 = 123_456;

#[test]
fn quiescence_neither_reads_nor_evicts_a_main_search_entry() {
    let fen = Board::board_from_fen(KIWIPETE_FEN);
    let mut board = fen.board;
    recompute_score(&mut board);
    board.sync_hash(fen.active_color);

    let mut ctx = test_ctx();
    let idx = (board.hash as usize) & (TT_SIZE - 1);
    ctx.tt[idx] = TtEntry {
        key: board.hash,
        score: POISON_SCORE,
        depth: 5,
        generation: ctx.tt_generation,
        flag: TtFlag::Exact,
        best_move: None,
    };

    let score = quiescence_minimax(
        &mut board,
        -1_000_000,
        1_000_000,
        fen.active_color,
        &mut ctx,
        4,
        0,
    );

    assert_ne!(
        score, POISON_SCORE,
        "quiescence must not return a main search entry"
    );
    assert_eq!(
        ctx.tt[idx].depth, 5,
        "quiescence must not evict a main search entry"
    );
    assert_eq!(ctx.tt[idx].score, POISON_SCORE);
}

#[test]
fn quiescence_only_ever_writes_at_its_own_depth() {
    let fen = Board::board_from_fen(KIWIPETE_FEN);
    let mut board = fen.board;
    recompute_score(&mut board);
    board.sync_hash(fen.active_color);

    let mut ctx = test_ctx();
    quiescence_minimax(
        &mut board,
        -1_000_000,
        1_000_000,
        fen.active_color,
        &mut ctx,
        4,
        0,
    );

    let written = ctx.tt.iter().filter(|e| e.key != 0).count();
    assert!(written > 0, "quiescence is expected to fill some slots");
    assert!(
        ctx.tt.iter().filter(|e| e.key != 0).all(|e| e.depth == 0),
        "a quiescence entry must never claim a main search depth"
    );
}
