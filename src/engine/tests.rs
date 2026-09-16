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
use crate::engine::minimax::{find_best_move, iterative_deepening, minimax};
use crate::engine::search_context::{SearchContext, SearchParams};
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
        iterative_deepening(&mut board, White, 2, &mut reached, 0.0, &mut params);
    }
    assert_eq!(
        ctx.stats.deadline, 0.0,
        "a search without timeout must stay unbounded"
    );

    {
        let mut params = SearchParams::new(&mut ctx, &history, 0);
        iterative_deepening(&mut board, White, 2, &mut reached, 50.0, &mut params);
    }
    assert!(
        ctx.stats.deadline > 0.0,
        "a timeout must arm the in-search deadline"
    );
}
