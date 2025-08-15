use std::io;

mod board;
use board::Board;
use board::Color;

struct Coord {
    col: u8,
    row: u8,
}

impl Coord {
    fn is_player_color(coord: &Coord, color: Color, board: &Board) -> bool {
        if color != board.grid[coord.row as usize][coord.col as usize].color {
            return false;
        } 
        true
    }
    fn get_coord_from_string(cell: String, color: Color, board: &Board) -> Result<Coord, String> {
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

fn main() {
    let board = Board::init_board();

    board.print();

    let i = 1;
    loop {
        println!("Turn {i}");
        if i % 2 != 0 {
            println!("White to move");
        }
        else {
            println!("Black to move");
        }
        let from_coord = loop {
            let mut from_cell = String::new();
            println!("From cell :");
            io::stdin().read_line(&mut from_cell).expect("Error");
            let from_cell = from_cell.trim();
            match Coord::get_coord_from_string(from_cell.to_string(), Color::WHITE, &board) { 
                Ok(coord) => {
                    if !Coord::is_player_color(&coord, Color::WHITE, &board) {
                        println!("No {:?} piece in {}", Color::WHITE, from_cell);
                        continue;
                    } 
                    //Si la fct renvoie une struct coord ET
                    //que la coord est bien une case du joueur : 
                    //=> on break, et le break assigne la coord a from_coord 
                    break coord
                } 
                Err(e) =>  {
                    println!("{e}");
                    continue;
                    //si non : on relance l'input
                },
            };
        };
        //ici, from_coord est assigne d'une struct coord validee
        let mut to_cell = String::new();
        println!("To cell :");
        io::stdin().read_line(&mut to_cell).expect("Error");
        let to_cell = to_cell.trim();
        //Ici checker si la case existe ET si une piece noire s'y trouve
            //si non : on relance l'input
        //Si les deux cases existent, on execute le coup 
        // println!("From {from_cell} to {to_cell}");
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
