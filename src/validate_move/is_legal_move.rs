use crate::Board;
use crate::Color;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::cell::Piece;
use crate::validate_move::piece_case::{
    bishop_case, king_case, knight_case, pawn_case, queen_case, rook_case,
};

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
                Piece::Pawn => pawn_case(from, to, color, board),
                Piece::Rook => rook_case(from, to, color, board),
                Piece::Knight => knight_case(from, to, color, board),
                Piece::Bishop => bishop_case(from, to, color, board),
                Piece::Queen => queen_case(from, to, color, board),
                Piece::King => king_case(from, to, color, board),
            }
        }
    }
}

///recursively search in line or diagonal if any Cell has a piece on it
///Getting the direction to go with diff
///setting the step to prorgress
///creating a next Coord as arg "from" for the next recursive
///if next == to : we succeed to reach "to" so it's validated
///if not : check if next is empty
///if next is empty, we call again the function
pub fn find_obstacle(from: &Coord, to: &Coord, board: &Board) -> bool {
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
    new_board.update_threatens_cells(color);
    if let Some(coord) = new_board.get_king(color) {
        new_board.threaten_cells.contains(&coord)
    } else {
        println!("Error : {:?} king not found", color);
        false
    }
}
