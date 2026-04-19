use crate::Board;
use crate::board::board::CastleRights;
use crate::board::cell::Cell::{Free, Occupied};
use crate::board::cell::Color::{Black, White};
use crate::board::cell::Coord;
use crate::board::cell::Piece::{King, Pawn, Queen, Rook};
use crate::engine::evaluator::{Evaluator, MaterialEvaluation};
use crate::engine::minimax::{find_best_move, minimax};

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

// --- evaluator ---

#[test]
fn test_evaluate_equal_material() {
    let board = empty_board(coord(0, 0), coord(7, 7));
    let eval = MaterialEvaluation;
    assert_eq!(eval.evaluate(&board, White), 0);
    assert_eq!(eval.evaluate(&board, Black), 0);
}

#[test]
fn test_evaluate_white_queen_advantage() {
    let mut board = empty_board(coord(0, 0), coord(7, 7));
    board.grid[3][3] = Occupied(Queen, White);
    let eval = MaterialEvaluation;
    assert_eq!(eval.evaluate(&board, White), 900);
    assert_eq!(eval.evaluate(&board, Black), -900);
}

// --- minimax ---

// Tour blanche d4, Dame noire d5 non défendue — le bot doit capturer
#[test]
fn test_captures_free_queen() {
    let mut board = empty_board(coord(0, 0), coord(7, 7));
    board.grid[3][3] = Occupied(Rook, White);
    board.grid[4][3] = Occupied(Queen, Black);

    let mv = find_best_move(&mut board, White, 2).expect("should find a move");
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

    let mv = find_best_move(&mut board, White, 2).expect("should find a move");
    let is_bad_capture = mv.origin == coord(3, 3) && mv.dest == coord(3, 4);
    assert!(
        !is_bad_capture,
        "bot should not take a pawn defended by a rook"
    );
}

// Pat classique : roi noir coincé en a8, dame blanche b6
#[test]
fn test_stalemate_returns_zero() {
    // Roi blanc a6 (5,0), Dame blanche b6 (5,1), Roi noir a8 (7,0)
    // C'est le tour du noir — aucun coup légal, pas en échec → pat
    let mut board = empty_board(coord(5, 0), coord(7, 0));
    board.grid[5][1] = Occupied(Queen, White);

    let score = minimax(&mut board, 1, false, Black, &MaterialEvaluation);
    assert_eq!(score, 0, "stalemate should return 0");
}
