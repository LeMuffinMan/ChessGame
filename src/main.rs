mod threats;
// use threats::find_threat_on_path;
use threats::update_threatens_cells;
mod board;
use board::Board;
use board::Color;
use board::Pieces;
mod get_inputs;
use get_inputs::Coord;
mod validate_move;
// use validate_move::is_legal_move;

//TO DO
//- Branch main / dev
//- accepter la pull request sur main
//- modifier dev en fct
//- merge sur main et bloquer les pushs
//
//- implementer refacto patoch
//- implementer stdin en pipe pour tests
//- refacto TOUT
//- commenter les doutes etc 
//      - Casts ? declarer un i32 le board ?
//      - Unit tests ?
//      - Iterator : perfs ? (update threats peut iterer differemment ?)
//      - Quelles fonctions doivent etre des impl ?
//      - rangement des structs ?
//
//++ implementer pat detect
//++ implementer check / mat detect
//++ implementer draw rules


fn main() {
    let mut board = Board::init_board();

    let mut i = 1;
    loop {
        update_threatens_cells(&mut board); //&mut to modify the threatens_cells lists
        board.print();
        println!("Turn {i}");
        let color = if i % 2 != 0 {
            println!("White to move");
            Color::WHITE
        } else {
            println!("Black to move");
            Color::BLACK
        };
        let from_coord = get_inputs::get_inputs("from", color, &board);
        let to_coord = get_inputs::get_inputs("to", color, &board);      
        println!("From {:?} to {:?}", from_coord, to_coord);
        if validate_move::is_legal_move(&from_coord, &to_coord, &color, &mut board) {
        //if a pawn is exposed to en_passant, it must be reset at end of its opponent turn
        // if let Some(coord) = board.en_passant 
        //     && board.grid[coord.row as usize][coord.col as usize].color != color {
        //     board.en_passant = None;
        // } 
            println!("Move validated");
            board.grid[to_coord.row as usize][to_coord.col as usize].piece = board.grid[from_coord.row as usize][from_coord.col as usize].piece;
            board.grid[to_coord.row as usize][to_coord.col as usize].color = board.grid[from_coord.row as usize][from_coord.col as usize].color;
            board.grid[from_coord.row as usize][from_coord.col as usize].piece = Pieces::NONE;
            board.grid[from_coord.row as usize][from_coord.col as usize].color = Color::NONE;
            //en passant ne se met pas correctement a jour
        } else {
            println!("Illegal move");
            continue ;
        }
        //A chaque tour, calculer chaque coup legal et comparer le move a la liste ?

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
