use crate::Coord;
use crate::board::Board;
use crate::board::board::CastleRights;
use crate::board::cell::Cell;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::pin_detection::{pin_detection, PinInfos};

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
    pub prev_score: i32,
}

impl Move {
    pub fn is_promotion(&self, board: &Board) -> bool {
        match board.grid[self.origin.row as usize][self.origin.col as usize].get_color() {
            Some(White) => self.dest.row == 7,
            Some(Black) => self.dest.row == 0,
            _ => unreachable!("Move without origin"),
        }
    }
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

// Vérifie si (dest - origin) est parallèle à la direction de clouage (dr, dc).
fn aligned_with_pin(origin: &Coord, dest: &Coord, dr: i8, dc: i8) -> bool {
    let row_diff = dest.row as i8 - origin.row as i8;
    let col_diff = dest.col as i8 - origin.col as i8;
    row_diff * dc == col_diff * dr
}

// Pousse un coup sans apply/undo/check_move quand c'est sûr.
// Fallback sur check_move si le roi est déjà en échec.
fn push_if_legal(
    board: &mut Board,
    origin: &Coord,
    dest: Coord,
    color: &Color,
    list: &mut MoveList,
    info: &PinInfos,
) {
    if board.get(&dest).is_color(color) {
        return;
    }
    if info.checker_count >= 1 {
        // Roi en échec : fallback check_move (correct, moins optimal)
        if let Some(m) = board.check_move(origin, &dest, color) {
            list.push(m);
        }
        return;
    }
    // Pas d'échec : vérifier le clouage
    if let Some((dr, dc)) = info.pins[origin.row as usize][origin.col as usize] {
        if !aligned_with_pin(origin, &dest, dr, dc) {
            return;
        }
    }
    list.push(board.build_move(*origin, dest, *color));
}

pub fn generate_moves(
    board: &mut Board,
    active_player: &Color,
    list: &mut MoveList,
    capture_only: bool,
) {
    let info = pin_detection(board, *active_player);
    for x in 0..8 {
        for y in 0..8 {
            if board.grid[x][y].is_color(active_player) {
                let origin = Coord { row: x as u8, col: y as u8 };
                if let Some(piece) = board.get(&origin).get_piece() {
                    match piece {
                        // Double échec : seul le roi peut bouger
                        _ if info.checker_count == 2 && *piece != King => continue,
                        Pawn   => pawn_moves(&origin, active_player, board, list, capture_only, &info),
                        Rook   => rook_moves(&origin, active_player, board, list, capture_only, &info),
                        Knight => knight_moves(&origin, active_player, board, list, capture_only, &info),
                        Bishop => bishop_moves(&origin, active_player, board, list, capture_only, &info),
                        Queen  => queen_moves(&origin, active_player, board, list, capture_only, &info),
                        King   => king_moves(&origin, active_player, board, list, capture_only),
                    }
                }
            }
        }
    }
}

fn pawn_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
    info: &PinInfos,
) {
    let dir = if *active_player == White { 1 } else { -1 };
    let last_rank = if *active_player == White { 7 } else { 0 };

    // Captures diagonales (et en passant)
    for dc in [1i8, -1] {
        if let Some(dest) = Board::checked_coord(origin.row as i8 + dir, origin.col as i8 + dc) {
            let target = board.get(&dest);
            let is_enemy = target.get_piece().is_some() && !target.is_color(active_player);
            let is_ep = board.en_passant.map_or(false, |ep| ep == dest);

            if is_enemy || is_ep {
                push_pawn_dest(origin, dest, *active_player, board, last_rank, list, info);
            }
        }
    }

    if capture_only {
        return;
    }

    // Avance d'une case
    if let Some(dest) = Board::checked_coord(origin.row as i8 + dir, origin.col as i8) {
        if board.get(&dest) == Cell::Free {
            push_pawn_dest(origin, dest, *active_player, board, last_rank, list, info);

            // Avance de deux cases depuis la rangée initiale
            let initial_row = if *active_player == White { origin.row == 1 } else { origin.row == 6 };
            if initial_row {
                if let Some(dest2) = Board::checked_coord(origin.row as i8 + dir * 2, origin.col as i8) {
                    if board.get(&dest2) == Cell::Free {
                        push_if_legal(board, origin, dest2, active_player, list, info);
                    }
                }
            }
        }
    }
}

