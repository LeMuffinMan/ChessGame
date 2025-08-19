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
    pub fn is_color(&self, color: &Color) -> bool {
        match self {
            Cell::Occupied(_, cell_color) => cell_color == color,
            Cell::Free => false,
        }
    }
    pub fn diff_color_and_not_white(&self, color: &Color) -> bool {
        match self {
            Cell::Free => false,
            Cell::Occupied(_, cell_color) => cell_color != color && *cell_color != White,
        }
    }
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
        write!(f, "{}", display_str)
    }
}

// #[derive(Copy, Clone)]
pub struct Board {
    pub grid: [[Cell; 8]; 8],
    pub en_passant: Option<Coord>,

    //en passant :
    //
    //Une Option<T>
    //Si Some() ne trouve pas None, ca veut dire que la prise en passant est possible, a la coord T
    //Si Some() trouve NONE c'est qu'il n'y a pas de en passant possible
    //On set T aux coordonees du pion qui vient de rendre possible la prise en passant
    //En fin de tour, on met T a None


    // check: bool,
    // pat: bool,
    // mate: bool,
    // 
    // Des que je valide un move pour le roi ou une des tours : on passe ce bool a false
    // Si le coup correspond a un des roques, on check le bool ici
    pub white_long_castle: bool,
    pub white_short_castle: bool,
    pub black_long_castle: bool,
    pub black_short_castle: bool,
    pub white_threatening_cells: Vec<Coord>,
    pub black_threatening_cells: Vec<Coord>,
}

impl Board {
    fn fill_side(&mut self, color: Color) {
        let color_idx = match color {
            White => 0,
            Black => 6,
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
            self.grid[color_idx + 1][x] = Cell::Occupied(Pawn, color);
        }
    }

    pub fn init_board() -> Board {
        let mut board = Board {
            grid: [[Cell::Free; 8]; 8],
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
