use std::io;

mod board;
use board::Board;
use board::Color;
use board::Pieces;

#[derive(Debug)]
struct Coord {
    col: u8,
    row: u8,
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
                if msg == "from" && color != board.grid[coord.row as usize][coord.col as usize].color {
                    println!("No {:?} piece in {}", color, input);
                    continue;
                }
                if  msg == "to" && color == board.grid[coord.row as usize][coord.col as usize].color {
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

///3 times identical position
///50 mvoes with no pawn move, no take
///A player only can ASK for Null : input to add 
fn special_null() {

}

///if the next player to play has no move possible
fn is_a_pat() {

}

///Check if on any adjacent case the king could avoid threat
fn can_king_move() {

}

///If an ally piece can block the threatening piece 
fn can_block_threat() {

    //Si on peut bloquer, simuler de nouveau le nouveau board avec is_king_exposed pour checker
    //plusieurs threats
}

///If an ally piece can take the threatening piece 
fn can_capture_threat() {

    //Si on peut bloquer, simuler de nouveau le nouveau board avec is_king_exposed pour checker
    //plusieurs threats
}

///Once we temporarly validated the move, we must know if the king of the active player is threaten
///Pour checker si on peut faire un move OU si le move resoud la situation d'echec
///Pour checker si le move qui a ete valide met le roi adverse en echec
fn is_king_exposed(king_cell: &Coord, board: &Board) -> bool {
    //Checker les cavaliers sur les 8 cases possibles
    //checker en ligne x 4
    //checker en diag x 4
    true
}

///check if the piece situated at from coords, can move to the "to" coords, and if there is an
///obstacle on way
fn is_legal_move(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    if board.grid[from.col as usize][from.row as usize].piece == Pieces::PAWN {
        //prise en passant
        //promotion
        return true;
    } else if board.grid[from.col as usize][from.row as usize].piece == Pieces::ROOK {
    
        return true;
    } else if board.grid[from.col as usize][from.row as usize].piece == Pieces::KNIGHT {
        //ignore obstacles
        return true;
    } else if board.grid[from.col as usize][from.row as usize].piece == Pieces::BISHOP {

        return true;
    } else if board.grid[from.col as usize][from.row as usize].piece == Pieces::QUEEN {
    
        return true;
    } else if board.grid[from.col as usize][from.row as usize].piece == Pieces::KING {
        //Roque
        return true;
    } 
    false
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
            //Dans la struct board : mettre a jour le bool check en fin de move et checker ici sa
            //valeur ?
            //si c'est resolu, on peut remttre le bool a false 
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