fn push_pawn_dest(
    origin: &Coord,
    dest: Coord,
    color: Color,
    board: &mut Board,
    last_rank: u8,
    list: &mut MoveList,
    info: &PinInfos,
) {
    if dest.row == last_rank {
        // Promotion : vérifier la légalité via pin/checker sans apply/undo
        let is_legal = if info.checker_count >= 1 {
            board.check_move(origin, &dest, &color).is_some()
        } else if let Some((dr, dc)) = info.pins[origin.row as usize][origin.col as usize] {
            aligned_with_pin(origin, &dest, dr, dc)
        } else {
            true
        };

        if is_legal {
            for p in [Queen, Rook, Bishop, Knight] {
                // create_move gère capture + prev_score correctement
                let mut m = board.create_move(*origin, dest, color, MoveType::Promotion(Queen));
                m.move_type = MoveType::Promotion(p);
                list.push(m);
            }
        }
    } else {
        let is_ep = board.en_passant.map_or(false, |ep| ep == dest);
        if is_ep {
            // En passant : garder check_move (découverte horizontale non détectable géométriquement)
            if let Some(m) = board.check_move(origin, &dest, &color) {
                list.push(m);
            }
        } else {
            push_if_legal(board, origin, dest, &color, list, info);
        }
    }
}

fn rook_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
    info: &PinInfos,
) {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    process_sliding_piece(origin, &directions, active_player, board, list, capture_only, info);
}

fn bishop_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
    info: &PinInfos,
) {
    let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
    process_sliding_piece(origin, &directions, active_player, board, list, capture_only, info);
}

fn queen_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
    info: &PinInfos,
) {
    bishop_moves(origin, active_player, board, list, capture_only, info);
    rook_moves(origin, active_player, board, list, capture_only, info);
}

fn process_sliding_piece(
    origin: &Coord,
    dirs: &[(i8, i8)],
    color: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
    info: &PinInfos,
) {
    for (dr, dc) in dirs {
        let (mut r, mut c) = (origin.row as i8 + dr, origin.col as i8 + dc);
        while let Some(dest) = Board::checked_coord(r, c) {
            let target = board.get(&dest);
            if target.is_color(color) {
                break;
            }

            if capture_only && target.get_piece().is_none() {
                r += dr;
                c += dc;
                continue;
            }

            push_if_legal(board, origin, dest, color, list, info);

            if target.get_piece().is_some() {
                break;
            }
            r += dr;
            c += dc;
        }
    }
}

fn knight_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
    info: &PinInfos,
) {
    #[rustfmt::skip]
    let offsets = [
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2), (1, -2), (-1, 2), (-1, -2),
    ];
    for (dr, dc) in offsets {
        if let Some(dest) = Board::checked_coord(origin.row as i8 + dr, origin.col as i8 + dc) {
            if capture_only {
                let cell = board.grid[dest.row as usize][dest.col as usize];
                if !cell.is_color(active_player) && cell.get_piece().is_none() {
                    continue;
                }
            }
            push_if_legal(board, origin, dest, active_player, list, info);
        }
    }
}

fn king_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
) {
    #[rustfmt::skip]
    let offsets = [
        (-1, 1),  (0, 1), (1, 1),
        (-1, 0),          (1, 0),
        (-1, -1), (0, -1), (1, -1),
    ];
    for (dr, dc) in offsets {
        if let Some(dest) = Board::checked_coord(origin.row as i8 + dr, origin.col as i8 + dc) {
            if capture_only {
                let cell = board.grid[dest.row as usize][dest.col as usize];
                if !cell.is_color(active_player) && cell.get_piece().is_none() {
                    continue;
                }
            }
            if let Some(m) = board.check_move(origin, &dest, active_player) {
                list.push(m);
            }
        }
    }

    if capture_only {
        return;
    }
    if board.check.is_none() {
        let rights = if *active_player == White {
            board.white_castle
        } else {
            board.black_castle
        };

        let row = origin.row as usize;
        let col = origin.col as usize;

        if rights.short {
            if board.grid[row][col + 1] == Cell::Free && board.grid[row][col + 2] == Cell::Free {
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
        }

        if rights.long {
            if board.grid[row][col - 1] == Cell::Free
                && board.grid[row][col - 2] == Cell::Free
                && board.grid[row][col - 3] == Cell::Free
            {
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
}
