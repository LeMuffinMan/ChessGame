use std::io;

mod board;
use board::Board;
use board::Color;

#[derive(Debug)]
struct Coord {
    col: u8,
    row: u8,
}

///Compare the color given as argument with the color of the cell in argument
fn is_player_color(coord: &Coord, color: Color, board: &Board) -> bool {
    if color != board.grid[coord.row as usize][coord.col as usize].color {
        return false;
    } 
    true
}

///translate pgn code into regular coordinates, with minimal error management
fn get_coord_from_string(cell: String) -> Result<Coord, String> {
    if cell.len() != 2 {
        return Err(format!("Invalid input size : {}", cell.len()));
    }
    let mut chars = cell.chars();
    let col = match chars.next() {
        Some(c) => c.to_ascii_lowercase() as u8 - b'a',
        None => return Err("Invalid input: error parsing column".to_string()),
    };
    let row = match chars.next() {
        Some(c) => c as u8 - b'0' - 1,
        None => return Err("Invalid input: error parsing row".to_string()),
    };
    if col > 7 || row > 7 {
        return Err(format!("Invalid input : outside of board : {} {}", row, col));
    }
    let coord = Coord {col, row};
    Ok(coord)
}

///return a struct coord after reading the input from stdin
fn get_inputs(msg: &str, color: Color, board: &Board) -> Coord {
    return loop {
        let mut input = String::new();
        println!("{} cell :", msg);
        io::stdin().read_line(&mut input).expect("Error");
        let input = input.trim();
        match get_coord_from_string(input.to_string()) {
            Ok(coord) => {
                if msg == "from" && !is_player_color(&coord, color, &board) {
                    println!("No {:?} piece in {}", color, input);
                    continue;
                }
                if  msg == "to" && is_player_color(&coord, color, &board) {
                    println!("There is already a {:?} piece in {}", color, input);
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

    let mut i = 1;
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
        //Coup possible pour la piece ? (legal + obstacle)
            //special : pion en passant / pion promotion / roque
        //Le nouveau board mettrait il le roi du joueur actif en echec ?
        //Si le roi etait en echec, est-ce que ce coup le resoud ?
        //Le coup provoque t il un check pour le joueur adverse ?
            //si oui : 
                //si la piece jouee ne peut etre capturee 
                //Si la piece jouee ne peut etre bloquee 
                //si le roi ne peut pas bouger hors de menace
                    //=> mat
        //is a pat ? (3 fois la meme situation / nb de coups sans prise / aucun movement possible)
            //=> Pat
        i += 1;
        println!("-----------------");
    }
}


        //Validation :
            //- si Roque : verifier si le roque est valide
                //- les deux pieces ont deja bouge ?
                //- la trajectoire entre elle est elle en echec ?
                //- le roi est il en echec ? (la tour aussi ?)
            //- Si le roi du joueur actif est en echec : verifier si le move le resoud ou pas
            //- move possible pour la piece ?
            //- obstacles ?
            //- mise en echec de son roi en faisant ce move ?
            //- Ajotuer le move a une liste pour checker le pat
        //Move valide
            //- Mettre a jour le board
            //- checker si le roi adverse est mis en echec
                //- Si oui : checker si c'est un mat
                    //- si une piece aliee peut manger la menace
                    //- si une piece aliee peut bloauer la ou les menaces
                    //- si le roi peut se deplacer pour se proteger
            //- checker si le roi adverse est en situation de pat
            //- checker si le nb de move sans prise / de move repete declenche le pat
            //- si c'est la 3eme occurrence de la meme situation du board : pat

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
