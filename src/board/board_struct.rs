use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::cell::Cell;
use crate::cell::Piece;
use crate::cell::Piece::*;

#[derive(Clone)]
pub struct Board {
    pub grid: [[Cell; 8]; 8],
    pub white_castle: (bool, bool), //(long, short)
    pub black_castle: (bool, bool),
    pub threaten_cells: Vec<Coord>,
    pub legals_moves: Vec<(Coord, Coord)>,
    pub en_passant: Option<Coord>,
    pub check: Option<Coord>,
    pub pawn_to_promote: Option<Coord>,
    pub promote: Option<Piece>,
}

impl Board {
    pub fn init_board() -> Board {
        let mut board = Board {
            grid: [[Cell::Free; 8]; 8],
            en_passant: None,
            white_castle: (true, true),
            black_castle: (true, true),
            threaten_cells: Vec::new(),
            legals_moves: Vec::new(),
            check: None,
            pawn_to_promote: None,
            promote: None,
        };

        board.fill_side(White);
        board.fill_side(Black);
        board.update_legals_moves(&Color::White);
        board
    }
    pub fn fill_side(&mut self, color: Color) {
        let color_idx = match color {
            White => 0,
            Black => 7,
        };
        for x in 0..8 {
            // fill the base line
            self.grid[color_idx][x] = match x {
                0 | 7 => Cell::Occupied(Rook, color),
                1 | 6 => Cell::Occupied(Knight, color),
                2 | 5 => Cell::Occupied(Bishop, color),
                3 => Cell::Occupied(Queen, color),
                4 => Cell::Occupied(King, color),
                _ => unreachable!(),
            };
            // fill the pawns
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
