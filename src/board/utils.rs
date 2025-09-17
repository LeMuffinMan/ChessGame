use crate::Board;
use crate::Color;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::cell::Piece;
use crate::board::validate_move::is_legal_move::is_legal_move;

impl Board {
    pub fn get(&self, coord: &Coord) -> Cell {
        self.grid[coord.row as usize][coord.col as usize]
    }

    pub fn is_legal_move(&self, from: &Coord, to: &Coord, color: &Color) -> bool {
        is_legal_move(from, to, color, self)
    }
    pub fn get_king(&self, color: &Color) -> Option<Coord> {
        for x in 0..8 {
            for y in 0..8 {
                if self.grid[x][y].is_color(color)
                    && let Some(Piece::King) = self.grid[x][y].get_piece()
                {
                    return Some(Coord {
                        row: x as u8,
                        col: y as u8,
                    });
                }
            }
        }
        None
    }

    pub fn print(&self) {
        print!(" ");
        for x in 0..8 {
            print!("   ");
            print!("{}", (b'A' + x as u8) as char);
        }
        println!();
        for y in (0..8).rev() {
            print!("  ");
            for _ in 0..8 {
                print!("----");
            }
            println!();
            print!("{} ", y + 1);
            for x in 0..8 {
                print!("| {} ", self.grid[y][x]);
            }
            println!("|");
        }
        print!("  ");
        for _ in 0..8 {
            print!("----");
        }
        println!();
    }
}

impl Coord {
    pub fn flip(&mut self, flip: bool) -> Coord {
        if flip {
            self.row = 7 - self.row;
            self.col = 7 - self.col;
        }
        *self
    }
}
