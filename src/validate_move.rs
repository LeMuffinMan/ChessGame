// use crate::find_threat_on_path;
use crate::Board;
use crate::Coord;
use crate::board::Cell;
use crate::board::{Color, Color::*, Piece};
///3 times identical position
///50 mvoes with no pawn move, no take
///A player only can ASK for Null : input to add
// fn special_null() {}
///if the next player to play has no move possible
// fn is_a_pat() {}
///Check if on any adjacent case the king could avoid threat
// fn can_king_survive() {}
///If an ally piece can block the threatening piece
// fn can_block_threat() {
//Si on peut bloquer, simuler de nouveau le nouveau board avec is_king_exposed pour checker
//plusieurs threats
// }
///If an ally piece can take the threatening piece
// fn can_capture_threat() {
//Si on peut bloquer, simuler de nouveau le nouveau board avec is_king_exposed pour checker
//plusieurs threats
// }
///Once we temporarly validated the move, we must know if the king of the active player is threaten
///Pour checker si on peut faire un move OU si le move resoud la situation d'echec
///Pour checker si le move qui a ete valide met le roi adverse en echec
// fn is_king_exposed(_king_cell: &Coord, _board: &Board) -> bool {
//Checker les cavaliers sur les 8 cases possibles
//checker en ligne x 4
//checker en diag x 4
//     true
// }
///recursively search in line or diagonal if any Cell has a piece on it
///Getting the direction to go with diff
///setting the step to prorgress
///creating a next Coord as arg "from" for the next recursive
///if next == to : we succeed to reach "to" so it's validated
///if not : check if next is empty
///if next is empty, we call again the function
fn find_obstacle(from: &Coord, to: &Coord, board: &Board) -> bool {
    let from_row = from.row as i8;
    let from_col = from.col as i8;
    let to_row = to.row as i8;
    let to_col = to.col as i8;

    let row_diff = to_row - from_row;
    let col_diff = to_col - from_col;

    let sign_row = if row_diff > 0 {
        1
    } else if row_diff < 0 {
        -1
    } else {
        0
    };

    let sign_col = if col_diff > 0 {
        1
    } else if col_diff < 0 {
        -1
    } else {
        0
    };

    let next_row = from_row + sign_row;
    let next_col = from_col + sign_col;

    let next = Coord {
        row: next_row as u8,
        col: next_col as u8,
    };

    if next == *to {
        return false;
    }

    if board.get(&next) != Cell::Free {
        return true;
    }

    find_obstacle(&next, to, board)
}

//Comment refacto proprement cette fonction ?
//plus de match / moins de if else
//faire des is_legal_move_pawn .. ?
///check if the piece situated at from coords, can move to the "to" coords, and if there is an
///obstacle on way
pub fn is_legal_move(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    let cell = board.get(from);
    match cell {
        Cell::Free => false,
        Cell::Occupied(piece, piece_color) => {
            match piece {
                Piece::Pawn => {
                    let dir: i8 = if *color == White { 1 } else { -1 };
                    let start_row = if *color == White { 1 } else { 6 };
                    let passant_row = if *color == White { 4 } else { 3 };

                    let row_diff = to.row as i8 - from.row as i8;
                    let col_diff = to.col as i8 - from.col as i8;

                    let target_square = &board.get(to);


                    //en passant :
                    //if last turn we saved the coord of the pawn expsing itself to en passant
                    // && the pawn to move, is on his possible raw to take en passant
                    // && the pawn try to move on the same col as the en_passant coord
                    // && the pawn moves in 1 diagonaly
                    // && the pawn tries to move behind the pawn exposed
                    
                    if let Some(coord) = &board.en_passant {
                        println!("en passant flag : {:?}", coord);
                    } else {
                        println!("en passant flag : None");
                    }
                    if let Some(coord) = &board.en_passant {
                        println!("Debug en passant");
                        println!(
                            "from.row = {}, passant_row = {} -> {}",
                            from.row,
                            passant_row,
                            from.row == passant_row
                        );
                        println!("col_diff.abs() = {} -> {}", col_diff, col_diff == 1);
                        println!(
                            "to.col = {}, coord.col = {} -> {}",
                            to.col,
                            coord.col,
                            to.col == coord.col
                        );
                        println!(
                            "to.row = {}, coord.row = {}, dir = {}, coord.row+dir = {} -> {}",
                            to.row,
                            coord.row,
                            dir,
                            coord.row as i8 + dir,
                            to.row as i8 == coord.row as i8 + dir
                        );
                    }
                    if let Some(coord) = &board.en_passant
                        && from.row == passant_row
                        && coord.col == to.col
                        && col_diff.abs() == 1
                        && to.row as i8 == coord.row as i8 + dir
                    {
                        return true;
                        //le print se fait en face du pion qui mange
                        //le pion mange est pas clean
                    }
                    //takes in diag if
                    //- the pawn tries to go one cell in his color direction
                    //- it tries to move in diagonal
                    //- there is an opponent piece in the dest cell
                    if row_diff as i8 == dir && (col_diff == 1 || col_diff as i8 == -1) {
                        return target_square.is_opponent_color(&piece_color);
                    }

                    //moves by one cell straight forward
                    //if it's empty
                    if row_diff as i8 == dir && col_diff == 0 {
                        return target_square.is_empty();
                    }

                    //first double move for pawns
                    if from.row == start_row && row_diff as i8 == dir * 2 && col_diff == 0 {
                        let mid_row = from.row as i8 + dir;
                        let mid_cell = board.grid[mid_row as usize][from.col as usize];
                        return mid_cell == Cell::Free
                            && *target_square == Cell::Free
                            && !find_obstacle(from, to, board);
                    }
                    false
                }
                Piece::Rook => true,
                Piece::Knight => {
                    //ignore obstacles
                    true
                }
                Piece::Bishop => true,
                Piece::Queen => true,
                Piece::King => {
                    //Roque
                    true
                }
            }
        }
    }
}
