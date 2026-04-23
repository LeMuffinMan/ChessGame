use crate::board::Board;
use crate::board::board::CastleRights;
use crate::board::cell::Cell::{Free, Occupied};
use crate::board::cell::Color::{Black, White};
use crate::board::cell::Coord;
use crate::board::cell::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::board::is_king_exposed::is_king_exposed;
use crate::board::move_gen::{generate_moves, Move, MoveList};

fn gen_moves(board: &mut Board, color: &crate::board::cell::Color) -> Vec<Move> {
    let mut list = MoveList::new();
    generate_moves(board, color, &mut list, false);
    list.moves[..list.count].to_vec()
}

fn board_core_eq(a: &Board, b: &Board) -> bool {
    a.grid == b.grid
        && a.en_passant == b.en_passant
        && a.white_castle == b.white_castle
        && a.black_castle == b.black_castle
        && a.white_king == b.white_king
        && a.black_king == b.black_king
    //Je devrais ajouter plus de comparaisons ?
}

fn coord(row: u8, col: u8) -> Coord {
    Coord { row, col }
}

// apply / undo tests

#[test]
fn test_apply_undo_regular() {
    let initial = Board::init_board();
    let mut board = Board::init_board();
    let from = coord(1, 4); // e2
    let to = coord(3, 4); // e4
    let m = board.build_move(from, to, White);
    board.apply_move(&m, White);
    board.undo_move(m, White);
    assert!(board_core_eq(&board, &initial));
}

#[test]
fn test_apply_undo_capture() {
    let mut board = Board::init_board();
    // Place a black pawn in d3 so the white pawn in e2 can capture
    board.grid[2][3] = Occupied(Pawn, Black);
    let snapshot = board.clone();
    let from = coord(1, 4); // e2
    let to = coord(2, 3); // d3 capture
    let m = board.build_move(from, to, White);
    board.apply_move(&m, White);
    //We want to restore the captured piece
    board.undo_move(m, White);
    assert!(board_core_eq(&board, &snapshot));
}

#[test]
fn test_apply_undo_en_passant() {
    let mut board = Board::init_board();
    // white pawn in e5 (4,4), black pawn in d5 (4,3) just moved 2 cells as first move
    board.grid[1][4] = Free; // free e2
    board.grid[4][4] = Occupied(Pawn, White); // e5
    board.grid[4][3] = Occupied(Pawn, Black); // d5
    board.en_passant = Some(coord(5, 3)); // d6 — en passant target
    let snapshot = board.clone();
    let from = coord(4, 4);
    let to = coord(5, 3); // d6
    let m = board.build_move(from, to, White);
    assert_eq!(m.move_type, crate::board::move_gen::MoveType::EnPassant);
    board.apply_move(&m, White);
    assert_eq!(board.grid[4][3], Free); // black pawn capture
    assert_eq!(board.grid[5][3], Occupied(Pawn, White)); // white pawn in d6
    board.undo_move(m, White);
    assert!(board_core_eq(&board, &snapshot));
}

#[test]
fn test_apply_undo_castle_right() {
    let mut board = Board::init_board();
    // Free f1 and g1
    board.grid[0][5] = Free;
    board.grid[0][6] = Free;
    let snapshot = board.clone();
    let from = coord(0, 4); // e1
    let to = coord(0, 6); // g1
    let m = board.build_move(from, to, White);
    board.apply_move(&m, White);
    assert_eq!(board.grid[0][6], Occupied(King, White)); // king in g1
    assert_eq!(board.grid[0][5], Occupied(Rook, White)); // rook in f1
    assert_eq!(board.grid[0][4], Free);
    assert_eq!(board.grid[0][7], Free);
    assert_eq!(board.white_king, coord(0, 6));
    board.undo_move(m, White);
    assert!(board_core_eq(&board, &snapshot));
}

#[test]
fn test_apply_undo_castle_left() {
    let mut board = Board::init_board();
    // Empty b1 c1 d1
    board.grid[0][1] = Free;
    board.grid[0][2] = Free;
    board.grid[0][3] = Free;
    let snapshot = board.clone();
    let from = coord(0, 4); // e1
    let to = coord(0, 2); // c1
    let m = board.build_move(from, to, White);
    board.apply_move(&m, White);
    assert_eq!(board.grid[0][2], Occupied(King, White)); // king in c1
    assert_eq!(board.grid[0][3], Occupied(Rook, White)); // rook in d1
    assert_eq!(board.grid[0][4], Free);
    assert_eq!(board.grid[0][0], Free);
    assert_eq!(board.white_king, coord(0, 2));
    board.undo_move(m, White);
    assert!(board_core_eq(&board, &snapshot));
}

//Is king exposed

