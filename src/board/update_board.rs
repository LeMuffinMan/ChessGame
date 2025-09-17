use crate::Board;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;

impl Board {
    //if a move has passed the is legal move and is king exposed test : we update the board
    //applying this move
    pub fn update_board(&mut self, from: &Coord, to: &Coord, color: &Color) {
        self.en_passant = None;
        self.check = None;
        match self.get(from).get_piece() {
            Some(piece) => match piece {
                Pawn => self.update_en_passant(from, to),
                King => self.update_king_castle(from, to, color),
                Knight | Rook | Queen | Bishop => {}
            },
            None => {
                println!("Error : update board : from cell empty")
            }
        }
        //replace puts Cell::Free in the board cell "from" and returns what "from" contained
        //we assign the "to" cell with this returned value
        self.grid[to.row as usize][to.col as usize] = std::mem::replace(
            &mut self.grid[from.row as usize][from.col as usize],
            Cell::Free,
        );
    }

    pub fn update_en_passant(&mut self, from: &Coord, to: &Coord) {
        //if the moving piece is a pawn and it just moved in diag : if the cell is empty, it is a
        //en passant, so we need to clean the cell "behind" this pawn following the side
        if self.grid[to.row as usize][to.col as usize].is_empty() && from.col != to.col {
            self.grid[to.row as usize][to.col as usize] =
                self.grid[from.row as usize][from.col as usize];
            self.grid[from.row as usize][to.col as usize] = Cell::Free;
            return;
        }
        //if the pawn moved 2 row, it oppens a en_passant move for opponent : we store the coord of
        //the exposed pasn
        let dif = from.row as i8 - to.row as i8;
        if dif.abs() == 2 {
            self.en_passant = Some(*to);
        }
    }

    pub fn update_king_castle(&mut self, from: &Coord, to: &Coord, color: &Color) {
        let dif_col = to.col as i8 - from.col as i8;
        let row = match color {
            White => 0,
            Black => 7,
        };
        //the dif_col allows us to know if its a long or a short castle
        //thus, wich rook to update
        if dif_col == -2 {
            let col = to.col as usize;
            if col > 0 {
                self.grid[row][0] = Cell::Free;
                self.grid[row][col + 1] = Cell::Occupied(Piece::Rook, *color);
            }
        } else if dif_col == 2 {
            let col = to.col as usize;
            if col > 0 {
                self.grid[row][7] = Cell::Free;
                self.grid[row][col - 1] = Cell::Occupied(Rook, *color);
            }
        }
    }
}
