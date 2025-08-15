use std::io;

mod board;
use board::Board;
use board::Color;

#[derive(Debug)]
struct Coord {
    col: u8,
    row: u8,
}

impl Coord {
    //pas forcement des impl de Coord
    fn is_player_color(coord: &Coord, color: Color, board: &Board) -> bool {
        if color != board.grid[coord.row as usize][coord.col as usize].color {
            return false;
        } 
        true
    }
    fn get_coord_from_string(cell: String) -> Result<Coord, String> {
        if cell.len() != 2 {
            return Err(format!("Invalid input size : {}", cell.len()));
        }
        let mut chars = cell.chars();
        let col = chars.next().unwrap().to_ascii_lowercase() as u8 - b'a';
        let row = chars.next().unwrap() as u8 - b'0' - 1;
        if col > 7 || row > 7 {
            return Err(format!("Invalid input : outside of board : {} {}", row, col));
        }
        let coord = Coord {col, row};
        Ok(coord)
    }
}

fn get_inputs(msg: &str, color: Color, board: &Board) -> Coord {
    return loop {
        let mut input = String::new();
        println!("{} cell :", msg);
        io::stdin().read_line(&mut input).expect("Error");
        let input = input.trim();
        match Coord::get_coord_from_string(input.to_string()) {
            Ok(coord) => {
                if (msg == "from" && !Coord::is_player_color(&coord, color, &board) ) || 
                    (msg == "to" && Coord::is_player_color(&coord, color, &board) ) {
                    println!("No {:?} piece in {}", color, input);
                    continue;
                }
                break coord
            }
            Err(e) => {
                println!("{e}");
                continue;
            }
        }
    }
}

fn main() {
    let board = Board::init_board();

    board.print();

    let i = 1;
    loop {
        println!("Turn {i}");
        let color = if i % 2 != 0 {
            println!("White to move");
            Color::WHITE
        } else {
            println!("Black to move");
            Color::BLACK
        };
        let from_coord = get_inputs("from", color, &board);
        let to_coord = get_inputs("to", color, &board);      
        println!("From {:?} to {:?}", from_coord, to_coord);
        break;
        // i += 1;
    }
}

//loop
        //Si le dernier coup a fait un check 
            //input + output
                //Autorise ?
                //mise en echec ?
                //obstacle ?
                //Sortie du check de base ?
                //Oui 
                    //Continue
                //Non 
                    //Refuser le coup
        //input : case de depart
            //si pas de piece COLOR en case de depart on relance
        //output : case d'arrivee
        //check la validite du move :
            //deplacement autorise pour la piece ?
            //obstacle ?
            //situation de check avant ? apres ?
        //Check du pat / mat
            //si le move met le roi adverse en mat OU si pat : 
                //break
        //print
