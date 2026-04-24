use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Cell::Occupied;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::is_king_exposed::is_king_exposed;
use crate::board::move_gen::MoveType;
use crate::board::move_gen::{CastleSide::*, Move, MoveType::*};
use crate::engine::zobris_table::hash_from_scratch;
// use crate::engine::zobris_table::piece_index;
// use crate::engine::evaluator::Evaluator;
// use crate::engine::evaluator::PositionalEvaluator;
use crate::engine::evaluator::get_piece_value_at;

#[derive(Clone, PartialEq)]
pub struct Board {
    pub grid: [[Cell; 8]; 8],
    pub white_castle: CastleRights,
    pub black_castle: CastleRights,
    pub white_king: Coord,
    pub black_king: Coord,
    pub en_passant: Option<Coord>,
    pub check: Option<Coord>,
    pub score: i32,
    pub hash: u64,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Default)]
pub struct CastleRights {
    pub long: bool,
    pub short: bool,
}

pub struct FenInfo {
    pub board: Board,
    pub active_color: Color,
    pub halfmove_clock: u32,
    pub fullmove: u32,
}
use crate::board::cell::Cell;

impl Board {
    pub fn build_move(&self, origin: Coord, dest: Coord, active_player: Color) -> Move {
        let m_type = self.get_move_type(
            origin,
            dest,
            self.grid[origin.row as usize][origin.col as usize].get_piece(),
        );
        self.create_move(origin, dest, active_player, m_type)
    }

    pub fn create_move(
        &self,
        origin: Coord,
        dest: Coord,
        active_player: Color,
        m_type: MoveType,
    ) -> Move {
        let capture_cell = match m_type {
            MoveType::EnPassant => match active_player {
                Color::White => self.grid[(dest.row - 1) as usize][dest.col as usize],
                Color::Black => self.grid[(dest.row + 1) as usize][dest.col as usize],
            },
            _ => self.grid[dest.row as usize][dest.col as usize],
        };

        self.new_move(origin, dest, capture_cell, m_type)
    }
    pub fn new_move(
        &self,
        origin: Coord,
        dest: Coord,
        capture_cell: Cell,
        m_type: MoveType,
    ) -> Move {
        Move {
            dest,
            origin,
            capture: capture_cell,
            en_passant: self.en_passant,
            check: self.check,
            white_castle: self.white_castle,
            black_castle: self.black_castle,
            move_type: m_type,
            prev_score: self.score,
        }
    }

    fn get_move_type(&self, origin: Coord, dest: Coord, piece_moving: Option<&Piece>) -> MoveType {
        match piece_moving {
            Some(Pawn) => {
                if dest.col != origin.col
                    && self.grid[dest.row as usize][dest.col as usize] == Cell::Free
                {
                    EnPassant
                } else {
                    Regular
                }
            }
            Some(King) => {
                if (origin.col as i8 - dest.col as i8).abs() > 1 {
                    if (origin.col as i8 - dest.col as i8) < 0 {
                        Castle(Right)
                    } else {
                        Castle(Left)
                    }
                } else {
                    Regular
                }
            }
            _ => Regular,
        }
    }

    fn update_capture_rook(&mut self, m: &Move) {
        if let Cell::Occupied(Rook, color) = m.capture {
            let rights = match color {
                White => &mut self.white_castle,
                Black => &mut self.black_castle,
            };
            match (m.dest.row, m.dest.col) {
                (0, 0) => rights.long = false,
                (0, 7) => rights.short = false,
                (7, 0) => rights.long = false,
                (7, 7) => rights.short = false,
                _ => {}
            }
        }
    }

    //mode : true = we add a piece to the board / false = we remove a piece from the board
    fn update_board_score(&mut self, cell: &Cell, target: &Coord, mode: bool) {
        if let Occupied(piece, color) = cell {
            let value = get_piece_value_at(piece, color, target);
            if mode {
                self.score += value;
            } else {
                self.score -= value;
            }
        }
    }

