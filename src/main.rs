mod threat;
use crate::threat::update_threatens_cells;
use crate::update_threatens_cells::update_threatens_cells;
use threat::get_threaten_cells;
mod board;
use board::Board;
// use board::Cell;
use board::Color;
mod get_inputs;
use get_inputs::Coord;
mod validate_move;




fn main() {
    let mut board = Board::init_board();

    let mut i = 1;
    loop {
        let color = if i % 2 != 0 {
            println!("White to move");
            Color::White
        } else {
            println!("Black to move");
            Color::Black
        };
        update_threatens_cells(&mut board, &color);
        if let Some(coord) = board.get_king(&color) {
            if board.threaten_cells.contains(&coord) {
                println!("Check !");
            }
        }
        board.update_legals_moves(&color);
        // for coord in &board.threaten_cells {
        //     println!("Cell threaten : ({}, {})", coord.row, coord.col);
        // }
        if board.legals_moves.is_empty() {
            let king_cell = board.get_king(&color);
            if let Some(coord) = king_cell {
                if board.threaten_cells.contains(&coord) {
                    println!("Checkmate ! {:?} loose", color);
                }
            } else {
                println!("Pat");
            }
            break; 
        }
        board.print();
        println!("Turn {i}");

        let (from_coord, to_coord) = get_inputs::get_move_from_stdin(color, &board);
        println!("From {from_coord:?} to {to_coord:?}");
        // if board.is_legal_move(&from_coord, &to_coord, &color) {
        if board.is_legal_move(&from_coord, &to_coord, &color) {
            if !validate_move::is_king_exposed(&from_coord, &to_coord, &color, &board) {
                println!("Move validated");
                board.update_board(&from_coord, &to_coord, &color);
            } else {
                println!("King is exposed : illegal move");
                continue;
            }
        } else {
            println!("Illegal move : {from_coord:?} -> {to_coord:?}");
            continue;
        }

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

//TO DO
//- merge sur main et bloquer les pushs
//- refacto TOUT
//- commenter les doutes etc
//      - Casts ? declarer un i32 le board ?
//      - Unit tests ?
//      - Tests avec cargo et mon stdin qui accepte les pipes ?
//          - separe les tests qui doivent etre valides / les autres
//      - Iterator : perfs ? (update threats peut iterer differemment ?)
//      - Quelles fonctions doivent etre des impl ?
//          - is legal comme wrapper ou comme impl ?
//      - rangement des structs ?
//
//- validate move : 
//      - tester is_king_exposed
//          - tester update_threatens_cells 
//
//++ implementer roque
//++ implementer pat detect
//++ implementer check / mat detect
//++ implementer draw rules
//++ implementer promotions

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
