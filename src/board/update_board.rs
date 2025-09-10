use crate::Board;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::cell::Cell;
use crate::cell::Piece;
use crate::cell::Piece::*;

impl Board {
    pub fn update_board(&mut self, from: &Coord, to: &Coord, color: &Color) {
        self.en_passant = None;
        self.check = None;
        match self.get(from).get_piece() {
            Some(piece) => match piece {
                Pawn => self.update_en_passant(from, to),
                Rook => {}
                King => self.update_king_castle(from, to, color),
                Knight => {}
                Queen => {}
                Bishop => {}
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
        //Si la piece au depart est un pion, et que sa case d'arrivee est vide et en diag
        //c'est une prise en passant : clean from cell, et le pion mange
        if self.grid[to.row as usize][to.col as usize].is_empty() && from.col != to.col {
            self.grid[to.row as usize][to.col as usize] =
                self.grid[from.row as usize][from.col as usize];
            self.grid[from.row as usize][to.col as usize] = Cell::Free;
            return;
        }
        //si le pion bouge de deux cases : c'est un double pas : flag en passant
        let dif = from.row as i8 - to.row as i8;
        if dif.abs() == 2 {
            self.en_passant = Some(*to);
            // println!("En passant flag at {:?}", to);
        }
    }

    pub fn update_king_castle(&mut self, from: &Coord, to: &Coord, color: &Color) {
        let dif_col = to.col as i8 - from.col as i8;
        let row = match color {
            White => 0,
            Black => 7,
        };
        //si le roi fait un castle a gauche : tour a droite
        if dif_col == -2 {
            let col = to.col as usize;
            if col > 0 {
                self.grid[row][0] = Cell::Free;
                self.grid[row][col + 1] = Cell::Occupied(Piece::Rook, *color);
            }
        }
        //si le roi fait un castle a droite : tour a gauche
        else if dif_col == 2 {
            let col = to.col as usize;
            if col > 0 {
                self.grid[row][7] = Cell::Free;
                self.grid[row][col - 1] = Cell::Occupied(Rook, *color);
            }
        }
    }
}
