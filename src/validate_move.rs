// use crate::find_threat_on_path;
use crate::Board;
use crate::Pieces;
use crate::Coord;
use crate::Color;
use crate::Color::*;
use crate::Color::NONE;
///3 times identical position
///50 mvoes with no pawn move, no take
///A player only can ASK for Null : input to add 
// fn special_null() {
//
// }

///Once we temporarly validated the move, we must know if the king of the active player is threaten
///Pour checker si on peut faire un move OU si le move resoud la situation d'echec
///Pour checker si le move qui a ete valide met le roi adverse en echec
// fn is_king_exposed(king_cell: &Coord, board: &Board) -> bool {
//     //Checker les cavaliers sur les 8 cases possibles
//     //checker en ligne x 4
//     //checker en diag x 4
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

    if next.row == to.row && next.col == to.col {
        return false;
    }

    if board.grid[next.row as usize][next.col as usize].piece != Pieces::NONE {
        return true;
    }

    find_obstacle(&next, to, board)
}


//Comment refacto proprement cette fonction ? 
//plus de match / moins de if else
//faire des is_legal_move_pawn .. ?
///check if the piece situated at from coords, can move to the "to" coords, and if there is an
///obstacle on way
pub fn is_legal_move(from: &Coord, to: &Coord, color: &Color, board: &mut Board) -> bool {

    let piece = board.grid[from.row as usize][from.col as usize].piece;
    //This vec contains all cells threatened by opponent
    let vec = match color {
        WHITE => &board.white_threatening_cells,
        BLACK => &board.black_threatening_cells,
        NONE => {
            println!("Error : invalid is_legal_move cell color"); 
            return false;
            //ameliorer la gestion d'erreur ?
            //to panic or not to panic ?
        }
    };

    match piece {
        Pieces::PAWN => {
            let dir: i8 = if *color == WHITE {1} else {-1};
            let start_row = if *color == WHITE {1} else {6};

            let row_diff = to.row - from.row;
            let col_diff = to.col - from.col;

            let target = &board.grid[to.row as usize][to.col as usize];

            if row_diff as i8 == dir && (col_diff == 1 || col_diff as i8 == -1) {
                return target.color != *color && target.color != NONE;
            }

            if row_diff as i8 == dir && col_diff == 0 {
                return target.color == NONE;
            }

            if from.row == start_row && row_diff as i8 == dir * 2 && col_diff == 0 {
                let mid_row = from.row as i8 + dir;
                return board.grid[mid_row as usize][from.col as usize].color == NONE
                    && target.color == NONE && !find_obstacle(from, to, board);
            }
            //Ajouter la prise en passant !!
            //si pion en face sur sa start raw et que coup precedent a fait deux cases
            //on peut manger en diag sur case vide
            false
        }
        Pieces::ROOK => {
            if from.row == to.row || from.col == to.col {
                return !find_obstacle(from, to, board)
            }
            false 

        }
        Pieces::KNIGHT => {
            let row_diff = (to.row as i8 - from.row as i8).abs();
            let col_diff = (to.col as i8 - from.col as i8).abs();

            if (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2) {
                return board.grid[to.row as usize][to.col as usize].color != board.grid[from.row as usize][from.col as usize].color;
            }

            false
        }
        Pieces::BISHOP => {
            let from_row = from.row as i8;
            let from_col = from.col as i8;
            let to_row = to.row as i8;
            let to_col = to.col as i8;

            let row_diff = to_row - from_row;
            let col_diff = to_col - from_col;

            if row_diff.abs() == col_diff.abs() {
                return !find_obstacle(from, to, board);
            }

            true
        }
        Pieces::QUEEN => {
            let from_row = from.row as i8;
            let from_col = from.col as i8;
            let to_row = to.row as i8;
            let to_col = to.col as i8;

            let row_diff = to_row - from_row;
            let col_diff = to_col - from_col;

            if row_diff.abs() == col_diff.abs() {
                return !find_obstacle(from, to, board);
            }
            if from.row == to.row || from.col == to.col {
                return !find_obstacle(from, to, board)
            }
            false
        }
        Pieces::KING => {
            let from_row = from.row as i8;
            let from_col = from.col as i8;
            let to_row = to.row as i8;
            let to_col = to.col as i8;

            let row_diff = to_row - from_row;
            let col_diff = to_col - from_col;

            if row_diff.abs() <= 1 && col_diff.abs() <= 1 {
                if board.grid[to.row as usize][to.col as usize].color != board.grid[from.row as usize][from.col as usize].color {
                    return !vec.contains(&to); // is the "to" cell in the threats map ?
                }
            }
            if col_diff == 3
                && !find_obstacle(from, to, board)
                // && !find_threat_on_path(from, to, board)
                && !vec.contains(&to) {
                if board.grid[from.row as usize][from.col as usize].color == WHITE {
                    return board.white_short_castle;
                } else {
                    return board.black_short_castle;
                }
            }
            if col_diff == 4
                && !find_obstacle(from, to, board)
                // && !find_threat_on_path(from, to, board) 
                // && !is_cell_threaten(to, board) declarer le vec des threats et utiliser
                // .contains
            {
                if board.grid[from.row as usize][from.col as usize].color == WHITE {
                    return board.white_long_castle;
                } else {
                    return board.black_long_castle;
                }
            }
            false
        }
        _ => {
            false
        }
    }
}
