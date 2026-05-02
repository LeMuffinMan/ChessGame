use crate::Coord;
use crate::board::Board;
use crate::board::cell::Cell;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Piece::*;
use crate::board::moves::move_structs::MoveType;
use crate::board::moves::move_gen::push_if_legal;
use crate::board::pin_detection::PinInfos;
use crate::board::moves::move_gen::aligned_with_pin;
use crate::board::moves::move_structs::MoveList;

pub fn pawn_moves(
    origin: &Coord,
    active_player: &Color,
    board: &mut Board,
    list: &mut MoveList,
    capture_only: bool,
    info: &PinInfos,
) {
    pawn_diag_en_passant(board, *origin, active_player, list, info);

    if capture_only {
        return;
    }

    pawn_move_forward(board, *origin, active_player, list, info);
}

pub fn pawn_move_forward(board: &mut Board, origin: Coord, active_player: &Color, list: &mut MoveList, info: &PinInfos) {
    let dir = if *active_player == White { 1 } else { -1 };
    let last_rank = if *active_player == White { 7 } else { 0 };

    if let Some(dest) = Board::checked_coord(origin.row as i8 + dir, origin.col as i8)
        && board.get(&dest) == Cell::Free {
            push_pawn_dest(&origin, dest, *active_player, board, last_rank, list, info);

            let initial_row = if *active_player == White {
                origin.row == 1
            } else {
                origin.row == 6
            };
            if initial_row
                && let Some(dest2) =
                    Board::checked_coord(origin.row as i8 + dir * 2, origin.col as i8)
                    && board.get(&dest2) == Cell::Free {
                        push_if_legal(board, &origin, dest2, active_player, list, info);
                    }
        }
}

pub fn pawn_diag_en_passant(board: &mut Board, origin: Coord, active_player: &Color, list: &mut MoveList, info: &PinInfos) {
    let dir = if *active_player == White { 1 } else { -1 };
    let last_rank = if *active_player == White { 7 } else { 0 };

    for dc in [1, -1] {
        if let Some(dest) = Board::checked_coord(origin.row as i8 + dir, origin.col as i8 + dc) {
            let target = board.get(&dest);
            let is_enemy = target.get_piece().is_some() && !target.is_color(active_player);
            let is_ep = board.en_passant == Some(dest);

            if is_enemy || is_ep {
                push_pawn_dest(&origin, dest, *active_player, board, last_rank, list, info);
            }
        }
    }
}

pub fn push_pawn_dest(
    origin: &Coord,
    dest: Coord,
    color: Color,
    board: &mut Board,
    last_rank: u8,
    list: &mut MoveList,
    info: &PinInfos,
) {
    if dest.row == last_rank {
        let is_legal = if info.checker_count >= 1 {
            board.check_move(origin, &dest, &color).is_some()
        } else if let Some((dr, dc)) = info.pins[origin.row as usize][origin.col as usize] {
            aligned_with_pin(origin, &dest, dr, dc)
        } else {
            true
        };

        if is_legal {
            for p in [Queen, Rook, Bishop, Knight] {
                // create_move gère capture + prev_score correctement
                let mut m = board.create_move(*origin, dest, color, MoveType::Promotion(Queen));
                m.move_type = MoveType::Promotion(p);
                list.push(m);
            }
        }
    } else {
        let is_ep = board.en_passant == Some(dest);
        if is_ep {
            if let Some(m) = board.check_move(origin, &dest, &color) {
                list.push(m);
            }
        } else {
            push_if_legal(board, origin, dest, &color, list, info);
        }
    }
}
