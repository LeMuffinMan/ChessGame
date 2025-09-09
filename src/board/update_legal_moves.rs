use crate::Board;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::cell::Piece;
use crate::validate_move::is_legal_move::is_king_exposed;

impl Board {
    fn test_and_push(&mut self, from: &Coord, to: &Coord, color: &Color) -> Option<(Coord, Coord)> {
        if self.is_legal_move(from, to, color) {
            if !is_king_exposed(from, to, color, self) {
                // println!("pushing from: ({}, {}), to: ({}, {})", from.row, from.col, to.row, to.col);
                return Some((*from, *to));
                // self.legals_moves.push((*from, *to));
            }
            // println!("king exposed: from: ({}, {}), to: ({}, {})", from.row, from.col, to.row, to.col);
        }
        // println!("illegal move: from: ({}, {}), to: ({}, {})", from.row, from.col, to.row, to.col);
        return None;
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
                                let vec = update_pawn_legals_moves(&from, color, self);
                                self.legals_moves.extend(vec);
                            }
                            Piece::Rook => {
                                let vec = update_rook_legals_moves(&from, color, self);
                                self.legals_moves.extend(vec);
                            }
                            Piece::Knight => {
                                let vec = update_knight_legals_moves(&from, color, self);
                                self.legals_moves.extend(vec);
                            }
                            Piece::Bishop => {
                                let vec = update_bishop_legals_moves(&from, color, self);
                                self.legals_moves.extend(vec);
                            }
                            Piece::Queen => {
                                let vec = update_queen_legals_moves(&from, color, self);
                                self.legals_moves.extend(vec);
                            }
                            Piece::King => { 
                                let vec = update_king_legals_moves(&from, color, self);
                                self.legals_moves.extend(vec);
                            }
                        }
                    }
                }
            }
        }
        println!("Legals moves : ");
        for (from, to) in &self.legals_moves {
            println!(
                "from: ({}, {}), to: ({}, {})",
                from.row, from.col, to.row, to.col
            );
        }
    }
}

pub fn update_pawn_legals_moves(from: &Coord, color: &Color, board: &mut Board) -> Vec<(Coord, Coord)> {
    let dir: i8 = if *color == White { 1 } else { -1 };
    let mut ret = Vec::new();
    //2 diagonales
    if let Some(to) = Board::checked_coord(from.row as i8 + dir, from.col as i8 + 1) {
        if let Some ((_, _)) = board.test_and_push(from, &to, color) {
            ret.push((*from, to));
        }
    }
    if let Some(to) = Board::checked_coord(from.row as i8 + dir, from.col as i8 - 1) {
        if let Some ((_, _)) = board.test_and_push(from, &to, color) {
            ret.push((*from, to));
        }
    }
    //2 straight forward
    if let Some(to) = Board::checked_coord(from.row as i8 + dir, from.col as i8) {
        if let Some ((_, _)) = board.test_and_push(from, &to, color) {
            ret.push((*from, to));
        }
    }
    if let Some(to) = Board::checked_coord(from.row as i8 + dir + dir, from.col as i8) {
        if let Some ((_, _)) = board.test_and_push(from, &to, color) {
            ret.push((*from, to));
        }
    }
    return ret;
}

pub fn update_rook_legals_moves(from: &Coord, color: &Color, board: &mut Board) -> Vec<(Coord, Coord)>{
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut ret = Vec::new();

    for (dr, dc) in directions {
        let mut r = from.row as i8 + dr;
        let mut c = from.col as i8 + dc;

        while let Some(to) = Board::checked_coord(r, c) {
            let target = board.get(&to);

            if target.is_color(color) {
                break;
            }
            if let Some ((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }

            r += dr;
            c += dc;
        }
    }
    return ret;
}

pub fn update_knight_legals_moves(from: &Coord, color: &Color, board: &mut Board) -> Vec<(Coord, Coord)>{
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
    let mut ret = Vec::new();

    for (dr, dc) in cells {
        let new_row = from.row as i8 + dr;
        let new_col = from.col as i8 + dc;
        if let Some(to) = Board::checked_coord(new_row, new_col) {
            if let Some ((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }
        }
    }
    return ret;
}

pub fn update_bishop_legals_moves(from: &Coord, color: &Color, board: &mut Board) -> Vec<(Coord, Coord)>{
    let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
    let mut ret = Vec::new();

    for (dr, dc) in directions {
        let mut r = from.row as i8 + dr;
        let mut c = from.col as i8 + dc;

        while let Some(to) = Board::checked_coord(r, c) {
            let target = board.get(&to);

            if target.is_color(color) {
                break;
            }
            if let Some ((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }

            r += dr;
            c += dc;
        }
    }
    return ret;
}

pub fn update_queen_legals_moves(from: &Coord, color: &Color, board: &mut Board) -> Vec<(Coord, Coord)>{
    let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
    let mut ret = Vec::new();

    for (dr, dc) in directions {
        let mut r = from.row as i8 + dr;
        let mut c = from.col as i8 + dc;

        while let Some(to) = Board::checked_coord(r, c) {
            let target = board.get(&to);

            if target.is_color(color) {
                break;
            }
            if let Some ((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }

            r += dr;
            c += dc;
        }
    }
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (dr, dc) in directions {
        let mut r = from.row as i8 + dr;
        let mut c = from.col as i8 + dc;

        while let Some(to) = Board::checked_coord(r, c) {
            let target = board.get(&to);

            if target.is_color(color) {
                break;
            }
            if let Some ((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }

            r += dr;
            c += dc;
        }
    }
    return ret;
}

pub fn update_king_legals_moves(from: &Coord, color: &Color, board: &mut Board) -> Vec<(Coord, Coord)>{
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
    let mut ret = Vec::new();

    for (dr, dc) in cells {
        let new_row = from.row as i8 + dr;
        let new_col = from.col as i8 + dc;
        if let Some(to) = Board::checked_coord(new_row, new_col) {
            if let Some ((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }
        }
    }
    return ret;
}


