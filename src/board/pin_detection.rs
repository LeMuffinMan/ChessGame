use crate::Board;
use crate::board::cell::Cell::{Free, Occupied};
use crate::board::cell::Color;
use crate::board::cell::Coord;
use crate::board::cell::Piece::{Bishop, Knight, Pawn, Queen, Rook};

pub struct PinInfos {
    pub pins: [[Option<(i8, i8)>; 8]; 8],
    pub checker_count: u8,
    pub checkers: [Coord; 2],
}

impl PinInfos {
    fn new() -> Self {
        Self {
            pins: [[None; 8]; 8],
            checker_count: 0,
            checkers: [Coord::default(); 2],
        }
    }

    fn add_checker(&mut self, c: Coord) {
        if self.checker_count < 2 {
            self.checkers[self.checker_count as usize] = c;
        }
        self.checker_count += 1;
    }
}

pub fn pin_detection(board: &Board, color: Color) -> PinInfos {
    let mut info = PinInfos::new();

    let king = match color {
        Color::White => board.white_king,
        Color::Black => board.black_king,
    };

    let directions: [(i8, i8); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for (dr, dc) in directions {
        let mut r = king.row as i8 + dr;
        let mut c = king.col as i8 + dc;
        let mut pin_candidate: Option<Coord> = None;

        while let Some(dest) = Board::checked_coord(r, c) {
            match board.grid[dest.row as usize][dest.col as usize] {
                Free => {}
                Occupied(piece, cell_color) => {
                    if cell_color == color {
                        if pin_candidate.is_none() {
                            pin_candidate = Some(dest);
                        } else {
                            break;
                        }
                    } else {
                        let is_diag = dr != 0 && dc != 0;
                        let can_attack_along_ray = match piece {
                            Queen => true,
                            Rook => !is_diag,
                            Bishop => is_diag,
                            _ => false,
                        };

                        if can_attack_along_ray {
                            match pin_candidate {
                                Some(cand) => {
                                    info.pins[cand.row as usize][cand.col as usize] =
                                        Some((dr, dc));
                                }
                                None => {
                                    info.add_checker(dest);
                                }
                            }
                        }
                        break;
                    }
                }
            }
            r += dr;
            c += dc;
        }
    }

    let knight_offsets: [(i8, i8); 8] = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];
    for (dr, dc) in knight_offsets {
        let r = king.row as i8 + dr;
        let c = king.col as i8 + dc;
        if let Some(dest) = Board::checked_coord(r, c) {
            if let Occupied(Knight, cell_color) = board.grid[dest.row as usize][dest.col as usize] {
                if cell_color != color {
                    info.add_checker(dest);
                }
            }
        }
    }

    let pawn_dir: i8 = match color {
        Color::White => 1,
        Color::Black => -1,
    };
    for dc in [1i8, -1] {
        let r = king.row as i8 + pawn_dir;
        let c = king.col as i8 + dc;
        if let Some(dest) = Board::checked_coord(r, c) {
            if let Occupied(Pawn, cell_color) = board.grid[dest.row as usize][dest.col as usize] {
                if cell_color != color {
                    info.add_checker(dest);
                }
            }
        }
    }

    info
}
