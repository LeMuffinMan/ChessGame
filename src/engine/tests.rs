use crate::Board;
use crate::board::CastleRights;
use crate::board::cell::Cell::{Free, Occupied};
use crate::board::cell::Color;
use crate::board::cell::Color::{Black, White};
use crate::board::cell::Coord;
use crate::board::cell::Piece::{King, Pawn, Queen, Rook};
use crate::board::moves::move_gen::generate_moves;
use crate::board::moves::move_structs::MoveList;
use crate::engine::evaluator::{evaluate, get_piece_value_at, non_pawn_raw};
use crate::engine::minimax::{find_best_move, minimax};
use crate::engine::search_context::{SearchContext, SearchParams};
use std::collections::HashMap;

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
    let moves = &mut move_list.moves[..move_list.count];

    let opponent = match color {
        White => Black,
        Black => White,
    };

    for i in 0..moves.len() {
        board.apply_move(&moves[i], color);
        res += perft(board, opponent, depth - 1);
        board.undo_move(moves[i], color);
    }
    res
}

#[test]
fn perft_d1() {
    let mut board = Board::init_board();
    assert_eq!(perft(&mut board, White, 1), 20);
}

#[test]
fn perft_d2() {
    let mut board = Board::init_board();
    assert_eq!(perft(&mut board, White, 2), 400);
}

#[test]
fn perft_d3() {
    let mut board = Board::init_board();
    assert_eq!(perft(&mut board, White, 3), 8902);
}
#[test]
fn perft_d4() {
    let mut board = Board::init_board();
    assert_eq!(perft(&mut board, White, 4), 197281);
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
