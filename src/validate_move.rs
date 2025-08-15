use Crate::Board;
use Crate::Pieces;
use Crate::Coord;
use Crate::Color;

///3 times identical position
///50 mvoes with no pawn move, no take
///A player only can ASK for Null : input to add 
fn special_null() {

}

///if the next player to play has no move possible
fn is_a_pat() {

}

///Check if on any adjacent case the king could avoid threat
fn can_king_survive() {

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

    match board.grid[from.col as usize][from.row as usize].piece {
        Pieces::PAWN => {

        //prise en passant
        //promotion
            true
        }
        Pieces::ROOK => {

            true
        }
        Pieces::KNIGHT => {

        //ignore obstacles
            true
        }
        Pieces::BISHOP => {

            true
        }
        Pieces::QUEEN => {

            true
        }
        Pieces::KING => {

        //Roque
            true
        }
        _ => {
            false
        }
    }
}
