use std::io;

mod board;
use board::Board;

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
        let mut from_cell = String::new();
        println!("From cell :");
        io::stdin().read_line(&mut from_cell).expect("Error");
        let from_cell = from_cell.trim();
        //Ici checker si la case existe
            //si non : on relance l'input
        let mut to_cell = String::new();
        println!("To cell :");
        io::stdin().read_line(&mut to_cell).expect("Error");
        let to_cell = to_cell.trim();
        //Ici checker si la case existe
            //si non : on relance l'input
        //Si les deux cases existent, on execute le coup 
        println!("From {from_cell} to {to_cell}");
        break;
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
}
