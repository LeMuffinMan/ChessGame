use crate::Coord;
use crate::board::Board;
use crate::board::board_struct::CastleRights;
use crate::board::cell::Cell;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Piece::*;

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
    // Promotion(Piece), dans mon implementation actuelle je n'ai pas encore recupere l'input pour la promotion
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
    let mut ret: Vec<Move> = Vec::new();

    // diagonals
    for dc in [1, -1] {
        if let Some(dest) = Board::checked_coord(origin.row as i8 + dir, origin.col as i8 + dc) {
            let target = board.get(&dest);
            let is_enemy = target.get_piece().is_some() && !target.is_color(active_player);
            if (is_enemy || board.en_passant == Some(dest))
                && let Some(m) = board.check_move(&origin, &dest, active_player)
            {
                ret.push(m);
            }
        }
    }

    // move 1 forward
    if let Some(dest) = Board::checked_coord(origin.row as i8 + dir, origin.col as i8)
        && board.get(&dest) == Cell::Free
        && let Some(m) = board.check_move(&origin, &dest, active_player)
    {
        ret.push(m);
    }

    // move 2 forward
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
            if let Some(m) = board.check_move(&origin, &dest, active_player) {
                ret.push(m);
            }
        }
    }

    ret
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
    //castle
    if board.check.is_none() {
        let little_castle = origin.col as i8 + 2;
        let long_castle = origin.col as i8 - 2;
        if let Some(dest) = Board::checked_coord(origin.row as i8, little_castle)
            && let Some(m) = board.check_move(&origin, &dest, active_player)
        {
            ret.push(m);
        }
        if let Some(dest) = Board::checked_coord(origin.row as i8, long_castle)
            && let Some(m) = board.check_move(&origin, &dest, active_player)
        {
            ret.push(m);
        }
    }
    ret
}
