use crate::Board;
use crate::board::board::CastleRights;
use crate::board::cell::Cell::{Free, Occupied};
use crate::board::cell::Color;
use crate::board::cell::Color::{Black, White};
use crate::board::cell::Coord;
use crate::board::cell::Piece::{King, Pawn, Queen, Rook};
use crate::board::moves::move_gen::generate_moves;
use crate::board::moves::move_structs::MoveList;
use crate::engine::evaluator::{evaluate, get_piece_value_at};
use crate::engine::minimax::{find_best_move, minimax};
use crate::engine::search_context::SearchContext;

fn coord(row: u8, col: u8) -> Coord {
    Coord { row, col }
}

fn empty_board(white_king: Coord, black_king: Coord) -> Board {
    let mut board = Board::init_board();
    for r in 0..8usize {
        for c in 0..8usize {
            board.grid[r][c] = Free;
        }
    }
    board.grid[white_king.row as usize][white_king.col as usize] = Occupied(King, White);
    board.grid[black_king.row as usize][black_king.col as usize] = Occupied(King, Black);
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

// board.score n'est pas maintenu sur les boards construits manuellement → recalcul nécessaire
fn recompute_score(board: &mut Board) {
    board.score = 0;
    for r in 0..8usize {
        for c in 0..8usize {
            if let Occupied(piece, color) = board.grid[r][c] {
                board.score += get_piece_value_at(
                    &piece,
                    &color,
                    &Coord { row: r as u8, col: c as u8 },
                );
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

// --- evaluator ---

#[test]
fn test_evaluate_equal_material() {
    let mut board = empty_board(coord(0, 0), coord(7, 7));
    recompute_score(&mut board);
    // Two kings cancel out → 0
    assert_eq!(evaluate(&board), 0);
}

#[test]
fn test_evaluate_white_queen_advantage() {
    let mut board = empty_board(coord(0, 0), coord(7, 7));
    board.grid[3][3] = Occupied(Queen, White);
    recompute_score(&mut board);
    // 900 (material) + 5 (QUEEN_PST[35]) = 905, kings cancel out
    assert_eq!(evaluate(&board), 905);
}

// --- minimax ---

// Tour blanche d4, Dame noire d5 non défendue — le bot doit capturer
#[test]
fn test_captures_free_queen() {
    let mut board = empty_board(coord(0, 0), coord(7, 7));
    board.grid[3][3] = Occupied(Rook, White);
    board.grid[4][3] = Occupied(Queen, Black);
    recompute_score(&mut board);
    board.sync_hash(White);

    let mv = find_best_move(&mut board, White, 2, &mut test_ctx())
        .expect("should find a move");
    assert_eq!(mv.origin, coord(3, 3));
    assert_eq!(mv.dest, coord(4, 3));
}

// Tour blanche d4, Pion noir e4, Tour noire g7 défend e4 — prendre le pion perd la tour
#[test]
fn test_avoids_losing_rook_depth2() {
    let mut board = empty_board(coord(0, 0), coord(7, 7));
    board.grid[3][3] = Occupied(Rook, White);
    board.grid[3][4] = Occupied(Pawn, Black);
    board.grid[7][7] = Free;
    board.grid[7][4] = Occupied(King, Black);
    board.black_king = coord(7, 4);
    board.grid[6][4] = Occupied(Rook, Black);
    recompute_score(&mut board);
    board.sync_hash(White);

    let mv = find_best_move(&mut board, White, 2, &mut test_ctx())
        .expect("should find a move");
    let is_bad_capture = mv.origin == coord(3, 3) && mv.dest == coord(3, 4);
    assert!(
        !is_bad_capture,
        "bot should not take a pawn defended by a rook"
    );
}

// Pat classique : roi noir coincé en a8, dame blanche b6
#[test]
fn test_stalemate_returns_zero() {
    let mut board = empty_board(coord(5, 0), coord(7, 0));
    board.grid[5][1] = Occupied(Queen, White);
    recompute_score(&mut board);
    board.sync_hash(Black);

    let mut ctx = test_ctx();
    let score = minimax(
        &mut board,
        1,
        Black,
        -1_000_000,
        1_000_000,
        &mut ctx,
        true,
    );
    assert_eq!(score, 0, "stalemate should return 0");
}

// Mat classique : roi noir en h8, dame blanche en g7, tour blanche en h1
#[test]
fn test_checkmate_returns_mate_score() {
    let mut board = empty_board(coord(5, 5), coord(7, 7));
    board.grid[6][6] = Occupied(Queen, White);
    board.grid[0][7] = Occupied(Rook, White);
    // Le roi noir est en échec (dame diagonale g7→h8) — setter manuellement
    // car board.check n'est pas maintenu sur les boards construits manuellement
    board.check = Some(coord(7, 7));
    recompute_score(&mut board);
    board.sync_hash(Black);

    let mut ctx = test_ctx();
    let score = minimax(
        &mut board,
        1,
        Black,
        -1_000_000,
        1_000_000,
        &mut ctx,
        true,
    );
    // Black est maté → White gagne → score large positif (convention board.score : positif = avantage blanc)
    assert!(
        score > 100_000,
        "checkmate should return a large positive score (white wins), got {score}"
    );
}