fn empty_board_with_kings() -> Board {
    let mut board = Board::init_board();
    for r in 0..8usize {
        for c in 0..8usize {
            board.grid[r][c] = Free;
        }
    }
    board.grid[0][4] = Occupied(King, White);
    board.grid[7][4] = Occupied(King, Black);
    board.white_king = coord(0, 4);
    board.black_king = coord(7, 4);
    board.white_castle = CastleRights { long: false, short: false };
    board.black_castle = CastleRights { long: false, short: false };
    board
}

#[test]
fn test_king_not_exposed_initial() {
    let board = Board::init_board();
    assert!(!is_king_exposed(&board, &White));
    assert!(!is_king_exposed(&board, &Black));
}

#[test]
fn test_king_exposed_by_rook() {
    let mut board = empty_board_with_kings();
    board.grid[7][4] = Free;
    board.grid[7][0] = Occupied(King, Black);
    board.black_king = coord(7, 0);
    board.grid[4][4] = Occupied(Rook, Black);
    assert!(is_king_exposed(&board, &White));
}

#[test]
fn test_king_exposed_by_bishop() {
    let mut board = empty_board_with_kings();
    board.grid[7][4] = Free;
    board.grid[7][0] = Occupied(King, Black);
    board.black_king = coord(7, 0);
    board.grid[1][3] = Occupied(Bishop, Black);
    assert!(is_king_exposed(&board, &White));
}

#[test]
fn test_king_exposed_by_knight() {
    let mut board = empty_board_with_kings();
    board.grid[7][4] = Free;
    board.grid[7][0] = Occupied(King, Black);
    board.black_king = coord(7, 0);
    board.grid[2][3] = Occupied(Knight, Black);
    assert!(is_king_exposed(&board, &White));
}

#[test]
fn test_king_exposed_by_pawn_white() {
    let mut board = empty_board_with_kings();
    board.grid[7][4] = Free;
    board.grid[7][0] = Occupied(King, Black);
    board.black_king = coord(7, 0);
    board.grid[1][3] = Occupied(Pawn, Black);
    assert!(is_king_exposed(&board, &White));
}

#[test]
fn test_king_not_exposed_by_pawn_wrong_direction() {
    let mut board = empty_board_with_kings();
    board.grid[7][4] = Free;
    board.grid[7][0] = Occupied(King, Black);
    board.black_king = coord(7, 0);
    board.grid[1][3] = Occupied(Pawn, White);
    assert!(!is_king_exposed(&board, &White));
}

#[test]
fn test_king_not_exposed_blocked_by_ally() {
    let mut board = empty_board_with_kings();
    board.grid[7][4] = Free;
    board.grid[7][0] = Occupied(King, Black);
    board.black_king = coord(7, 0);
    board.grid[4][4] = Occupied(Rook, Black);
    board.grid[2][4] = Occupied(Pawn, White);
    assert!(!is_king_exposed(&board, &White));
}

// generate_moves — régression

#[test]
fn test_generate_moves_initial_white() {
    let mut board = Board::init_board();
    assert_eq!(gen_moves(&mut board, &White).len(), 20);
}

#[test]
fn test_generate_moves_initial_black() {
    let mut board = Board::init_board();
    assert_eq!(gen_moves(&mut board, &Black).len(), 20);
}

#[test]
fn test_generate_moves_after_e4_black() {
    let mut board = Board::init_board();
    let m = board.build_move(coord(1, 4), coord(3, 4), White);
    board.apply_move(&m, White);
    assert_eq!(gen_moves(&mut board, &Black).len(), 20);
}

#[test]
fn test_generate_moves_stalemate() {
    // pat classique : roi noir en a8, dame blanche en c7, roi blanc en c6
    let mut board = Board::init_board();
    for r in 0..8usize {
        for c in 0..8usize {
            board.grid[r][c] = Free;
        }
    }
    board.grid[7][0] = Occupied(King, Black);
    board.grid[6][2] = Occupied(Queen, White);
    board.grid[5][2] = Occupied(King, White);
    board.white_king = coord(5, 2);
    board.black_king = coord(7, 0);
    board.white_castle = CastleRights { long: false, short: false };
    board.black_castle = CastleRights { long: false, short: false };
    let moves = gen_moves(&mut board, &Black);
    assert!(moves.is_empty());
    assert!(!is_king_exposed(&board, &Black));
}

// generate_moves — castle

fn has_move(board: &mut Board, color: &crate::board::cell::Color, from: Coord, to: Coord) -> bool {
    gen_moves(board, color)
        .iter()
        .any(|m| m.origin == from && m.dest == to)
}

#[test]
fn test_castle_right_in_legal_moves() {
    let mut board = Board::init_board();
    board.grid[0][5] = Free;
    board.grid[0][6] = Free;
    assert!(has_move(&mut board, &White, coord(0, 4), coord(0, 6)));
}

#[test]
fn test_castle_left_in_legal_moves() {
    let mut board = Board::init_board();
    board.grid[0][1] = Free;
    board.grid[0][2] = Free;
    board.grid[0][3] = Free;
    assert!(has_move(&mut board, &White, coord(0, 4), coord(0, 2)));
}

