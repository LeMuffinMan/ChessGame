use crate::Board;
use crate::Color;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::board::validate_move::piece_case::{
    bishop_case, king_case, knight_case, pawn_case, queen_case, rook_case,
};

///check if the piece on from coords, can move to the "to" coords, and if there is an
///obstacle on way
pub fn is_legal_move(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
    let cell = board.get(from);
    match cell {
        Cell::Free => false,
        //ici soit je vire color soit je change de nom de variable
        Cell::Occupied(piece, piece_color) => {
            if piece_color != *color {
                return false;
            }
            if board.get(to).is_color(color) {
                return false;
            }
            match piece {
                Pawn => pawn_case(from, to, color, board),
                Rook => rook_case(from, to, color, board),
                Knight => knight_case(from, to, color, board),
                Bishop => bishop_case(from, to, color, board),
                Queen => queen_case(from, to, color, board),
                King => king_case(from, to, color, board),
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
//Makes a copy of the board, and update it with the legal move to verify is the active player king
//is in check position or does not solve a previous check position
// pub fn is_king_exposed(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
//     let mut new_board = board.clone();
//     new_board.update_board(from, to, color);
//     new_board.update_threatens_cells(color);
//     if let Some(coord) = new_board.get_king(color) {
//         new_board.threaten_cells.contains(&coord)
//     } else {
//         println!("Error : {:?} king not found", color);
//         false
//     }

pub fn is_king_exposed(self) -> bool {
    king_pos = board.getKing(board.active_player);
    // on devrait garder de cote les coords deux rois pour ne pas avoir a les rechercher a chaque fois ?

    i = 0;
    //on cherche en ligne vers la droite
    while king_pos.row + i < 8 {
        piece = (
            board.getPiece(king_pos.row + i, king_pos.col),
            board.getColor(king_pos.row + i, king_pos.col),
        );
        // si on tombe sur une piece aliee, elle protege des tours / queens
        if piece.1 == board.active_player {
            break;
        }
        if piece.1 != board.active_player && (piece.0 == Rook || piece.0 == Queen) {
            return true;
        }
        if i == 1 && piece.0 == King && piece.1 != board.active_player {
            return true;
        }
        i += 1;
    }
    i = 0;
    //on cherche vers la gauche
    while king_pos.row + i > 0 {
        piece = (
            board.getPiece(king_pos.row + i, king_pos.col),
            board.getColor(king_pos.row + i, king_pos.col),
        );
        // si on tombe sur une piece aliee, elle protege des tours / queens
        if piece.1 == board.active_player {
            break;
        }
        if piece.1 != board.active_player && (piece.0 == Rook || piece.0 == Queen) {
            return true;
        }
        if i == -1 && piece.0 == King && piece.1 != board.active_player {
            return true;
        }
        i -= 1;
    }

    //on cherche vers le bas
    i = 0;
    while king_pos.col + i < 8 {
        piece = (
            board.getPiece(king_pos.row, king_pos.col + i),
            board.getColor(king_pos.row, king_pos.col + i),
        );
        // si on tombe sur une piece aliee, elle protege des tours / queens
        if piece.1 == board.active_player {
            break;
        }
        if piece.1 != board.active_player && (piece.0 == Rook || piece.0 == Queen) {
            return true;
        }
        if i == -1 && piece.0 == King && piece.1 != board.active_player {
            return true;
        }
        i -= 1;
    }
    // on cherche vers le haut
    i = 0;
    while king_pos.col + i > 0 {
        piece = (
            board.getPiece(king_pos.row + i, king_pos.col),
            board.getColor(king_pos.row + i, king_pos.col),
        );
        // si on tombe sur une piece aliee, elle protege des tours / queens
        if piece.1 == board.active_player {
            break;
        }
        if piece.1 != board.active_player && (piece.0 == Rook || piece.0 == Queen) {
            return true;
        }
        if i == 1 && piece.0 == King && piece.1 != board.active_player {
            return true;
        }
        i += 1;
    }

    //diag en haut a gauche x++ et y--
    i = 0;
    j = 0;
    while king_pos.row + i < 8 && king_pos.col + j > 0 {
        piece = (
            board.getPiece(king_pos.row + i, king_pos.col + j),
            board.getColor(king_pos.col + i, king_pos.col + j),
        );
        // si on tombe sur une piece aliee, elle protege des tours / queens
        if piece.1 == board.active_player {
            break;
            //si on ne break pas, on est sur une piece adverse
        }
        //un pion noir en haut a gauche
        if board.active_color == White && i == 1 && j == -1 && piece.0 == Pawn {
            return true;
        }
        if piece.1 != board.active_player && (piece.0 == Bishop || piece.0 == Queen) {
            return true;
        }
        if i == 1 && piece.0 == King {
            return true;
        }
        i += 1;
        j -= 1;
    }
    //diag en haut a droite x++ et y++
    i = 0;
    j = 0;
    while king_pos.row + i < 8 && king_pos.col + j < 8 {
        piece = (
            board.getPiece(king_pos.row + i, king_pos.col + j),
            board.getColor(king_pos.col + i, king_pos.col + j),
        );
        // si on tombe sur une piece aliee, elle protege des tours / queens
        if piece.1 == board.active_player {
            break;
            //si on ne break pas, on est sur une piece adverse
        }
        //un pion noir en haut a gauche
        if board.active_color == White && i == 1 && j == 1 && piece.0 == Pawn {
            return true;
        }
        if piece.1 != board.active_player && (piece.0 == Bishop || piece.0 == Queen) {
            return true;
        }
        if i == 1 && piece.0 == King {
            return true;
        }
        i += 1;
        j += 1;
    }
    //diag en bas a gauche x-- et y--
    i = 0;
    j = 0;
    while king_pos.row + i > 0 && king_pos.col + j > 0 {
        piece = (
            board.getPiece(king_pos.row + i, king_pos.col + j),
            board.getColor(king_pos.col + i, king_pos.col + j),
        );
        // si on tombe sur une piece aliee, elle protege des tours / queens
        if piece.1 == board.active_player {
            break;
            //si on ne break pas, on est sur une piece adverse
        }
        //un pion noir en haut a gauche
        if board.active_color == Black && i == -1 && j == -1 && piece.0 == Pawn {
            return true;
        }
        if piece.1 != board.active_player && (piece.0 == Bishop || piece.0 == Queen) {
            return true;
        }
        if i == 1 && piece.0 == King {
            return true;
        }
        i -= 1;
        j -= 1;
    }

    //diag en bas a droite x-- et y++
    i = 0;
    j = 0;
    while king_pos.row + i > 0 && king_pos.col + j < 8 {
        piece = (
            board.getPiece(king_pos.row + i, king_pos.col + j),
            board.getColor(king_pos.col + i, king_pos.col + j),
        );
        // si on tombe sur une piece aliee, elle protege des tours / queens
        if piece.1 == board.active_player {
            break;
            //si on ne break pas, on est sur une piece adverse
        }
        //un pion noir en haut a gauche
        if board.active_color == Black && i == -1 && j == 1 && piece.0 == Pawn {
            return true;
        }
        if piece.1 != board.active_player && (piece.0 == Bishop || piece.0 == Queen) {
            return true;
        }
        if i == 1 && piece.0 == King {
            return true;
        }
        i -= 1;
        j += 1;
    }
    // chercher les 8 positions du cavalier
    if board.getColor(king_pos.row + 2, king_pos.col + 1) != board.active_color
        && board.getPiece(king_pos.row + 2, king_pos.col + 1) == Knight
    {
        return true;
    }

    if board.getColor(king_pos.row + 2, king_pos.col - 1) != board.active_color
        && board.getPiece(king_pos.row + 2, king_pos.col - 1) == Knight
    {
        return true;
    }

    if board.getColor(king_pos.row - 2, king_pos.col + 1) != board.active_color
        && board.getPiece(king_pos.row - 2, king_pos.col + 1) == Knight
    {
        return true;
    }

    if board.getColor(king_pos.row - 2, king_pos.col - 1) != board.active_color
        && board.getPiece(king_pos.row - 2, king_pos.col - 1) == Knight
    {
        return true;
    }

    if board.getColor(king_pos.row + 1, king_pos.col + 2) != board.active_color
        && board.getPiece(king_pos.row + 1, king_pos.col + 2) == Knight
    {
        return true;
    }

    if board.getColor(king_pos.row - 1, king_pos.col + 2) != board.active_color
        && board.getPiece(king_pos.row - 1, king_pos.col + 2) == Knight
    {
        return true;
    }

    if board.getColor(king_pos.row + 1, king_pos.col - 2) != board.active_color
        && board.getPiece(king_pos.row + 1, king_pos.col - 2) == Knight
    {
        return true;
    }

    if board.getColor(king_pos.row - 1, king_pos.col - 2) != board.active_color
        && board.getPiece(king_pos.row - 1, king_pos.col - 2) == Knight
    {
        return true;
    }
    return false;
}
