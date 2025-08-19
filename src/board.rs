use crate::Coord; // Mettre Coord ici ?

#[derive(Copy, Clone, PartialEq, Debug)] //copy pour initialiser le tableau | copy depend de clone ?
pub enum Pieces {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    QUEEN,
    KING,
    NONE, //Possile de supprimer none en utilisant Option<T> ?
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    BLACK,
    WHITE,
    NONE,
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub piece: Pieces,
    pub color: Color,
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
    pub fn init_board() -> Board {
        let empty_cell = Cell {
            piece: Pieces::NONE,
            color: Color::NONE,
        };
        let mut board = Board {
            grid: [[empty_cell; 8]; 8],
            white_long_castle: true,
            white_short_castle: true,
            black_long_castle: true,
            black_short_castle: true,
            en_passant: None,
            white_threatening_cells: Vec::new(),
            black_threatening_cells: Vec::new(),
        };
        for y in 0..8 {
            for x in 0..8 {
                board.grid[y][x] = match y {
                    0 => match x {
                        //pour la ligne tout en bas
                        0 | 7 => Cell {
                            piece: Pieces::ROOK,
                            color: Color::WHITE,
                        },
                        1 | 6 => Cell {
                            piece: Pieces::KNIGHT,
                            color: Color::WHITE,
                        },
                        2 | 5 => Cell {
                            piece: Pieces::BISHOP,
                            color: Color::WHITE,
                        },
                        3 => Cell {
                            piece: Pieces::QUEEN,
                            color: Color::WHITE,
                        },
                        4 => Cell {
                            piece: Pieces::KING,
                            color: Color::WHITE,
                        },
                        _ => empty_cell, //cas a couvrir par defaut mais impossible car board 8x8
                    },
                    1 => Cell {
                        piece: Pieces::PAWN,
                        color: Color::WHITE,
                    },
                    6 => Cell {
                        piece: Pieces::PAWN,
                        color: Color::BLACK,
                    },
                    7 => match x {
                        0 | 7 => Cell {
                            piece: Pieces::ROOK,
                            color: Color::BLACK,
                        },
                        1 | 6 => Cell {
                            piece: Pieces::KNIGHT,
                            color: Color::BLACK,
                        },
                        2 | 5 => Cell {
                            piece: Pieces::BISHOP,
                            color: Color::BLACK,
                        },
                        3 => Cell {
                            piece: Pieces::KING,
                            color: Color::BLACK,
                        },
                        4 => Cell {
                            piece: Pieces::QUEEN,
                            color: Color::BLACK,
                        },
                        _ => empty_cell, //cas a couvrir par defaut mais impossible car board 8x8
                    },
                    _ => empty_cell,
                };
            }
        }
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
                let c = match self.grid[y][x].piece {
                    Pieces::PAWN => match self.grid[y][x].color {
                        Color::WHITE => "♟",
                        Color::BLACK => "♙",
                        _ => "?",
                    },
                    Pieces::ROOK => match self.grid[y][x].color {
                        Color::WHITE => "♜",
                        Color::BLACK => "♖",
                        _ => "?",
                    },
                    Pieces::KNIGHT => match self.grid[y][x].color {
                        Color::WHITE => "♞",
                        Color::BLACK => "♘",
                        _ => "?",
                    },
                    Pieces::BISHOP => match self.grid[y][x].color {
                        Color::WHITE => "♝",
                        Color::BLACK => "♗",
                        _ => "?",
                    },
                    Pieces::QUEEN => match self.grid[y][x].color {
                        Color::WHITE => "♛",
                        Color::BLACK => "♕",
                        _ => "?",
                    },
                    Pieces::KING => match self.grid[y][x].color {
                        Color::WHITE => "♚",
                        Color::BLACK => "♔",
                        _ => "?",
                    },
                    Pieces::NONE => " ",
                };
                print!("| {} ", c);
            }
            println!("|");
        }
        print!("  ");
        for _ in 0..8 {
            print!("----");
        }
        println!();
    }
}

