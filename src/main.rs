mod board;
use board::Board;

fn main() {
    let board = Board::init_board();

    board.print();

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
