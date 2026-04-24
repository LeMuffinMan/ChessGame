use crate::Coord;
use crate::board::Board;
use crate::board::pin_detection::PinInfos;
use crate::board::moves::move_gen::push_if_legal;
use crate::board::moves::move_structs::MoveList;
use crate::Color;

pub fn knight_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
    info: &PinInfos,
) {
    #[rustfmt::skip]
    let offsets = [
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2), (1, -2), (-1, 2), (-1, -2),
    ];
    for (dr, dc) in offsets {
        if let Some(dest) = Board::checked_coord(origin.row as i8 + dr, origin.col as i8 + dc) {
            if capture_only {
                let cell = board.grid[dest.row as usize][dest.col as usize];
                if !cell.is_color(active_player) && cell.get_piece().is_none() {
                    continue;
                }
            }
            push_if_legal(board, origin, dest, active_player, list, info);
        }
    }

}
