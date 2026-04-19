use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::board::is_king_exposed::is_king_exposed;
use crate::board::move_gen::{CastleSide::*, Move, MoveType::*};

#[derive(Clone, PartialEq)]
pub struct Board {
    pub grid: [[Cell; 8]; 8],
    pub white_castle: CastleRights,
    pub black_castle: CastleRights,
    pub white_king: Coord,
    pub black_king: Coord,
    pub en_passant: Option<Coord>,
    pub check: Option<Coord>,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct CastleRights {
    pub long: bool,
    pub short: bool,
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
        };

        board.fill_side(White);
        board.fill_side(Black);
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

    pub fn build_move(&self, from: Coord, to: Coord, active_player: Color) -> Move {
        let piece_moving = self.grid[from.row as usize][from.col as usize].get_piece();

        let m_type = match piece_moving {
            Some(Pawn) => {
                if to.col != from.col && self.grid[to.row as usize][to.col as usize] == Cell::Free {
                    EnPassant
                } else {
                    Regular
                }
            }
            Some(King) => {
                if (from.col as i8 - to.col as i8).abs() > 1 {
                    if (from.col as i8 - to.col as i8) < 0 {
                        Castle(Right)
                    } else {
                        Castle(Left)
                    }
                } else {
                    Regular
                }
            }
            _ => Regular,
        };

        let m: Move = match m_type {
            Regular => Move {
                dest: to,
                origin: from,
                capture: self.grid[to.row as usize][to.col as usize],
                en_passant: self.en_passant,
                check: self.check,
                white_castle: self.white_castle,
                black_castle: self.black_castle,
                move_type: m_type,
            },
            EnPassant => match active_player {
                White => Move {
                    dest: to,
                    origin: from,
                    capture: self.grid[(to.row - 1) as usize][to.col as usize],
                    en_passant: self.en_passant,
                    check: self.check,
                    white_castle: self.white_castle,
                    black_castle: self.black_castle,
                    move_type: m_type,
                },
                Black => Move {
                    dest: to,
                    origin: from,
                    capture: self.grid[(to.row + 1) as usize][to.col as usize],
                    en_passant: self.en_passant,
                    check: self.check,
                    white_castle: self.white_castle,
                    black_castle: self.black_castle,
                    move_type: m_type,
                },
            },
            _ => Move {
                dest: to,
                origin: from,
                capture: self.grid[to.row as usize][to.col as usize],
                en_passant: self.en_passant,
                check: self.check,
                white_castle: self.white_castle,
                black_castle: self.black_castle,
                move_type: m_type,
            }, // castle right / left
        };
        return m;
    }

    pub fn apply_move(&mut self, m: &Move, active_player: Color) {
        self.en_passant = None;
        self.check = None;
        match self.get(&m.origin).get_piece() {
            Some(piece) => match piece {
                Pawn => {
                    self.update_en_passant(&m.origin, &m.dest);
                    if m.move_type == EnPassant {
                        match active_player {
                            White => {
                                self.grid[(m.dest.row - 1) as usize][m.dest.col as usize] =
                                    Cell::Free
                            }
                            Black => {
                                self.grid[(m.dest.row + 1) as usize][m.dest.col as usize] =
                                    Cell::Free
                            }
                        }
                    }
                }
                King => {
                    self.update_king_castle(&m.origin, &m.dest, &active_player);
                    match active_player {
                        White => self.white_king = m.dest,
                        Black => self.black_king = m.dest,
                    }
                }
                Knight | Rook | Queen | Bishop => {}
            },
            None => {
                println!("Error : update board : from cell empty")
            }
        }
        self.grid[m.dest.row as usize][m.dest.col as usize] = std::mem::replace(
            &mut self.grid[m.origin.row as usize][m.origin.col as usize],
            Cell::Free,
        );
        if let Promotion(promoted) = m.move_type {
            self.grid[m.dest.row as usize][m.dest.col as usize] =
                Cell::Occupied(promoted, active_player);
        }
    }