    pub fn apply_move(&mut self, m: &Move, active_player: Color) {
        let capture_coord = match m.move_type {
            EnPassant => {
                let row = if active_player == White {
                    m.dest.row - 1
                } else {
                    m.dest.row + 1
                };
                Coord {
                    row,
                    col: m.dest.col,
                }
            }
            _ => m.dest,
        };

        self.update_board_score(&self.get(&m.origin), &m.origin, false);
        self.update_board_score(&m.capture, &capture_coord, false);

        self.en_passant = None;
        self.check = None;
        match self.get(&m.origin).get_piece() {
            Some(Pawn) => self.update_en_passant(&m.origin, &m.dest, &active_player),
            Some(King) => self.update_king_move(&active_player, m),
            Some(Rook) => self.update_rook_move(&active_player, m),
            _ => {}
        }
        self.update_capture_rook(m);

        if let EnPassant = m.move_type {
            self.grid[capture_coord.row as usize][capture_coord.col as usize] = Cell::Free;
        }

        self.grid[m.dest.row as usize][m.dest.col as usize] = std::mem::replace(
            &mut self.grid[m.origin.row as usize][m.origin.col as usize],
            Cell::Free,
        );

        if let Promotion(promoted) = m.move_type {
            self.grid[m.dest.row as usize][m.dest.col as usize] =
                Cell::Occupied(promoted, active_player);
        }

        if let Castle(side) = m.move_type {
            let row = if active_player == White { 0 } else { 7 };
            let (r_orig, r_dest) = if side == Right { (7, 5) } else { (0, 3) };
            let rook = Cell::Occupied(Rook, active_player);
            self.update_board_score(&rook, &Coord { row, col: r_orig }, false);
            self.grid[row as usize][r_orig as usize] = Cell::Free;
            self.grid[row as usize][r_dest as usize] = rook;
            self.update_board_score(&rook, &Coord { row, col: r_dest }, true);
        }

        self.update_board_score(&self.get(&m.dest), &m.dest, true);

        // self.debug_check_score(&format!(
        //     "after apply_move active={:?} type={:?} from=({},{}) to=({},{}) capture={:?}",
        //     active_player,
        //     m.move_type,
        //     m.origin.row,
        //     m.origin.col,
        //     m.dest.row,
        //     m.dest.col,
        //     m.capture
        // ))
    }

    pub fn update_king_move(&mut self, active_player: &Color, m: &Move) {
        self.update_king_castle(&m.origin, &m.dest, &active_player);
        match active_player {
            White => {
                self.white_king = m.dest;
                self.white_castle = CastleRights {
                    long: false,
                    short: false,
                };
            }
            Black => {
                self.black_king = m.dest;
                self.black_castle = CastleRights {
                    long: false,
                    short: false,
                };
            }
        }
    }

    pub fn update_rook_move(&mut self, active_player: &Color, m: &Move) {
        match (active_player, m.origin.row, m.origin.col) {
            (White, 0, 0) => self.white_castle.long = false,
            (White, 0, 7) => self.white_castle.short = false,
            (Black, 7, 0) => self.black_castle.long = false,
            (Black, 7, 7) => self.black_castle.short = false,
            _ => {}
        }
    }

