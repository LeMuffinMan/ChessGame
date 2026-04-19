use crate::Coord;

use crate::board::Board;
use crate::board::board::CastleRights;
use crate::board::cell::Cell;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::is_king_exposed::is_king_exposed;

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    pub origin: Coord,
    pub dest: Coord,
    pub capture: Cell,
    pub en_passant: Option<Coord>,
    pub check: Option<Coord>,
    pub white_castle: CastleRights,
    pub black_castle: CastleRights,
    pub move_type: MoveType,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MoveType {
    Regular,
    EnPassant,
    Castle(CastleSide),
    Promotion(Piece),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CastleSide {
    Left,
    Right,
}

pub fn generate_moves(board: &mut Board, active_player: &Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for x in 0..8 {
        for y in 0..8 {
            if board.grid[x][y].is_color(active_player) {
                let origin = Coord {
                    row: x as u8,
                    col: y as u8,
                };
                if let Some(piece) = board.get(&origin).get_piece() {
                    match piece {
                        Pawn => {
                            let vec = generate_pawn_moves(&origin, active_player, board);
                            moves.extend(vec);
                        }
                        Rook => {
                            let vec = generate_rook_moves(&origin, active_player, board);
                            moves.extend(vec);
                        }
                        Knight => {
                            let vec = generate_knight_moves(&origin, active_player, board);
                            moves.extend(vec);
                        }
                        Bishop => {
                            let vec = generate_bishop_moves(&origin, active_player, board);
                            moves.extend(vec);
                        }
                        Queen => {
                            let vec = generate_queen_moves(&origin, active_player, board);
                            moves.extend(vec);
                        }
                        King => {
                            let vec = generate_king_moves(&origin, active_player, board);
                            moves.extend(vec);
                        }
                    }
                }
            }
        }
    }
    moves
}

pub fn generate_pawn_moves(origin: &Coord, active_player: &Color, board: &mut Board) -> Vec<Move> {
    let dir: i8 = if *active_player == White { 1 } else { -1 };
    let last_rank: u8 = if *active_player == White { 7 } else { 0 };
    let mut ret: Vec<Move> = Vec::new();

    // diagonals — enemy piece or en passant
    // board.en_passant stores the enemy pawn's position (e.g. e5), not the capture square (e.g. e6)
    // so we derive the capture square: ep.row + dir (one step in our direction)
    for dc in [1i8, -1] {
        if let Some(dest) = Board::checked_coord(origin.row as i8 + dir, origin.col as i8 + dc) {
            let target = board.get(&dest);
            let is_enemy = target.get_piece().is_some() && !target.is_color(active_player);
            let is_en_passant = match board.en_passant {
                None => false,
                Some(ep) => dest.col == ep.col && dest.row as i8 == ep.row as i8 + dir,
            };
            if is_enemy || is_en_passant {
                push_pawn_dest(origin, &dest, active_player, board, last_rank, &mut ret);
            }
        }
    }

    // forward — only on empty square
    if let Some(dest) = Board::checked_coord(origin.row as i8 + dir, origin.col as i8)
        && board.get(&dest) == Cell::Free
    {
        push_pawn_dest(origin, &dest, active_player, board, last_rank, &mut ret);
    }

    // double push — starting rank, both squares empty
    let initial_row = match active_player {
        White => origin.row == 1,
        Black => origin.row == 6,
    };
    if initial_row {
        if let Some(mid) = Board::checked_coord(origin.row as i8 + dir, origin.col as i8)
            && let Some(dest) = Board::checked_coord(origin.row as i8 + dir + dir, origin.col as i8)
            && board.get(&mid) == Cell::Free
            && board.get(&dest) == Cell::Free
        {
            if let Some(m) = board.check_move(origin, &dest, active_player) {
                ret.push(m);
            }
        }
    }

    ret
}

// If a pawn reach the last rank, we want to generate 4 moves for the 4 possible promotions
fn push_pawn_dest(
    origin: &Coord,
    dest: &Coord,
    active_player: &Color,
    board: &mut Board,
    last_rank: u8,
    ret: &mut Vec<Move>,
) {
    if dest.row == last_rank {
        let base = Move {
            origin: *origin,
            dest: *dest,
            capture: board.get(dest),
            en_passant: board.en_passant,
            check: board.check,
            white_castle: board.white_castle,
            black_castle: board.black_castle,
            move_type: MoveType::Promotion(Queen),
        };
        board.apply_move(&base, *active_player);
        let exposed = is_king_exposed(board, active_player);
        board.undo_move(base, *active_player);
        if !exposed {
            for promoted in [Queen, Rook, Bishop, Knight] {
                ret.push(Move {
                    move_type: MoveType::Promotion(promoted),
                    ..base
                });
            }
        }
    } else if let Some(m) = board.check_move(origin, dest, active_player) {
        ret.push(m);
    }
}

pub fn generate_rook_moves(origin: &Coord, active_player: &Color, board: &mut Board) -> Vec<Move> {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut ret: Vec<Move> = Vec::new();

    for (dr, dc) in directions {
        let mut r = origin.row as i8 + dr;
        let mut c = origin.col as i8 + dc;

        while let Some(dest) = Board::checked_coord(r, c) {
            let target = board.get(&dest);

            if target.is_color(active_player) {
                break;
            }
            let is_capture = target.get_piece().is_some();
            if let Some(m) = board.check_move(&origin, &dest, active_player) {
                ret.push(m);
            }
            if is_capture {
                break;
            }
            r += dr;
            c += dc;
        }
    }
    ret
}

pub fn generate_knight_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
) -> Vec<Move> {
    let cells: [(i8, i8); 8] = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];
    let mut ret = Vec::new();

    for (dr, dc) in cells {
        let new_row = origin.row as i8 + dr;
        let new_col = origin.col as i8 + dc;
        if let Some(dest) = Board::checked_coord(new_row, new_col)
            && let Some(m) = board.check_move(&origin, &dest, active_player)
        {
            ret.push(m);
        }
    }
    ret
}

