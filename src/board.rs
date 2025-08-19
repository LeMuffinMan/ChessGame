use crate::Coord;
use crate::validate_move;

#[derive(Copy, Clone, Eq, PartialEq, Debug)] //copy pour initialiser le tableau | copy depend de clone ?
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
use Piece::*;

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum Color {
    Black,
    White,
}
use Color::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Occupied(Piece, Color),
    Free,
}

impl Cell {
    pub fn get_piece(&self) -> Option<&Piece> {
        match self {
            Cell::Occupied(piece, _) => Some(piece),
            Cell::Free => None,
        }
    }
    // pub fn get_color(&self) -> &Color {
    //     match self {
    //         Cell::Occupied(_, color) => Some(color),
    //         Cell::Free => None,
    //     }
    // }
    pub fn is_color(&self, color: &Color) -> bool {
        match self {
            Cell::Occupied(_, cell_color) => cell_color == color,
            Cell::Free => false,
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Cell::Free => true,
            Cell::Occupied(_, _) => true,
        }
    }
    pub fn is_opponent_color(&self, color: &Color) -> bool {
        match self {
            Cell::Free => false,
            Cell::Occupied(_, cell_color) => cell_color != color,
        }
    }
    // pub fn diff_color_and_not_white(&self, color: &Color) -> bool {
    //     match self {
    //         Cell::Free => false,
    //         Cell::Occupied(_, cell_color) => cell_color != color && *cell_color != White,
    //     }
    // }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Cell::Occupied(piece, color) => {
                let piece_char = match piece {
                    Pawn => "p",
                    Rook => "r",
                    Knight => "n",
                    Bishop => "b",
                    Queen => "q",
                    King => "k",
                };
                match color {
                    Black => piece_char.to_uppercase(),
                    White => piece_char.to_string(),
                }
            }
            Cell::Free => String::from(" "),
        };
        write!(f, "{display_str}")
    }
}

// #[derive(Copy, Clone)]
pub struct Board {
    pub grid: [[Cell; 8]; 8],
    // pub white_castle: (bool, bool), //(short, long)
    // pub black_castle: (bool, bool),
    pub threaten_cells: Vec<Coord>, // <=> vec(coord_of_cell_threaten, color_of_player_threatening)
    //each element of the vector is a tuple :
    //- the coord of the threaten cell
    //- the color which is threatening the cell
    pub en_passant: Option<Coord>,
    //Works as a boolean containing a coord if true
    //if an en_passant takes is possible : exists
    //- contains the coord of the pawn exposed to en_passant
    //else : None
    //does not update print correctly
    //does not set back to none correctly

    //check: bool,
    //legal_moves: Vec<(Coord, Coord)>
}

//after updating threats, we check each legal moves for active player
//we compose a vector of tuple (from, to) for each possible move using validate_move
//- if the vec is empty : return is_pat() || is_check_resolved()
//- else we can use the vec to compare user input and reject or accept it
//
//loop {
//
// update_threats()
// update_legal_moves()
//      if vector empty
//         - if check == true => mat
//         - else -> pat
// getinputs()
// is_legal_move()
// update_check()
//
//}
//Validate move :
// - is_legal_move
// - if check == true  && active king is threaten
//    - reject and ask new inputs
//
// Update_check() //at end of turn after validated the move, we update the check bool for next
// player
//   - if check == true -> check = false //we would have rejected a move not solving the check
//   - if opponent king is threathen : check = true
//

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
            // white_castle: (true, true),
            // black_castle: (true, true),
            threaten_cells: Vec::new(),
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

    pub fn is_legal_move(&self, from: &Coord, to: &Coord, color: &Color) -> bool {
        validate_move::is_legal_move(from, to, color, self)
    }

    pub fn get(&self, coord: &Coord) -> Cell {
        self.grid[coord.row as usize][coord.col as usize]
    }
}
