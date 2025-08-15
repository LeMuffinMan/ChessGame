
#[derive(Copy, Clone)] //copy pour initialiser le tableau | copy depend de clone ?
enum Pieces {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    QUEEN,
    KING,
    NONE,
}

#[derive(Copy, Clone)] //copy pour initialiser le tableau | copy depend de clone ?
enum Color {
    BLACK,
    WHITE,
    NONE,
}

#[derive(Copy, Clone)] //copy pour initialiser le tableau | copy depend de clone ?
struct Cell {
    piece: Pieces,
    color: Color,
}

struct Board {
    grid: [[Cell; 8]; 8],
    // check: bool,
    // pat: bool,
    // mate: bool,
}

impl Board {
    fn init_board() -> Board {
        let empty_cell = Cell {
            piece: Pieces::NONE, 
            color: Color::NONE,
        };
        let mut board = Board {
            grid: [[empty_cell; 8]; 8],
        };
        for x in 0..8 {
            if x == 0 {
                board.grid[x][0] = Cell { piece: Pieces::ROOK, color: Color::BLACK };
                board.grid[x][1] = Cell { piece: Pieces::KNIGHT, color: Color::BLACK };
                board.grid[x][2] = Cell { piece: Pieces::BISHOP, color: Color::BLACK };
                board.grid[x][3] = Cell { piece: Pieces::KING, color: Color::BLACK };
                board.grid[x][4] = Cell { piece: Pieces::QUEEN, color: Color::BLACK };
                board.grid[x][5] = Cell { piece: Pieces::BISHOP, color: Color::BLACK };
                board.grid[x][6] = Cell { piece: Pieces::KNIGHT, color: Color::BLACK };
                board.grid[x][7] = Cell { piece: Pieces::ROOK, color: Color::BLACK };
                for y in 0..8 {
                    board.grid[x][y] = Cell { piece: Pieces::NONE, color: Color::BLACK };
                }
            }
            if x == 7 {
                board.grid[x][0] = Cell { piece: Pieces::ROOK, color: Color::WHITE };
                board.grid[x][1] = Cell { piece: Pieces::KNIGHT, color: Color::WHITE };
                board.grid[x][2] = Cell { piece: Pieces::BISHOP, color: Color::WHITE };
                board.grid[x][3] = Cell { piece: Pieces::KING, color: Color::WHITE };
                board.grid[x][4] = Cell { piece: Pieces::QUEEN, color: Color::WHITE };
                board.grid[x][5] = Cell { piece: Pieces::BISHOP, color: Color::WHITE };
                board.grid[x][6] = Cell { piece: Pieces::KNIGHT, color: Color::WHITE };
                board.grid[x][7] = Cell { piece: Pieces::ROOK, color: Color::WHITE };
                for y in 0..8 {
                    board.grid[x][y] = Cell { piece: Pieces::PAWN, color: Color::WHITE };
                }
            }
            if x == 1 {
                for y in 0 ..8 {
                    board.grid[x][y] = Cell { piece: Pieces::PAWN, color: Color::BLACK };
                }
            }
            if x == 6 {
                for y in 0 ..8 {
                    board.grid[x][y] = Cell { piece: Pieces::PAWN, color: Color::WHITE };
                }
            }
        }
        board
    }
    fn print(&self) {
        for x in 0 ..8 {
            for y in 0 ..8 {
                match self.grid[x][y].piece {
                    Pieces::PAWN => println!("Pawn"),
                    Pieces::ROOK => println!("ROOK"),
                    Pieces::KNIGHT => println!("Knight"),
                    Pieces::BISHOP => println!("Bishop"),
                    Pieces::QUEEN => println!("Queen"),
                    Pieces::KING => println!("King"),
                    Pieces::NONE => println!("None"),
                }
            }
        }
    }
}

fn main() {
    let board = Board::init_board();

    board.print();
}
