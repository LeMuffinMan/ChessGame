use crate::Board;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::cell::Cell;
use crate::validate_move::is_legal_move::find_obstacle;

pub fn queen_case(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    if !board.get(to).is_color(color) {
        if from.row != to.row && from.col != to.col {
            return !find_obstacle(from, to, board);
        }
        if from.row == to.row || from.col == to.col {
            return !find_obstacle(from, to, board);
        }
    }
    false
}

pub fn bishop_case(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    if !board.get(to).is_color(color) {
        if from.row != to.row && from.col != to.col {
            return !find_obstacle(from, to, board);
        }
    }
    false
}

pub fn knight_case(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
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
        if to.row as i8 == from.row as i8 + *dx && to.col as i8 == from.col as i8 + *dy {
            if !board.get(to).is_color(color) {
                return true;
            }
        }
    }
    false
}

pub fn rook_case(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    if !board.get(to).is_color(color) {
        if from.row == to.row || from.col == to.col {
            return !find_obstacle(from, to, board);
        }
    }
    false
}

pub fn pawn_case(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
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
    if row_diff == dir && (col_diff == 1 || col_diff == -1) {
        return target_square.is_opponent_color(color);
    }

    //moves by one cell straight forward
    //if it's empty
    if row_diff == dir && col_diff == 0 {
        return target_square.is_empty();
    }

    //first double move for pawns
    if from.row == start_row && row_diff == dir * 2 && col_diff == 0 {
        let mid_row = from.row as i8 + dir;
        let mid_cell = board.grid[mid_row as usize][from.col as usize];
        return mid_cell == Cell::Free
            && *target_square == Cell::Free
            && !find_obstacle(from, to, board);
    }
    false
}

pub fn king_case(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    let dif_col = to.col as i8 - from.col as i8;
    if dif_col.abs() == 2 {
        let castle_bools = if *color == White {
            board.white_castle
        } else {
            board.black_castle
        };
        //si il bouge de deux a gauche : grand roque
        if dif_col < 0 && castle_bools.0 {
            //si le roi et aucune des deux cases qu'il traverse n'est en echec
            //Si toutes les cases entre K et R sont vides
            let mut to_dir = *to;
            to_dir.col += 1;
            return !find_obstacle(from, &to_dir, board);
        }
        //si deux a droite : petit roque
        else if dif_col > 0 && castle_bools.1 {
            //si le roi et aucune des deux cases qu'il traverse n'est en echec
            //Si toutes les cases entre K et R sont vides
            let mut to_dir = *to; // *to instead of to.clone() because Coord implement Copy
            to_dir.col -= 1;
            return !find_obstacle(from, &to_dir, board);
        } else {
            return false;
        }
    }
    let cells: [(i8, i8); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for (dx, dy) in cells.iter() {
        if to.row as i8 == from.row as i8 + *dx && to.col as i8 == from.col as i8 + *dy {
            if !board.get(to).is_color(color) {
                return true;
            }
        }
    }
    false
}
