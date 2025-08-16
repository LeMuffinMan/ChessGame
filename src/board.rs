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

pub struct Board {
    pub grid: [[Cell; 8]; 8],
    // check: bool,
    // pat: bool,
    // mate: bool,
}

impl Board {
    pub fn init_board() -> Board {
        let empty_cell = Cell {
            piece: Pieces::NONE,
            color: Color::NONE,
        };
        let mut board = Board {
            grid: [[empty_cell; 8]; 8],
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
                        Color::WHITE => "p",
                        Color::BLACK => "P",
                        _ => "?",
                    },
                    Pieces::ROOK => match self.grid[y][x].color {
                        Color::WHITE => "r",
                        Color::BLACK => "R",
                        _ => "?",
                    },
                    Pieces::KNIGHT => match self.grid[y][x].color {
                        Color::WHITE => "n",
                        Color::BLACK => "N",
                        _ => "?",
                    },
                    Pieces::BISHOP => match self.grid[y][x].color {
                        Color::WHITE => "b",
                        Color::BLACK => "B",
                        _ => "?",
                    },
                    Pieces::QUEEN => match self.grid[y][x].color {
                        Color::WHITE => "q",
                        Color::BLACK => "Q",
                        _ => "?",
                    },
                    Pieces::KING => match self.grid[y][x].color {
                        Color::WHITE => "k",
                        Color::BLACK => "K",
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

