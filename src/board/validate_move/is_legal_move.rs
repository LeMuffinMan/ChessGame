use crate::Board;
use crate::Color;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::validate_move::piece_case::{
    bishop_case, king_case, knight_case, pawn_case, queen_case, rook_case,
};

///check if the piece on from coords, can move to the "to" coords, and if there is an
///obstacle on way
pub fn is_legal_move(
    from: &Coord,
    to: &Coord,
    color: &Color,
    threaten_cells: &Vec<Coord>,
    board: &Board,
) -> bool {
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
                King => king_case(from, to, color, threaten_cells, board),
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
//Makes a copy of the board, and update it with the legal move to verify is the active player Some(&King)
//is in check position or does not solve a previous check position
// pub fn is_Some(&King)_exposed(from: &Coord, to: &Coord, color: &Color, board: &Board) -> bool {
//     let mut new_board = board.clone();
//     new_board.update_board(from, to, color);
//     new_board.update_threatens_cells(color);
//     if let Some(coord) = new_board.get_Some(&King)(color) {
//         new_board.threaten_cells.contains(&coord)
//     } else {
//         println!("Error : {:?} Some(&King) not found", color);
//         false
//     }

pub fn is_king_exposed(board: &Board, active_player: &Color) -> bool {
    let king_pos = match active_player {
        Color::White => board.white_king,
        Color::Black => board.black_king,
    };

    // Rooks, Bishops, Queens directions
    let directions = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for (dr, dc) in directions {
        for dist in 1..8 {
            let r = king_pos.row as i32 + dr * dist;
            let c = king_pos.col as i32 + dc * dist;

            if r < 0 || r >= 8 || c < 0 || c >= 8 {
                break;
            }

            let cell = &board.grid[r as usize][c as usize];
            if let (Some(p_type), Some(p_color)) = (cell.get_piece(), cell.get_color()) {
                if *p_color == *active_player {
                    break; // friendly piece blocking
                } else {
                    // opponent piece found is it threats this line ?
                    let is_diag = dr != 0 && dc != 0;
                    match p_type {
                        Piece::Queen => return true,
                        Piece::Rook if !is_diag => return true,
                        Piece::Bishop if is_diag => return true,
                        Piece::King if dist == 1 => return true,
                        Piece::Pawn if dist == 1 && is_diag => {
                            // Pawns threats is diag in front of them
                            let attack_dir = if *active_player == Color::White {
                                1
                            } else {
                                -1
                            };
                            if dr == attack_dir {
                                return true;
                            }
                        }
                        _ => break, // no threats : knight for exemple
                    }
                    break; // opponent piece blocks
                }
            }
        }
    }

    // Knights
    let knight_moves = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    for (dr, dc) in knight_moves {
        let r = king_pos.row as i32 + dr;
        let c = king_pos.col as i32 + dc;

        if r >= 0 && r < 8 && c >= 0 && c < 8 {
            let cell = &board.grid[r as usize][c as usize];
            if cell.get_piece() == Some(&Piece::Knight) && cell.get_color() != Some(active_player) {
                return true;
            }
        }
    }

    false
}
