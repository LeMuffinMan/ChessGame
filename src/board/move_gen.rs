use crate::Coord;

use crate::board::Board;
use crate::board::board::CastleRights;
use crate::board::cell::Cell;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::is_king_exposed::is_king_exposed;

#[derive(Copy, Clone, PartialEq, Default)]
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

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum MoveType {
    #[default]
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

const MAX_MOVES: usize = 256;

pub struct MoveList {
    pub moves: [Move; MAX_MOVES],
    pub count: usize,
}

impl MoveList {
    pub fn new() -> Self {
        // C'est de l'allocation sur la pile : extrêmement rapide !
        Self {
            moves: [Move::default(); MAX_MOVES],
            count: 0,
        }
    }

    pub fn push(&mut self, m: Move) {
        self.moves[self.count] = m;
        self.count += 1;
    }
}

pub fn generate_moves(board: &mut Board, active_player: &Color, list: &mut MoveList) {
    for x in 0..8 {
        for y in 0..8 {
            if board.grid[x][y].is_color(active_player) {
                let origin = Coord {
                    row: x as u8,
                    col: y as u8,
                };
                if let Some(piece) = board.get(&origin).get_piece() {
                    match piece {
                        Pawn => generate_pawn_moves(&origin, active_player, board, list),
                        Rook => generate_rook_moves(&origin, active_player, board, list),
                        Knight => generate_knight_moves(&origin, active_player, board, list),
                        Bishop => generate_bishop_moves(&origin, active_player, board, list),
                        Queen => generate_queen_moves(&origin, active_player, board, list),
                        King => generate_king_moves(&origin, active_player, board, list),
                    }
                }
            }
        }
    }
}

pub fn generate_pawn_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
) {
    let dir = if *active_player == White { 1 } else { -1 };
    let last_rank = if *active_player == White { 7 } else { 0 };

    for dc in [1, -1] {
        if let Some(dest) = Board::checked_coord(origin.row as i8 + dir, origin.col as i8 + dc) {
            let target = board.get(&dest);
            let is_enemy = target.get_piece().is_some() && !target.is_color(active_player);
            let is_ep = board.en_passant.map_or(false, |ep| {
                dest.col == ep.col && dest.row as i8 == ep.row as i8 + dir
            });

            if is_enemy || is_ep {
                push_pawn_dest(origin, &dest, active_player, board, last_rank, list);
            }
        }
    }

    if let Some(dest) = Board::checked_coord(origin.row as i8 + dir, origin.col as i8) {
        if board.get(&dest) == Cell::Free {
            push_pawn_dest(origin, &dest, active_player, board, last_rank, list);

            // Double avance initiale
            let initial_row = if *active_player == White {
                origin.row == 1
            } else {
                origin.row == 6
            };
            if initial_row {
                if let Some(dest2) =
                    Board::checked_coord(origin.row as i8 + dir * 2, origin.col as i8)
                {
                    if board.get(&dest2) == Cell::Free {
                        if let Some(m) = board.check_move(origin, &dest2, active_player) {
                            list.push(m);
                        }
                    }
                }
            }
        }
    }
}

fn push_pawn_dest(
    origin: &Coord,
    dest: &Coord,
    color: &Color,
    board: &mut Board,
    last_rank: u8,
    list: &mut MoveList,
) {
    if dest.row == last_rank {
        // Simuler pour l'échec
        let base = Move {
            origin: *origin,
            dest: *dest,
            capture: board.get(dest),
            en_passant: board.en_passant,
            white_castle: board.white_castle,
            black_castle: board.black_castle,
            move_type: MoveType::Promotion(Queen),
            ..Default::default()
        };

        board.apply_move(&base, *color);
        let safe = !is_king_exposed(board, color);
        board.undo_move(base, *color);

        if safe {
            for p in [Queen, Rook, Bishop, Knight] {
                let mut m = base;
                m.move_type = MoveType::Promotion(p);
                list.push(m);
            }
        }
    } else if let Some(m) = board.check_move(origin, dest, color) {
        list.push(m);
    }
}
pub fn generate_rook_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
) {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    process_sliding_piece(origin, &directions, active_player, board, list);
}

pub fn generate_bishop_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
) {
    let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
    process_sliding_piece(origin, &directions, active_player, board, list);
}

pub fn generate_queen_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
) {
    generate_bishop_moves(origin, active_player, board, list);
    generate_rook_moves(origin, active_player, board, list);
}

fn process_sliding_piece(
    origin: &Coord,
    dirs: &[(i8, i8)],
    color: &Color,
    board: &mut Board,
    list: &mut MoveList,
) {
    for (dr, dc) in dirs {
        let (mut r, mut c) = (origin.row as i8 + dr, origin.col as i8 + dc);
        while let Some(dest) = Board::checked_coord(r, c) {
            let target = board.get(&dest);
            if target.is_color(color) {
                break;
            }

            if let Some(m) = board.check_move(origin, &dest, color) {
                list.push(m);
            }

            if target.get_piece().is_some() {
                break;
            }
            r += dr;
            c += dc;
        }
    }
}

pub fn generate_knight_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
) {
    #[rustfmt::skip]
    let offsets = [
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2),(1, -2), (-1, 2), (-1, -2),
    ];
    for (dr, dc) in offsets {
        if let Some(dest) = Board::checked_coord(origin.row as i8 + dr, origin.col as i8 + dc) {
            if let Some(m) = board.check_move(origin, &dest, active_player) {
                list.push(m);
            }
        }
    }
}

pub fn generate_king_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
) {
    #[rustfmt::skip]
    let offsets = [
        (-1, 1), (0, 1), (1, 1),  (-1, 0),
        (1, 0), (-1, -1), (0, -1),  (1, -1),
    ];
    for (dr, dc) in offsets {
        if let Some(dest) = Board::checked_coord(origin.row as i8 + dr, origin.col as i8 + dc) {
            if let Some(m) = board.check_move(origin, &dest, active_player) {
                list.push(m);
            }
        }
    }

    if board.check.is_none() {
        let rights = if *active_player == White {
            board.white_castle
        } else {
            board.black_castle
        };
        // Petit roque
        if rights.short {
            if let (Some(t), Some(d)) = (
                Board::checked_coord(origin.row as i8, origin.col as i8 + 1),
                Board::checked_coord(origin.row as i8, origin.col as i8 + 2),
            ) {
                if board.check_move(origin, &t, active_player).is_some() {
                    if let Some(m) = board.check_move(origin, &d, active_player) {
                        list.push(m);
                    }
                }
            }
        }
        if rights.long {
            if let (Some(t), Some(d)) = (
                Board::checked_coord(origin.row as i8, origin.col as i8 - 1),
                Board::checked_coord(origin.row as i8, origin.col as i8 - 2),
            ) {
                if board.check_move(origin, &t, active_player).is_some() {
                    if let Some(m) = board.check_move(origin, &d, active_player) {
                        list.push(m);
                    }
                }
            }
        }
    }
}
