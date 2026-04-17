use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;

#[derive(Clone, PartialEq)]
pub struct Board {
    pub grid: [[Cell; 8]; 8],
    pub white_castle: (bool, bool), //(long, short)
    pub black_castle: (bool, bool),
    pub threaten_cells: Vec<Coord>,
    pub legals_moves: Vec<(Coord, Coord)>,
    pub en_passant: Option<Coord>,
    pub check: Option<Coord>,
    pub pawn_to_promote: Option<Coord>,
    pub promote: Option<Piece>,
}

//cancastle
// left
// right
// leftandright

impl Board {
    //Considering to move from GameState, into board :
    //  - Active player / opponent color
    //  - end state
    //  - turn
    pub fn init_board() -> Board {
        let mut board = Board {
            grid: [[Cell::Free; 8]; 8],
            en_passant: None,
            white_castle: (true, true),
            black_castle: (true, true),
            threaten_cells: Vec::new(),
            legals_moves: Vec::new(),
            check: None,
            pawn_to_promote: None,
            promote: None,
        };

        board.fill_side(White);
        board.fill_side(Black);
        board.update_legals_moves(&White);
        board.update_threatens_cells(&White);

        board
    }

    //Setting up basic chess board
    pub fn fill_side(&mut self, color: Color) {
        let color_idx = match color {
            White => 0,
            Black => 7,
        };
        for x in 0..8 {
            // fill the base line
            self.grid[color_idx][x] = match x {
                0 | 7 => Cell::Occupied(Rook, color),
                1 | 6 => Cell::Occupied(Knight, color),
                2 | 5 => Cell::Occupied(Bishop, color),
                3 => Cell::Occupied(Queen, color),
                4 => Cell::Occupied(King, color),
                _ => unreachable!(),
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

    //Since we need player input to know in which piece promote a pawn, i need to
    //store the coord of the pawn to promote and stop the try move process
    //the GUI will hook on the coord position stored and force player to input a desired promotion
    //Then this hook process the end of try move we skipped earlier
    pub fn promote_pawn(&mut self, color: &Color) {
        let promote_row = if *color == White { 7 } else { 0 };
        for y in 0..8 {
            if self.grid[promote_row][y].is_color(color)
                && let Some(piece) = self.grid[promote_row][y].get_piece()
                && *piece == Pawn
            {
                let coord = Coord {
                    row: promote_row as u8,
                    col: y as u8,
                };
                self.pawn_to_promote = Some(coord);
            }
        }
    }

    pub fn build_move(self, from: Coord, to: Coord) -> Move {
        let m: Move;

        let piece_moving = self.getPiece(from);
        let color_moving = self.getcolor(from);

        m.origin = from;
        m.dest = to;
        m.capture = self.getPiece(to);
        m.en_passant = self.en_passant;
        m.white_castle = self.white_castle;
        m.black_castle = self.black_castle;
        m.move_type = match piece_moving {
            Pawn => {
                if (to.row - from.row).abs() == 2 {
                    //un double saut ou une promotion
                    EnPassant
                } else {
                    Regular
                }
            }
            King => {
                //ca casserai les castles
                if (from.col - to.col).abs() > 1 {
                    //if it's a king moving more than 1 cell
                    if from.col - to.col < 0 {
                        //if the king is moving right
                        Castle(Right)
                    } else {
                        Castle(Left)
                    }
                } else {
                    Regular
                }
            }
            _ => Regular,
        }
    }

    pub fn apply_move(self, m: Move) {
        self.board.update_board(from, to, color);
        //refacto update_board et la remplacer par apply_move ?
    }

    pub fn undo_move(self, m: Move) {

        //Dans quelle situation j'ai besoin de move_type, je peux juste override board avec les datas de move ?
        // match m.move_type {
        //     EnPassant => {

        //     },
        //     Castle(side) => {
        //         match side {
        //             Right => {},
        //             Left => {},
        //         }
        //     }
        //     Regular => {

        //     },
        // }

        self.board.getPiece(m.origin) = self.board.getPiece(m.dest);
        self.board.setPiece(m.dest) = m.capture;
        self.board.en_passant = m.en_passant;
        self.board.white_castle = m.white_castle;
        self.board.black_castle = m.black_castle;
    }
}
