use crate::Board;
use crate::board::board::CastleRights;
use crate::board::cell::Cell::{Free, Occupied};
use crate::board::cell::Color::{Black, White};
use crate::board::cell::Coord;
use crate::board::cell::Piece::{King, Pawn, Queen, Rook};
use crate::engine::evaluator::{Evaluator, BISHOP_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE, KING_VALUE};
use crate::engine::minimax::{find_best_move, minimax};
use crate::engine::search_stats::SearchStats;

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
    board.white_castle = CastleRights { long: false, short: false };
    board.black_castle = CastleRights { long: false, short: false };
    board
}

fn test_stats() -> SearchStats {
    SearchStats {
        nodes: 0,
        bot_time_thinking: 0.0,
        cutoffs: 0,
        nps: 0.0,
        killer_moves: [[None; 2]; 64],
    }
}

// Évaluateur matériel pur (sans PST) pour les tests — recompute depuis le board
// Nécessaire car board.score n'est pas maintenu sur les boards construits manuellement
struct MaterialEvaluator;
impl Evaluator for MaterialEvaluator {
    fn evaluate(&self, board: &Board) -> i32 {
        let mut score = 0;
        for x in 0..8 {
            for y in 0..8 {
                if let Occupied(piece, color) = board.grid[x][y] {
                    let v = match piece {
                        crate::board::cell::Piece::Pawn   => PAWN_VALUE,
                        crate::board::cell::Piece::Knight => KNIGHT_VALUE,
                        crate::board::cell::Piece::Bishop => BISHOP_VALUE,
                        crate::board::cell::Piece::Rook   => ROOK_VALUE,
                        crate::board::cell::Piece::Queen  => QUEEN_VALUE,
                        crate::board::cell::Piece::King   => KING_VALUE,
                    };
                    score += if color == White { v } else { -v };
                }
            }
        }
        score
    }
}

// --- evaluator ---

#[test]
fn test_evaluate_equal_material() {
    let board = empty_board(coord(0, 0), coord(7, 7));
    let eval = MaterialEvaluator;
    // Two kings cancel out → 0
    assert_eq!(eval.evaluate(&board), 0);
}

#[test]
fn test_evaluate_white_queen_advantage() {
    let mut board = empty_board(coord(0, 0), coord(7, 7));
    board.grid[3][3] = Occupied(Queen, White);
    let eval = MaterialEvaluator;
    assert_eq!(eval.evaluate(&board), 900);
}

// --- minimax ---

// Tour blanche d4, Dame noire d5 non défendue — le bot doit capturer
#[test]
fn test_captures_free_queen() {
    let mut board = empty_board(coord(0, 0), coord(7, 7));
    board.grid[3][3] = Occupied(Rook, White);
    board.grid[4][3] = Occupied(Queen, Black);

    let mv = find_best_move(&mut board, White, &MaterialEvaluator, 2, &mut test_stats())
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

    let mv = find_best_move(&mut board, White, &MaterialEvaluator, 2, &mut test_stats())
        .expect("should find a move");
    let is_bad_capture = mv.origin == coord(3, 3) && mv.dest == coord(3, 4);
    assert!(!is_bad_capture, "bot should not take a pawn defended by a rook");
}

// Pat classique : roi noir coincé en a8, dame blanche b6
#[test]
fn test_stalemate_returns_zero() {
    let mut board = empty_board(coord(5, 0), coord(7, 0));
    board.grid[5][1] = Occupied(Queen, White);

    let score = minimax(&mut board, 1, Black, &MaterialEvaluator, -1_000_000, 1_000_000, &mut test_stats());
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

    let score = minimax(&mut board, 1, Black, &MaterialEvaluator, -1_000_000, 1_000_000, &mut test_stats());
    // Black est maté → White gagne → score large positif (convention board.score : positif = avantage blanc)
    assert!(score > 100_000, "checkmate should return a large positive score (white wins), got {score}");
}
