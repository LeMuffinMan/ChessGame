use crate::Coord;
use crate::Color;
use crate::Color::*;
use crate::cell::Cell;
use crate::cell::Piece;
use crate::cell::Piece::*;
use crate::validate_move::validate_move::is_king_exposed;
use crate::validate_move::validate_move::is_legal_move;

#[derive(Clone)]
pub struct Board {
    pub grid: [[Cell; 8]; 8],
    pub white_castle: (bool, bool), //(long, short)
    pub black_castle: (bool, bool),
    pub threaten_cells: Vec<Coord>,
    pub legals_moves: Vec<(Coord, Coord)>,
    pub en_passant: Option<Coord>,
}

impl Board {
    fn fill_side(&mut self, color: Color) {
        let color_idx = match color {
            White => 0,
            Black => 7,
        };
        for x in 0..8 {
            // fill the base line
            self.grid[color_idx][x] = match x {
                //pour la ligne tout en bas
                0 | 7 => Cell::Occupied(Rook, color),
                1 | 6 => Cell::Occupied(Knight, color),
                2 | 5 => Cell::Occupied(Bishop, color),
                3 => Cell::Occupied(Queen, color),
                4 => Cell::Occupied(King, color),
                _ => unreachable!(), //cas a couvrir par defaut mais impossible car board 8x8
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

    pub fn init_board() -> Board {
        let mut board = Board {
            grid: [[Cell::Free; 8]; 8],
            en_passant: None,
            white_castle: (true, true),
            black_castle: (true, true),
            threaten_cells: Vec::new(),
            legals_moves: Vec::new(),
        };

        board.fill_side(White);
        board.fill_side(Black);

        board
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

    pub fn get(&self, coord: &Coord) -> Cell {
        self.grid[coord.row as usize][coord.col as usize]
    }

    pub fn is_legal_move(&self, from: &Coord, to: &Coord, color: &Color) -> bool {
        is_legal_move(from, to, color, self)
    }
    pub fn get_king(&self, color: &Color) -> Option<Coord> {
        for x in 0..8 {
            for y in 0..8 {
                if self.grid[x][y].is_color(color) {
                    if let Some(Piece::King) = self.grid[x][y].get_piece() {
                        return Some(Coord {
                            row: x as u8,
                            col: y as u8,
                        });
                    }
                }
            }
        }
        None
    }

    pub fn update_board(&mut self, from: &Coord, to: &Coord, color: &Color) {
        self.en_passant = None;
        //prise en passant
        match self.grid[from.row as usize][from.col as usize].get_piece() {
            Some(piece) => match piece {
                Pawn => {
                    //Si la piece au depart est un pion, et que sa case d'arrivee est vide et en diag
                    //c'est une prise en passant : clean from cell, et le pion mange
                    if self.grid[to.row as usize][to.col as usize].is_empty() && from.col != to.col
                    {
                        self.grid[from.row as usize][to.col as usize] = Cell::Free;
                        self.grid[to.row as usize][to.col as usize] =
                            self.grid[from.row as usize][from.col as usize];
                        self.grid[from.row as usize][from.col as usize] = Cell::Free;
                        return;
                    }
                    //si le pion bouge de deux cases : c'est un double pas : flag en passant
                    let dif = from.row as i8 - to.row as i8;
                    if dif.abs() == 2 {
                        self.en_passant = Some(*to);
                        // println!("En passant flag at {:?}", to);
                    }
                }
                Rook => {
                    //si une des tour bouge : on passe a false le castle_bool qui correspond
                    let mut castle_bools = if *color == White {
                        self.white_castle
                    } else {
                        self.black_castle
                    };
                    if castle_bools.0 == true || castle_bools.1 == true {
                        match from.col {
                            0 => castle_bools.0 = false,
                            7 => castle_bools.1 = false,
                            _ => {}
                        };
                    }
                }
                King => {
                    //si le roi bouge : on invalide les deux castles
                    let mut castle_bools = if *color == White {
                        self.white_castle
                    } else {
                        self.black_castle
                    };
                    if castle_bools.0 == true || castle_bools.1 == true {
                        castle_bools.0 = false;
                        castle_bools.1 = false;
                    }
                    //Roque
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
                    //regular moves : checker la threat
                }
                Knight => {
                    //
                }
                Queen => {}
                Bishop => {}
            },
            None => {
                println!("Error : update board : from cell empty")
            }
        }
        //Dans tous les autres cas : on vide la case de depart et on ecrase la case d'arrivee
        //replace puts Cell::Free in the board cell "from" and returns what "from" contained
        //we assign the "to" cell with this returned value
        self.grid[to.row as usize][to.col as usize] = std::mem::replace(
            &mut self.grid[from.row as usize][from.col as usize],
            Cell::Free,
        );
    }

    fn test_and_push(&mut self, from: &Coord, to: &Coord, color: &Color) {
        if self.is_legal_move(from, to, color) {
            if !is_king_exposed(from, to, color, self) {
                // println!("pushing from: ({}, {}), to: ({}, {})", from.row, from.col, to.row, to.col);
                self.legals_moves.push((*from, *to));
            }
            // println!("king exposed: from: ({}, {}), to: ({}, {})", from.row, from.col, to.row, to.col);
        }
        // println!("illegal move: from: ({}, {}), to: ({}, {})", from.row, from.col, to.row, to.col);
    }

    fn checked_coord(row: i8, col: i8) -> Option<Coord> {
        if (0..8).contains(&row) && (0..8).contains(&col) {
            Some(Coord {
                row: row as u8,
                col: col as u8,
            })
        } else {
            None
        }
    }

    pub fn update_legals_moves(&mut self, color: &Color) {
        self.legals_moves.clear();
        for x in 0..8 {
            for y in 0..8 {
                if self.grid[x][y].is_color(color) {
                    let from = Coord {
                        row: x as u8,
                        col: y as u8,
                    };
                    if let Some(piece) = self.get(&from).get_piece() {
                        match piece {
                            Piece::Pawn => {
                                let dir: i8 = if *color == White { 1 } else { -1 };
                                //2 diagonales
                                if let Some(to) =
                                    Board::checked_coord(from.row as i8 + dir, from.col as i8 + 1)
                                {
                                    self.test_and_push(&from, &to, color);
                                }
                                if let Some(to) =
                                    Board::checked_coord(from.row as i8 + dir, from.col as i8 - 1)
                                {
                                    self.test_and_push(&from, &to, color);
                                }
                                //2 straight forward
                                if let Some(to) =
                                    Board::checked_coord(from.row as i8 + dir, from.col as i8)
                                {
                                    //Si to.row = promote row
                                    //  tester R
                                    //  Tester N
                                    //  Tester B
                                    //  Tester Q
                                    self.test_and_push(&from, &to, color);
                                }
                                if let Some(to) =
                                    Board::checked_coord(from.row as i8 + dir + dir, from.col as i8)
                                {
                                    self.test_and_push(&from, &to, color);
                                }
                            }
                            Piece::Rook => {
                                let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

                                for (dr, dc) in directions {
                                    let mut r = from.row as i8 + dr;
                                    let mut c = from.col as i8 + dc;

                                    while let Some(to) = Board::checked_coord(r, c) {
                                        let target = self.get(&to);

                                        if target.is_color(color) {
                                            break;
                                        }
                                        self.test_and_push(&from, &to, color);

                                        r += dr;
                                        c += dc;
                                    }
                                }
                            }
                            Piece::Knight => {
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

                                for (dr, dc) in cells {
                                    let new_row = from.row as i8 + dr;
                                    let new_col = from.col as i8 + dc;
                                    if let Some(to) = Board::checked_coord(new_row, new_col) {
                                        self.test_and_push(&from, &to, color);
                                    }
                                }
                            }
                            Piece::Bishop => {
                                let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];

                                for (dr, dc) in directions {
                                    let mut r = from.row as i8 + dr;
                                    let mut c = from.col as i8 + dc;

                                    while let Some(to) = Board::checked_coord(r, c) {
                                        let target = self.get(&to);

                                        if target.is_color(color) {
                                            break;
                                        }
                                        self.test_and_push(&from, &to, color);

                                        r += dr;
                                        c += dc;
                                    }
                                }
                            }
                            Piece::Queen => {
                                let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];

                                for (dr, dc) in directions {
                                    let mut r = from.row as i8 + dr;
                                    let mut c = from.col as i8 + dc;

                                    while let Some(to) = Board::checked_coord(r, c) {
                                        let target = self.get(&to);

                                        if target.is_color(color) {
                                            break;
                                        }
                                        self.test_and_push(&from, &to, color);

                                        r += dr;
                                        c += dc;
                                    }
                                }
                                let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

                                for (dr, dc) in directions {
                                    let mut r = from.row as i8 + dr;
                                    let mut c = from.col as i8 + dc;

                                    while let Some(to) = Board::checked_coord(r, c) {
                                        let target = self.get(&to);

                                        if target.is_color(color) {
                                            break;
                                        }
                                        self.test_and_push(&from, &to, color);

                                        r += dr;
                                        c += dc;
                                    }
                                }
                            }
                            Piece::King => {
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

                                for (dr, dc) in cells {
                                    let new_row = from.row as i8 + dr;
                                    let new_col = from.col as i8 + dc;
                                    if let Some(to) = Board::checked_coord(new_row, new_col) {
                                        self.test_and_push(&from, &to, color);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // for (from, to) in &self.legals_moves {
        //     println!("from: ({}, {}), to: ({}, {})", from.row, from.col, to.row, to.col);
        // }
    }
    pub fn promote_pawn(&mut self, color: &Color) {
        use std::io::{self, BufRead}; // en haut ou dans la fonction ?

        let stdin = io::stdin();
        let mut line = String::new();

        let promote_row = if *color == Color::White { 7 } else { 0 };
        for y in 0..8 {
            if self.grid[promote_row][y].is_color(color) {
                if let Some(piece) = self.grid[7][y].get_piece()
                    && *piece == Piece::Pawn
                {
                    println!("Pawn promote : R/N/B/Q");
                    line.clear();
                    loop {
                        if stdin.lock().read_line(&mut line).unwrap() == 0 {
                            println!("EOF");
                            break;
                        }
                        let input = line.trim();
                        if input.len() != 1 {
                            println!(
                                "Incorrect input.\nAllowed inputs : R = Rook | N = Knight | B = Bishop | Q = Queen"
                            );
                            continue;
                        }
                        self.grid[promote_row as usize][y as usize] = match input {
                            "R" => Cell::Occupied(Piece::Rook, *color),
                            "N" => Cell::Occupied(Piece::Knight, *color),
                            "B" => Cell::Occupied(Piece::Bishop, *color),
                            "Q" => Cell::Occupied(Piece::Queen, *color),
                            _ => {
                                println!(
                                    "Incorrect input.\nAllowed inputs : R = Rook | N = Knight | B = Bishop | Q = Queen"
                                );
                                continue;
                            }
                        };
                        let from_row = if *color == Color::White { 6 } else { 1 };
                        self.grid[from_row][y] = Cell::Free;
                        break;
                    }
                }
            }
        }
    }
}