#[test]
fn test_castle_denied_through_check() {
    let mut board = Board::init_board();
    board.grid[0][5] = Free;
    board.grid[0][6] = Free;
    board.grid[1][5] = Occupied(Rook, Black); // tour noire menace f1
    assert!(!has_move(&mut board, &White, coord(0, 4), coord(0, 6)));
}

#[test]
fn test_castle_denied_king_in_check() {
    let mut board = Board::init_board();
    board.grid[0][5] = Free;
    board.grid[0][6] = Free;
    board.check = Some(coord(0, 4));
    assert!(!has_move(&mut board, &White, coord(0, 4), coord(0, 6)));
}

#[test]
fn test_castle_denied_rights_lost() {
    let mut board = Board::init_board();
    board.grid[0][5] = Free;
    board.grid[0][6] = Free;
    board.white_castle = CastleRights {
        long: false,
        short: false,
    };
    assert!(!has_move(&mut board, &White, coord(0, 4), coord(0, 6)));
}

// generate_moves — comportements spécifiques (bugs fixés)

#[test]
fn test_pawn_cannot_move_straight_to_occupied() {
    let mut board = Board::init_board();
    // pion noir en e3 bloque le pion blanc en e2
    board.grid[2][4] = Occupied(Pawn, Black);
    let moves = gen_moves(&mut board, &White);
    assert!(
        !moves
            .iter()
            .any(|m| m.origin == coord(1, 4) && m.dest == coord(2, 4))
    );
    assert!(
        !moves
            .iter()
            .any(|m| m.origin == coord(1, 4) && m.dest == coord(3, 4))
    );
}

#[test]
fn test_pawn_cannot_move_diagonal_to_empty() {
    let mut board = Board::init_board();
    // case d3 est vide : le pion en e2 ne doit pas pouvoir aller en d3
    board.grid[2][3] = Free;
    let moves = gen_moves(&mut board, &White);
    assert!(
        !moves
            .iter()
            .any(|m| m.origin == coord(1, 4) && m.dest == coord(2, 3))
    );
}

#[test]
fn test_pawn_double_push_blocked_by_intermediate() {
    let mut board = Board::init_board();
    // pion blanc en e3 bloque la double poussée du pion en e2
    board.grid[2][4] = Occupied(Pawn, White);
    board.grid[1][4] = Free;
    let moves = gen_moves(&mut board, &White);
    assert!(
        !moves
            .iter()
            .any(|m| m.origin == coord(1, 4) && m.dest == coord(3, 4))
    );
}

#[test]
fn test_rook_stops_after_capture() {
    let mut board = Board::init_board();
    // tour blanche en a1, pion noir en a4 — la tour doit s'arrêter à a4
    for r in 0..8usize {
        for c in 0..8usize {
            board.grid[r][c] = Free;
        }
    }
    board.grid[0][4] = Occupied(King, White);
    board.grid[7][4] = Occupied(King, Black);
    board.white_king = coord(0, 4);
    board.black_king = coord(7, 4);
    board.grid[0][0] = Occupied(Rook, White);
    board.grid[3][0] = Occupied(Pawn, Black);
    let moves = gen_moves(&mut board, &White);
    // la tour peut aller en a2, a3, a4 (capture) mais PAS a5+
    assert!(
        moves
            .iter()
            .any(|m| m.origin == coord(0, 0) && m.dest == coord(3, 0))
    );
    assert!(
        !moves
            .iter()
            .any(|m| m.origin == coord(0, 0) && m.dest == coord(4, 0))
    );
}

#[test]
fn test_castle_rights_revoked_on_king_move() {
    let mut board = Board::init_board();
    board.grid[0][5] = Free;
    board.grid[0][6] = Free;
    let m = board.build_move(coord(0, 4), coord(0, 5), White);
    board.apply_move(&m, White);
    assert_eq!(
        board.white_castle,
        CastleRights {
            long: false,
            short: false
        }
    );
}

#[test]
fn test_castle_rights_revoked_on_rook_move() {
    let mut board = Board::init_board();
    board.grid[1][7] = Free; // free the pawn in front of h1 rook
    board.grid[2][7] = Free;
    let m = board.build_move(coord(0, 7), coord(2, 7), White);
    board.apply_move(&m, White);
    assert_eq!(
        board.white_castle,
        CastleRights {
            long: true,
            short: false
        }
    );
}

#[test]
fn test_castle_rights_restored_on_undo() {
    let mut board = Board::init_board();
    board.grid[0][5] = Free;
    board.grid[0][6] = Free;
    let from = coord(0, 4);
    let to = coord(0, 6);
    let m = board.build_move(from, to, White);
    board.apply_move(&m, White);
    assert_eq!(
        board.white_castle,
        CastleRights {
            long: false,
            short: false
        }
    );
    board.undo_move(m, White);
    assert_eq!(
        board.white_castle,
        CastleRights {
            long: true,
            short: true
        }
    );
}
