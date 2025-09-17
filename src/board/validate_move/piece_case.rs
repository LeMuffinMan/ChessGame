use crate::Board;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::validate_move::is_legal_move::find_obstacle;

//we call recursives searches in diag and line
pub fn queen_case(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    if !board.get(to).is_color(color) {
        let row_diff = to.row as i32 - from.row as i32;
        let col_diff = to.col as i32 - from.col as i32;
        if row_diff.abs() == col_diff.abs() {
            return !find_obstacle(from, to, board);
        }
        if from.row == to.row || from.col == to.col {
            return !find_obstacle(from, to, board);
        }
    }
    false
}

//we call recursives searches in diag
pub fn bishop_case(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    if !board.get(to).is_color(color) {
        let row_diff = to.row as i32 - from.row as i32;
        let col_diff = to.col as i32 - from.col as i32;
        // println!("row_diff = {row_diff} | col_diff = {col_diff}");
        if row_diff.abs() == col_diff.abs() {
            return !find_obstacle(from, to, board);
        }
    }
    false
}

//hard coded 8 possible positions, unsing it as a masks and apply each of it as a move
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
        if to.row as i8 == from.row as i8 + *dx
            && to.col as i8 == from.col as i8 + *dy
            && !board.get(to).is_color(color)
        {
            return true;
        }
    }
    false
}

//recursively search in line
pub fn rook_case(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    if !board.get(to).is_color(color) && from.row == to.row || from.col == to.col {
        return !find_obstacle(from, to, board);
    }
    false
}

//the trickiest one with the king
//it can goes 1 cell in one direction dependending of the color player
//BUT it can move 2 cells if it's the first pawn move
//By doing so, it exposes himself at a en_passant if it land in a column next to an opponent pawn
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

//We need to check for a regular move, but also for the castles
pub fn king_case(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    let dif_col = to.col as i8 - from.col as i8;
    if dif_col.abs() == 2 {
        let castle_bools = if *color == White {
            board.white_castle
        } else {
            board.black_castle
        };
        //if the king tries a long castle, we need to check te threats on the cells it go trhough
        if dif_col < 0 && castle_bools.1 {
            let mut to_dir = *to;
            to_dir.col += 1;
            let cell_1 = Coord {
                row: from.row,
                col: from.col - 1,
            };
            let cell_2 = Coord {
                row: from.row,
                col: from.col - 2,
            };
            let cell_3 = Coord {
                row: from.row,
                col: from.col - 3,
            };

            //if there is no pieces in between the king and the left rook
            //And the king does not cross a threaten cell
            //and the king is not in check position
            if board.get(&cell_1).is_empty()
                && board.get(&cell_2).is_empty()
                && board.get(&cell_3).is_empty()
                && !board.threaten_cells.contains(&cell_1)
                && !board.threaten_cells.contains(&cell_2)
                && board.check.is_none()
            {
                return true;
            }
            //Same for short castle, it's 1 cell shorter
        } else if dif_col > 0 && castle_bools.0 {
            //si le roi et aucune des deux cases qu'il traverse n'est en echec
            //Si toutes les cases entre K et R sont vides
            let mut to_dir = *to; // *to instead of to.clone() because Coord implement Copy
            to_dir.col -= 1;
            let cell_1 = Coord {
                row: from.row,
                col: from.col + 1,
            };
            let cell_2 = Coord {
                row: from.row,
                col: from.col + 2,
            };
            //si le roi et aucune des deux cases qu'il traverse n'est en echec
            //Si toutes les cases entre K et R sont vides
            if board.get(&cell_1).is_empty()
                && board.get(&cell_2).is_empty()
                && !board.threaten_cells.contains(&cell_1)
                && !board.threaten_cells.contains(&cell_2)
                && board.check.is_none()
            {
                return true;
            }
        }
        return false;
    }

    //hard coded to use as a mask as for knight
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
        if to.row as i8 == from.row as i8 + *dx
            && to.col as i8 == from.col as i8 + *dy
            && !board.get(to).is_color(color)
        {
            return true;
        }
    }
    false
}
