use crate::Board;
use crate::Coord;
use crate::board::cell::Cell;

impl Board {
    pub fn get(&self, coord: &Coord) -> Cell {
        self[*coord]
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
                print!("| {} ", self[(y, x)]);
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