    //refacto : prend une Struct Undo en param ?
    pub fn undo_move(&mut self, m: Move, active_player: Color) {
        let capture_coord = match m.move_type {
            EnPassant => {
                let row = if active_player == White {
                    m.dest.row - 1
                } else {
                    m.dest.row + 1
                };
                Coord {
                    row,
                    col: m.dest.col,
                }
            }
            _ => m.dest,
        };

        match m.move_type {
            EnPassant => {
                self.grid[m.dest.row as usize][m.dest.col as usize] = Cell::Free;
                self.grid[capture_coord.row as usize][capture_coord.col as usize] = m.capture;
                self.grid[m.origin.row as usize][m.origin.col as usize] =
                    Cell::Occupied(Pawn, active_player);
            }
            Castle(side) => {
                let row = if active_player == White { 0 } else { 7 };
                let (r_orig, r_dest) = if side == Right { (7, 5) } else { (0, 3) };
                let rook = Cell::Occupied(Rook, active_player);

                self.grid[row as usize][r_dest as usize] = Cell::Free;
                self.grid[row as usize][r_orig as usize] = rook;

                self.grid[row as usize][4] = Cell::Occupied(King, active_player);
                self.grid[m.dest.row as usize][m.dest.col as usize] = Cell::Free;

                if active_player == White {
                    self.white_king = Coord { row: 0, col: 4 };
                } else {
                    self.black_king = Coord { row: 7, col: 4 };
                }
            }
            Promotion(_) => {
                self.grid[m.origin.row as usize][m.origin.col as usize] =
                    Cell::Occupied(Pawn, active_player);
                self.grid[m.dest.row as usize][m.dest.col as usize] = m.capture;
            }
            Regular => {
                let moving_piece = self.get(&m.dest);
                self.grid[m.origin.row as usize][m.origin.col as usize] = moving_piece;
                self.grid[m.dest.row as usize][m.dest.col as usize] = m.capture;

                if let Some(King) = moving_piece.get_piece() {
                    match active_player {
                        White => self.white_king = m.origin,
                        Black => self.black_king = m.origin,
                    }
                }
            }
        }

        self.en_passant = m.en_passant;
        self.check = m.check;
        self.white_castle = m.white_castle;
        self.black_castle = m.black_castle;
        self.score = m.prev_score;

        // self.debug_check_score(&format!(
        //     "after undo_move active={:?} type={:?} from=({},{}) to=({},{}) capture={:?}",
        //     active_player,
        //     m.move_type,
        //     m.origin.row,
        //     m.origin.col,
        //     m.dest.row,
        //     m.dest.col,
        //     m.capture
        // ));
    }

    //reduire l'appel a check move ou enlever le is_king_exposed ? renommer
    pub fn check_move(
        &mut self,
        origin: &Coord,
        dest: &Coord,
        active_player: &Color,
    ) -> Option<Move> {
        if self.get(dest).is_color(active_player) {
            return None;
        }
        let m = self.build_move(*origin, *dest, *active_player);
        self.apply_move(&m, *active_player);
        let exposed = is_king_exposed(self, active_player);
        self.undo_move(m, *active_player);
        if !exposed {
            return Some(m);
        }
        None
    }

    //renommer
    pub fn checked_coord(row: i8, col: i8) -> Option<Coord> {
        if (0..8).contains(&row) && (0..8).contains(&col) {
            Some(Coord {
                row: row as u8,
                col: col as u8,
            })
        } else {
            None
        }
    }

    pub fn update_en_passant(&mut self, origin: &Coord, to: &Coord, active_player: &Color) {
        let dif = to.row as i8 - origin.row as i8;
        if dif.abs() == 2 {
            let mid_row = match active_player {
                White => to.row - 1,
                Black => to.row + 1,
            };
            self.en_passant = Some(Coord {
                row: mid_row as u8,
                col: origin.col,
            });
        } else {
            self.en_passant = None;
        }
    }

    pub fn update_king_castle(&mut self, origin: &Coord, to: &Coord, color: &Color) {
        let dif_col = to.col as i8 - origin.col as i8;
        let row = match color {
            White => 0,
            Black => 7,
        };
        if dif_col == -2 {
            let col = to.col as usize;
            if col > 0 {
                self.grid[row][0] = Cell::Free;
                self.grid[row][col + 1] = Cell::Occupied(Rook, *color);
            }
        } else if dif_col == 2 {
            let col = to.col as usize;
            if col > 0 {
                self.grid[row][7] = Cell::Free;
                self.grid[row][col - 1] = Cell::Occupied(Rook, *color);
            }
        }
    }

