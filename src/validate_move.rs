use crate::Board;
use crate::Coord;
use crate::board::Cell;
use crate::board::{Color, Color::*, Piece};
use crate::update_threatens_cells;

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


pub fn is_king_exposed(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    let mut new_board = board.clone();
    new_board.update_board(from, to, color);
    update_threatens_cells(&mut new_board, color);
    if let Some(coord) = new_board.get_king(color) {
        new_board.threaten_cells.contains(&coord)
    } else {
        println!("Error : {:?} king not found", color);
        false
    }
}


///check if the piece on from coords, can move to the "to" coords, and if there is an
///obstacle on way
//il faut qu'elle renvoie un board mis a jour, ou rien
//on check is_king_exposed sur ce board et on return true ou false
pub fn is_legal_move(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    let cell = board.get(from);
    match cell {
        Cell::Free => false,
        //ici soit je vire color soit je change de nom de variable
        Cell::Occupied(piece, piece_color) => {
            if piece_color != *color {
                return false;
            }
            match piece {
                Piece::Pawn => {
                    pawn_case(from, to, color, board) 
                }
                Piece::Rook => {
                    if !board.get(to).is_color(color) {
                        if from.row == to.row || from.col == to.col {
                            return !find_obstacle(from, to, board);
                        }
                    }
                    false
                }
                Piece::Knight => {
                    let cells: [(i8, i8); 8] = [
                        (2, 1),
                        (2, -1),
                        (-2, 1),
                        (-2, -1),
                        (1, 2),
                        (1, -2),
                        (-1, 2),
                        (-1, -2),
                    ];

                    for (dx, dy) in cells.iter() {
                       if to.row as i8 == from.row as i8 + *dx as i8
                        && to.col as i8 == from.col as i8 + *dy as i8 {
                            if !board.get(to).is_color(color) {
                                return true;
                            }
                        }
                    }
                    false
                }
                Piece::Bishop => 
                {
                    if !board.get(to).is_color(color) {
                        //meilleure verfi de la diagonale ?
                        if from.row != to.row && from.col != to.col {
                            return !find_obstacle(from, to, board);
                        }
                    }
                    false
                }
                Piece::Queen => 
                {
                    if !board.get(to).is_color(color) {
                        //meilleure verfi de la diagonale ?
                        if from.row != to.row && from.col != to.col {
                            return !find_obstacle(from, to, board);
                        }
                        if from.row == to.row || from.col == to.col {
                            return !find_obstacle(from, to, board);
                        }
                    }
                    false
                }
                Piece::King => {
                    king_case(from, to, color, board)
                }
            }
        }
    }
}

fn pawn_case(from :&Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    let dir: i8 = if *color == White { 1 } else { -1 };
    let start_row = if *color == White { 1 } else { 6 };
    let passant_row = if *color == White { 4 } else { 3 };

    let row_diff = to.row as i8 - from.row as i8;
    let col_diff = to.col as i8 - from.col as i8;

    let target_square = &board.get(to);

    //en passant :
    //if an opponent pawn moved by 2 last turn
    // && the pawn to move, is on his possible raw to take en passant
    // && the pawn try to move on the same col as the en_passant coord
    // && the pawn moves in 1 diagonaly
    // && the pawn tries to move behind the pawn exposed
    if let Some(coord) = &board.en_passant //virer la var en passant / chercher si a cote de coord
        && from.row == passant_row
        && coord.col == to.col
        && col_diff.abs() == 1
        && to.row as i8 == coord.row as i8 + dir
    {
        return true;
    }

    //takes in diag if
    //- the pawn tries to go one cell in his color direction
    //- it tries to move in diagonal
    //- there is an opponent piece in the dest cell
    if row_diff as i8 == dir && (col_diff == 1 || col_diff as i8 == -1) {
        return target_square.is_opponent_color(color);
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

fn king_case(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    let dif_col = to.col as i8 - from.col as i8;
    if dif_col.abs() == 2 {
        let castle_bools = if *color == White { board.white_castle } else { board.black_castle };
        //si il bouge de deux a gauche : grand roque
        if dif_col < 0 && castle_bools.0 == true {
            //si le roi et aucune des deux cases qu'il traverse n'est en echec 
            //Si toutes les cases entre K et R sont vides 
            let mut to_dir = to.clone();
            to_dir.col += 1;
            return !find_obstacle(from, &to_dir, board);
        }
        //si deux a droite : petit roque
        else if dif_col > 0 && castle_bools.1 == true {
            //si le roi et aucune des deux cases qu'il traverse n'est en echec 
            //Si toutes les cases entre K et R sont vides 
            let mut to_dir = to.clone();
            to_dir.col -= 1;
            return !find_obstacle(from, &to_dir, board);
        } 
        else { false; }
    }
    let cells: [(i8, i8); 8] = [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for (dx, dy) in cells.iter() {
        if to.row as i8 == from.row as i8 + *dx as i8
        && to.col as i8 == from.col as i8 + *dy as i8 {
            if !board.get(to).is_color(color) {
                return true;
            }
        }
    }
    false
    //Si il bouge de deux : castle
        //gauche : big 
        //droite : little
        //Ssi bool est ok
        //checker si le roi est en situation d'echec
        //Checker les menaces sur toute la ligne de mvt // board.threaten_cells.contains(cells)
        //si les deux cases traersees par le roi son vides 
        //update board fera le move, et mettra a off les deux bool
    //bouge d'une case
        //si pas occupee par piece alliee
        //si pas menacee 
        //a update board : on vire les deux bool de castle
    // true
}


