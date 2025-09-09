use crate::Board;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::cell::Piece;
use crate::validate_move::is_legal_move::is_king_exposed;

impl Board {
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
                            Piece::Pawn => self.update_pawn_legals_moves(&from, color),
                            Piece::Rook => self.update_rook_legals_moves(&from, color),
                            Piece::Knight => self.update_knight_legals_moves(&from, color),
                            Piece::Bishop => self.update_bishop_legals_moves(&from, color),
                            Piece::Queen => self.update_queen_legals_moves(&from, color),
                            Piece::King => self.update_king_legals_moves(&from, color),
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

    fn update_pawn_legals_moves(&mut self, from: &Coord, color: &Color) {
        let dir: i8 = if *color == White { 1 } else { -1 };
        //2 diagonales
        if let Some(to) = Board::checked_coord(from.row as i8 + dir, from.col as i8 + 1) {
            self.test_and_push(from, &to, color);
        }
        if let Some(to) = Board::checked_coord(from.row as i8 + dir, from.col as i8 - 1) {
            self.test_and_push(from, &to, color);
        }
        //2 straight forward
        if let Some(to) = Board::checked_coord(from.row as i8 + dir, from.col as i8) {
            self.test_and_push(from, &to, color);
        }
        if let Some(to) = Board::checked_coord(from.row as i8 + dir + dir, from.col as i8) {
            self.test_and_push(from, &to, color);
        }
    }

    fn update_rook_legals_moves(&mut self, from: &Coord, color: &Color) {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for (dr, dc) in directions {
            let mut r = from.row as i8 + dr;
            let mut c = from.col as i8 + dc;

            while let Some(to) = Board::checked_coord(r, c) {
                let target = self.get(&to);

                if target.is_color(color) {
                    break;
                }
                self.test_and_push(from, &to, color);

                r += dr;
                c += dc;
            }
        }
    }

    fn update_knight_legals_moves(&mut self, from: &Coord, color: &Color) {
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
                self.test_and_push(from, &to, color);
            }
        }
    }

    fn update_bishop_legals_moves(&mut self, from: &Coord, color: &Color) {
        let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];

        for (dr, dc) in directions {
            let mut r = from.row as i8 + dr;
            let mut c = from.col as i8 + dc;

            while let Some(to) = Board::checked_coord(r, c) {
                let target = self.get(&to);

                if target.is_color(color) {
                    break;
                }
                self.test_and_push(from, &to, color);

                r += dr;
                c += dc;
            }
        }
    }

    fn update_queen_legals_moves(&mut self, from: &Coord, color: &Color) {
        let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];

        for (dr, dc) in directions {
            let mut r = from.row as i8 + dr;
            let mut c = from.col as i8 + dc;

            while let Some(to) = Board::checked_coord(r, c) {
                let target = self.get(&to);

                if target.is_color(color) {
                    break;
                }
                self.test_and_push(from, &to, color);

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
                self.test_and_push(from, &to, color);

                r += dr;
                c += dc;
            }
        }
    }

    fn update_king_legals_moves(&mut self, from: &Coord, color: &Color) {
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
                self.test_and_push(from, &to, color);
            }
        }
    }
}
