use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::cell::Cell::Occupied;
use crate::board::cell::Piece::*;
use crate::engine::evaluator::{get_piece_value_at, non_pawn_raw};
use crate::engine::zobris_table::hash_from_scratch;
use std::ops::Index;
use std::ops::IndexMut;

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
    pub non_pawn_material: i32,
    pub hash: u64,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Default)]
pub struct CastleRights {
    pub long: bool,
    pub short: bool,
}

impl IndexMut<Coord> for Board {
    fn index_mut(&mut self, coord: Coord) -> &mut Cell {
        &mut self.grid[coord.row as usize][coord.col as usize]
    }
}

impl Index<Coord> for Board {
    type Output = Cell;

    fn index(&self, coord: Coord) -> &Cell {
        &self.grid[coord.row as usize][coord.col as usize]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, coord: (usize, usize)) -> &mut Cell {
        &mut self.grid[coord.0][coord.1]
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Cell;

    fn index(&self, coord: (usize, usize)) -> &Cell {
        &self.grid[coord.0][coord.1]
    }
}

impl IndexMut<(i8, i8)> for Board {
    fn index_mut(&mut self, coord: (i8, i8)) -> &mut Cell {
        &mut self.grid[coord.0 as usize][coord.1 as usize]
    }
}

impl Index<(i8, i8)> for Board {
    type Output = Cell;

    fn index(&self, coord: (i8, i8)) -> &Cell {
        &self.grid[coord.0 as usize][coord.1 as usize]
    }
}

impl Index<(i32, i32)> for Board {
    type Output = Cell;

    fn index(&self, coord: (i32, i32)) -> &Cell {
        &self.grid[coord.0 as usize][coord.1 as usize]
    }
}

impl IndexMut<(i32, i32)> for Board {
    fn index_mut(&mut self, coord: (i32, i32)) -> &mut Cell {
        &mut self.grid[coord.0 as usize][coord.1 as usize]
    }
}

impl Board {
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
            non_pawn_material: 0,
            hash: 0,
        };
        board.fill_side(White);
        board.fill_side(Black);
        for x in 0..8 {
            for y in 0..8 {
                let target = Coord { row: x, col: y };
                if let Occupied(piece, color) = board.get(&target) {
                    board.score += get_piece_value_at(&piece, &color, &target);
                    board.non_pawn_material += non_pawn_raw(&piece);
                }
            }
        }
        board.hash = hash_from_scratch(&board, Color::White);
        board
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

    pub fn sync_hash(&mut self, active: Color) {
        self.hash = hash_from_scratch(self, active);
    }

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
}
