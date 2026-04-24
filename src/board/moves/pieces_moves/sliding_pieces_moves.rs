use crate::Coord;
use crate::board::Board;
use crate::board::cell::Color;
use crate::board::pin_detection::PinInfos;
use crate::board::moves::move_structs::MoveList;
use crate::board::moves::move_gen::push_if_legal;


pub fn rook_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
    info: &PinInfos,
) {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    process_sliding_piece(
        origin,
        &directions,
        active_player,
        board,
        list,
        capture_only,
        info,
    );
}

pub fn bishop_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
    info: &PinInfos,
) {
    let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
    process_sliding_piece(
        origin,
        &directions,
        active_player,
        board,
        list,
        capture_only,
        info,
    );
}

pub fn queen_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
    info: &PinInfos,
) {
    bishop_moves(origin, active_player, board, list, capture_only, info);
    rook_moves(origin, active_player, board, list, capture_only, info);
}

fn process_sliding_piece(
    origin: &Coord,
    dirs: &[(i8, i8)],
    color: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
    info: &PinInfos,
) {
    for (dr, dc) in dirs {
        let (mut r, mut c) = (origin.row as i8 + dr, origin.col as i8 + dc);
        while let Some(dest) = Board::checked_coord(r, c) {
            let target = board.get(&dest);
            if target.is_color(color) {
                break;
            }

            if capture_only && target.get_piece().is_none() {
                r += dr;
                c += dc;
                continue;
            }

            push_if_legal(board, origin, dest, color, list, info);

            if target.get_piece().is_some() {
                break;
            }
            r += dr;
            c += dc;
        }
    }
}