    pub fn undo_move(&mut self, m: Move, active_player: Color) {
        match m.move_type {
            EnPassant => {
                self.grid[m.dest.row as usize][m.dest.col as usize] = Cell::Free;
                match active_player {
                    White => {
                        self.grid[(m.dest.row - 1) as usize][m.dest.col as usize] = m.capture;
                        self.grid[m.origin.row as usize][m.origin.col as usize] =
                            Cell::Occupied(Pawn, active_player);
                    }
                    Black => {
                        self.grid[(m.dest.row + 1) as usize][m.dest.col as usize] = m.capture;
                        self.grid[m.origin.row as usize][m.origin.col as usize] =
                            Cell::Occupied(Pawn, active_player);
                    }
                }
            }
            Castle(side) => match side {
                Right => match active_player {
                    White => {
                        self.grid[0][4] = Cell::Occupied(King, White);
                        self.grid[0][5] = Cell::Free;
                        self.grid[0][6] = Cell::Free;
                        self.grid[0][7] = Cell::Occupied(Rook, White);
                        self.white_king = Coord { row: 0, col: 4 };
                    }
                    Black => {
                        self.grid[7][4] = Cell::Occupied(King, Black);
                        self.grid[7][5] = Cell::Free;
                        self.grid[7][6] = Cell::Free;
                        self.grid[7][7] = Cell::Occupied(Rook, Black);
                        self.black_king = Coord { row: 7, col: 4 };
                    }
                },
                Left => match active_player {
                    White => {
                        self.grid[0][4] = Cell::Occupied(King, White);
                        self.grid[0][3] = Cell::Free;
                        self.grid[0][2] = Cell::Free;
                        self.grid[0][1] = Cell::Free;
                        self.grid[0][0] = Cell::Occupied(Rook, White);
                        self.white_king = Coord { row: 0, col: 4 };
                    }
                    Black => {
                        self.grid[7][4] = Cell::Occupied(King, Black);
                        self.grid[7][3] = Cell::Free;
                        self.grid[7][2] = Cell::Free;
                        self.grid[7][1] = Cell::Free;
                        self.grid[7][0] = Cell::Occupied(Rook, Black);
                        self.black_king = Coord { row: 7, col: 4 };
                    }
                },
            },
            Promotion(_) => {
                self.grid[m.origin.row as usize][m.origin.col as usize] =
                    Cell::Occupied(Pawn, active_player);
                self.grid[m.dest.row as usize][m.dest.col as usize] = m.capture;
            }
            Regular => {
                self.grid[m.origin.row as usize][m.origin.col as usize] = self.get(&m.dest);
                if self.grid[m.origin.row as usize][m.origin.col as usize].get_piece()
                    == Some(&King)
                {
                    match active_player {
                        White => self.white_king = m.origin,
                        Black => self.black_king = m.origin,
                    }
                }
                self.grid[m.dest.row as usize][m.dest.col as usize] = m.capture;
            }
        }
        self.en_passant = m.en_passant;
        self.check = m.check;
        self.white_castle = m.white_castle;
        self.black_castle = m.black_castle;
    }
    pub fn check_move(
        &mut self,
        origin: &Coord,
        dest: &Coord,
        active_player: &Color,
    ) -> Option<Move> {
        if self.get(dest).is_color(active_player) {
            return None;
        }
        let m = self.build_move(*origin, *dest, *active_player);
        self.apply_move(&m, *active_player);
        let exposed = is_king_exposed(self, active_player);
        self.undo_move(m, *active_player);
        if !exposed {
            return Some(m);
        }
        None
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

    pub fn update_en_passant(&mut self, from: &Coord, to: &Coord) {
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
        if dif_col == -2 {
            let col = to.col as usize;
            if col > 0 {
                self.grid[row][0] = Cell::Free;
                self.grid[row][col + 1] = Cell::Occupied(Rook, *color);
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
