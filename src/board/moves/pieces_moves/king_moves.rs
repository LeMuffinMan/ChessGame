use crate::Coord;
use crate::board::Board;
use crate::board::cell::Cell;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::moves::move_structs::MoveList;

pub fn king_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
) {
    king_offsets(board, *origin, capture_only, active_player, list);
    if capture_only {
        return;
    }
    king_castles(board, *origin, active_player, list);
}

fn king_offsets(
    board: &mut Board,
    origin: Coord,
    capture_only: bool,
    active_player: &Color,
    list: &mut MoveList,
) {
    #[rustfmt::skip]
    let offsets = [
        (-1, 1),  (0, 1), (1, 1),
        (-1, 0),          (1, 0),
        (-1, -1), (0, -1), (1, -1),
    ];
    for (dr, dc) in offsets {
        if let Some(dest) = Board::checked_coord(origin.row as i8 + dr, origin.col as i8 + dc) {
            if capture_only {
                let cell = board[dest];
                if !cell.is_color(active_player) && cell.get_piece().is_none() {
                    continue;
                }
            }
            if let Some(m) = board.check_move(&origin, &dest, active_player) {
                list.push(m);
            }
        }
    }
}

fn king_castles(board: &mut Board, origin: Coord, active_player: &Color, list: &mut MoveList) {
    if board.check.is_none() {
        let rights = if *active_player == White {
            board.white_castle
        } else {
            board.black_castle
        };

        let row = origin.row as usize;
        let col = origin.col as usize;

        if rights.short
            && board[(row, col + 1)] == Cell::Free && board[(row, col + 2)] == Cell::Free
                && let (Some(t), Some(d)) = (
                    Board::checked_coord(origin.row as i8, origin.col as i8 + 1),
                    Board::checked_coord(origin.row as i8, origin.col as i8 + 2),
                )
                    && board.check_move(&origin, &t, active_player).is_some()
                        && let Some(m) = board.check_move(&origin, &d, active_player) {
                            list.push(m);
                        }

        if rights.long
            && board[(row, col - 1)] == Cell::Free
                && board[(row, col - 2)] == Cell::Free
                && board[(row, col - 3)] == Cell::Free
                && let (Some(t), Some(d)) = (
                    Board::checked_coord(origin.row as i8, origin.col as i8 - 1),
                    Board::checked_coord(origin.row as i8, origin.col as i8 - 2),
                )
                    && board.check_move(&origin, &t, active_player).is_some()
                        && let Some(m) = board.check_move(&origin, &d, active_player) {
                            list.push(m);
                        }
    }
}
