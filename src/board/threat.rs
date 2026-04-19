use crate::Board;
use crate::Color;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::cell::Color::*;
use crate::board::cell::Piece::*;

impl Board {
    pub fn update_threatens_cells(&mut self, color: &Color) -> Vec<Coord> {
        let mut threaten_cells: Vec<Coord> = Vec::new();

        for row in 0..8 {
            for col in 0..8 {
                let coord = Coord { row: row as u8, col: col as u8 };
                let cell = self.get(&coord);
                if let Some(piece) = cell.get_piece()
                    && !cell.is_color(color)
                {
                    match piece {
                        Pawn   => pawn_threats(&cell, &coord, color, &mut threaten_cells),
                        Rook   => rook_threats(&coord, row, col, self, &mut threaten_cells),
                        Knight => knight_threats(&coord, color, &cell, &mut threaten_cells),
                        Bishop => bishop_threats(&coord, row, col, self, &mut threaten_cells),
                        Queen  => {
                            rook_threats(&coord, row, col, self, &mut threaten_cells);
                            bishop_threats(&coord, row, col, self, &mut threaten_cells);
                        }
                        King => king_threats(&coord, &mut threaten_cells),
                    }
                }
            }
        }
        threaten_cells
    }
}

fn pawn_threats(cell: &Cell, coord: &Coord, color: &Color, threaten_cells: &mut Vec<Coord>) {
    let offsets: [(i8, i8); 2] = if cell.is_color(&White) { [(1, -1), (1, 1)] } else { [(-1, -1), (-1, 1)] };
    for (dr, dc) in offsets {
        let r = coord.row as i8 + dr;
        let c = coord.col as i8 + dc;
        if (0..8).contains(&r) && (0..8).contains(&c) && !cell.is_color(color) {
            threaten_cells.push(Coord { row: r as u8, col: c as u8 });
        }
    }
}

fn rook_threats(coord: &Coord, row: usize, col: usize, board: &mut Board, threaten_cells: &mut Vec<Coord>) {
    if row < 7 { threaten_in_line(coord, row as u8 + 1, col as u8,     board, threaten_cells); }
    if row > 0 { threaten_in_line(coord, row as u8 - 1, col as u8,     board, threaten_cells); }
    if col < 7 { threaten_in_line(coord, row as u8,     col as u8 + 1, board, threaten_cells); }
    if col > 0 { threaten_in_line(coord, row as u8,     col as u8 - 1, board, threaten_cells); }
}

fn knight_threats(coord: &Coord, color: &Color, cell: &Cell, threaten_cells: &mut Vec<Coord>) {
    let offsets: [(i8, i8); 8] = [(2,1),(2,-1),(-2,1),(-2,-1),(1,2),(1,-2),(-1,2),(-1,-2)];
    for (dr, dc) in offsets {
        let r = coord.row as i8 + dr;
        let c = coord.col as i8 + dc;
        if (0..8).contains(&r) && (0..8).contains(&c) && !cell.is_color(color) {
            threaten_cells.push(Coord { row: r as u8, col: c as u8 });
        }
    }
}

fn bishop_threats(coord: &Coord, row: usize, col: usize, board: &mut Board, threaten_cells: &mut Vec<Coord>) {
    if row < 7 && col < 7 { threaten_in_diag(coord, row as u8 + 1, col as u8 + 1, board, threaten_cells); }
    if row < 7 && col > 0 { threaten_in_diag(coord, row as u8 + 1, col as u8 - 1, board, threaten_cells); }
    if row > 0 && col < 7 { threaten_in_diag(coord, row as u8 - 1, col as u8 + 1, board, threaten_cells); }
    if row > 0 && col > 0 { threaten_in_diag(coord, row as u8 - 1, col as u8 - 1, board, threaten_cells); }
}

fn king_threats(coord: &Coord, threaten_cells: &mut Vec<Coord>) {
    let offsets: [(i8, i8); 8] = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
    for (dr, dc) in offsets {
        let r = coord.row as i8 + dr;
        let c = coord.col as i8 + dc;
        if (0..8).contains(&r) && (0..8).contains(&c) {
            threaten_cells.push(Coord { row: r as u8, col: c as u8 });
        }
    }
}

fn threaten_in_diag(from: &Coord, row: u8, col: u8, board: &mut Board, threaten_cells: &mut Vec<Coord>) {
    if row > 7 || col > 7 { return; }
    let target = Coord { row, col };
    threaten_cells.push(target);
    if !board.get(&target).is_empty() { return; }
    match (row.cmp(&from.row), col.cmp(&from.col)) {
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) if row < 7 && col < 7 =>
            threaten_in_diag(from, row + 1, col + 1, board, threaten_cells),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) if row < 7 && col > 0 =>
            threaten_in_diag(from, row + 1, col - 1, board, threaten_cells),
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) if row > 0 && col < 7 =>
            threaten_in_diag(from, row - 1, col + 1, board, threaten_cells),
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) if row > 0 && col > 0 =>
            threaten_in_diag(from, row - 1, col - 1, board, threaten_cells),
        _ => {}
    }
}

fn threaten_in_line(from: &Coord, row: u8, col: u8, board: &mut Board, threaten_cells: &mut Vec<Coord>) {
    if row > 7 || col > 7 { return; }
    let target = Coord { row, col };
    threaten_cells.push(target);
    if !board.get(&target).is_empty() { return; }
    match (row.cmp(&from.row), col.cmp(&from.col)) {
        (std::cmp::Ordering::Greater, _) if row < 7 => threaten_in_line(from, row + 1, col, board, threaten_cells),
        (std::cmp::Ordering::Less,    _) if row > 0 => threaten_in_line(from, row - 1, col, board, threaten_cells),
        (_, std::cmp::Ordering::Greater) if col < 7 => threaten_in_line(from, row, col + 1, board, threaten_cells),
        (_, std::cmp::Ordering::Less)    if col > 0 => threaten_in_line(from, row, col - 1, board, threaten_cells),
        _ => {}
    }
}