pub fn generate_bishop_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
) -> Vec<Move> {
    let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
    let mut ret = Vec::new();

    for (dr, dc) in directions {
        let mut r = origin.row as i8 + dr;
        let mut c = origin.col as i8 + dc;

        while let Some(dest) = Board::checked_coord(r, c) {
            let target = board.get(&dest);

            if target.is_color(active_player) {
                break;
            }
            let is_capture = target.get_piece().is_some();
            if let Some(m) = board.check_move(&origin, &dest, active_player) {
                ret.push(m);
            }
            if is_capture {
                break;
            }
            r += dr;
            c += dc;
        }
    }
    ret
}

pub fn generate_queen_moves(origin: &Coord, active_player: &Color, board: &mut Board) -> Vec<Move> {
    let mut ret: Vec<Move> = Vec::new();
    ret.extend(generate_bishop_moves(origin, active_player, board));
    ret.extend(generate_rook_moves(origin, active_player, board));
    ret
}

pub fn generate_king_moves(origin: &Coord, active_player: &Color, board: &mut Board) -> Vec<Move> {
    let cells: [(i8, i8); 8] = [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut ret = Vec::new();

    for (dr, dc) in cells {
        let new_row = origin.row as i8 + dr;
        let new_col = origin.col as i8 + dc;
        if let Some(dest) = Board::checked_coord(new_row, new_col)
            && let Some(m) = board.check_move(&origin, &dest, active_player)
        {
            ret.push(m);
        }
    }
    // castle : king not in check, rights available, path clear and not threaten
    if board.check.is_none() {
        let castle_rights = match active_player {
            White => board.white_castle,
            Black => board.black_castle,
        };
        if castle_rights.short {
            if let Some(through) = Board::checked_coord(origin.row as i8, origin.col as i8 + 1)
                && let Some(dest) = Board::checked_coord(origin.row as i8, origin.col as i8 + 2)
                && board.check_move(&origin, &through, active_player).is_some()
                && let Some(m) = board.check_move(&origin, &dest, active_player)
            {
                ret.push(m);
            }
        }
        if castle_rights.long {
            if let Some(through) = Board::checked_coord(origin.row as i8, origin.col as i8 - 1)
                && let Some(dest) = Board::checked_coord(origin.row as i8, origin.col as i8 - 2)
                && board.check_move(&origin, &through, active_player).is_some()
                && let Some(m) = board.check_move(&origin, &dest, active_player)
            {
                ret.push(m);
            }
        }
    }
    ret
}