    pub fn init_board() -> Board {
        let mut board = Board {
            grid: [[Cell::Free; 8]; 8],
            en_passant: None,
            white_castle: CastleRights {
                long: true,
                short: true,
            },
            black_castle: CastleRights {
                long: true,
                short: true,
            },
            white_king: (Coord { row: 0, col: 4 }),
            black_king: (Coord { row: 7, col: 4 }),
            check: None,
            score: 0,
            hash: 0,
        };
        board.fill_side(White);
        board.fill_side(Black);
        for x in 0..8 {
            for y in 0..8 {
                let target = Coord { row: x, col: y };
                if let Occupied(piece, color) = board.get(&target) {
                    board.score += get_piece_value_at(&piece, &color, &target);
                }
            }
        }
        board.hash = hash_from_scratch(&board, Color::White);
        board
    }

    pub fn board_from_fen(fen: &str) -> FenInfo {
        let mut board = Board {
            grid: [[Cell::Free; 8]; 8],
            en_passant: None,
            white_castle: CastleRights {
                long: false,
                short: false,
            },
            black_castle: CastleRights {
                long: false,
                short: false,
            },
            white_king: Coord { row: 0, col: 4 },
            black_king: Coord { row: 7, col: 4 },
            check: None,
            score: 0,
            hash: 0,
        };

        let mut parts = fen.split(' ');

        let placement = parts.next().unwrap_or("");
        let mut row: i8 = 7;
        let mut col: i8 = 0;

        for c in placement.chars() {
            match c {
                '/' => {
                    row -= 1;
                    col = 0;
                }
                '1'..='8' => {
                    col += c as i8 - '0' as i8;
                }
                _ => {
                    let color = if c.is_uppercase() { White } else { Black };
                    let piece = match c.to_ascii_lowercase() {
                        'p' => Pawn,
                        'r' => Rook,
                        'n' => Knight,
                        'b' => Bishop,
                        'q' => Queen,
                        'k' => King,
                        _ => unreachable!("invalid FEN char: {c}"),
                    };
                    let coord = Coord {
                        row: row as u8,
                        col: col as u8,
                    };
                    if piece == King {
                        match color {
                            White => board.white_king = coord,
                            Black => board.black_king = coord,
                        }
                    }
                    board.grid[row as usize][col as usize] = Occupied(piece, color);
                    board.score += get_piece_value_at(&piece, &color, &coord);
                    col += 1;
                }
            }
        }

        let active_color = match parts.next().unwrap_or("w") {
            "b" => Black,
            _ => White,
        };

        for c in parts.next().unwrap_or("-").chars() {
            match c {
                'K' => board.white_castle.short = true,
                'Q' => board.white_castle.long = true,
                'k' => board.black_castle.short = true,
                'q' => board.black_castle.long = true,
                _ => {}
            }
        }

        let ep = parts.next().unwrap_or("-");
        if ep != "-" {
            let mut chars = ep.chars();
            if let (Some(file), Some(rank)) = (chars.next(), chars.next()) {
                board.en_passant = Some(Coord {
                    row: rank as u8 - b'1',
                    col: file as u8 - b'a',
                });
            }
        }

        let halfmove_clock = parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);

        let fullmove = parts.next().unwrap_or("1").parse::<u32>().unwrap_or(1);

        FenInfo {
            board,
            active_color,
            halfmove_clock,
            fullmove,
        }
    }

    pub fn fill_side(&mut self, color: Color) {
        let color_idx = match color {
            White => 0,
            Black => 7,
        };
        for x in 0..8 {
            self.grid[color_idx][x] = match x {
                0 | 7 => Cell::Occupied(Rook, color),
                1 | 6 => Cell::Occupied(Knight, color),
                2 | 5 => Cell::Occupied(Bishop, color),
                3 => Cell::Occupied(Queen, color),
                4 => Cell::Occupied(King, color),
                _ => unreachable!(),
            };
            match color_idx {
                0 => self.grid[color_idx + 1][x] = Cell::Occupied(Pawn, color),
                7 => self.grid[color_idx - 1][x] = Cell::Occupied(Pawn, color),
                _ => unreachable!(),
            };
            if color_idx == 0 {
                self.grid[color_idx + 1][x] = Cell::Occupied(Pawn, color);
            } else {
                self.grid[color_idx - 1][x] = Cell::Occupied(Pawn, color);
            }
        }
    }
}
