use crate::Coord;
use crate::board::Board;
use crate::board::cell::Color;
use crate::board::moves::pieces_moves::king_moves::king_moves;
use crate::board::cell::Piece::*;
use crate::board::moves::move_structs::MoveList;
use crate::board::pin_detection::{PinInfos, pin_detection};
use crate::board::moves::pieces_moves::pawn_moves::pawn_moves;
use crate::board::moves::pieces_moves::sliding_pieces_moves::{rook_moves, bishop_moves, queen_moves};
use crate::board::moves::pieces_moves::knight_moves::knight_moves;

pub fn generate_moves(
    board: &mut Board,
    active_player: &Color,
    list: &mut MoveList,
    capture_only: bool,
) {
    let info = pin_detection(board, *active_player);
    for x in 0..8 {
        for y in 0..8 {
            if board.grid[x][y].is_color(active_player) {
                let origin = Coord {
                    row: x as u8,
                    col: y as u8,
                };
                if let Some(piece) = board.get(&origin).get_piece() {
                    match piece {
                        // double check, only king can move
                        _ if info.checker_count == 2 && *piece != King => continue,
                        Pawn => {
                            pawn_moves(&origin, active_player, board, list, capture_only, &info)
                        }
                        Rook => {
                            rook_moves(&origin, active_player, board, list, capture_only, &info)
                        }
                        Knight => {
                            knight_moves(&origin, active_player, board, list, capture_only, &info)
                        }
                        Bishop => {
                            bishop_moves(&origin, active_player, board, list, capture_only, &info)
                        }
                        Queen => {
                            queen_moves(&origin, active_player, board, list, capture_only, &info)
                        }
                        King => king_moves(&origin, active_player, board, list, capture_only),
                    }
                }
            }
        }
    }
}


//will the piece pinned expose king ?
pub fn aligned_with_pin(origin: &Coord, dest: &Coord, dr: i8, dc: i8) -> bool {
    let row_diff = dest.row as i8 - origin.row as i8;
    let col_diff = dest.col as i8 - origin.col as i8;
    row_diff * dc == col_diff * dr
}

// if safe, push without apply / undo / check_move
// only call check move if king is exposed
pub fn push_if_legal(
    board: &mut Board,
    origin: &Coord,
    dest: Coord,
    color: &Color,
    list: &mut MoveList,
    info: &PinInfos,
) {
    if board.get(&dest).is_color(color) {
        return;
    }
    if info.checker_count >= 1 {
        //If king is in check, we check_move
        if let Some(m) = board.check_move(origin, &dest, color) {
            list.push(m);
        }
        return;
    }
    if let Some((dr, dc)) = info.pins[origin.row as usize][origin.col as usize] {
        if !aligned_with_pin(origin, &dest, dr, dc) {
            return;
        }
    }
    list.push(board.build_move(*origin, dest, *color));
}
