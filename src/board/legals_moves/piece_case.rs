use crate::Board;
use crate::Color;
use crate::Color::*;
use crate::Coord;

pub fn update_pawn_legals_moves(
    from: &Coord,
    color: &Color,
    board: &mut Board,
) -> Vec<(Coord, Coord)> {
    let dir: i8 = if *color == White { 1 } else { -1 };
    let mut ret = Vec::new();
    //2 diagonales
    if let Some(to) = Board::checked_coord(from.row as i8 + dir, from.col as i8 + 1) {
        if let Some((_, _)) = board.test_and_push(from, &to, color) {
            ret.push((*from, to));
        }
    }
    if let Some(to) = Board::checked_coord(from.row as i8 + dir, from.col as i8 - 1) {
        if let Some((_, _)) = board.test_and_push(from, &to, color) {
            ret.push((*from, to));
        }
    }
    //2 straight forward
    if let Some(to) = Board::checked_coord(from.row as i8 + dir, from.col as i8) {
        if let Some((_, _)) = board.test_and_push(from, &to, color) {
            ret.push((*from, to));
        }
    }
    if let Some(to) = Board::checked_coord(from.row as i8 + dir + dir, from.col as i8) {
        if let Some((_, _)) = board.test_and_push(from, &to, color) {
            ret.push((*from, to));
        }
    }
    return ret;
}

pub fn update_rook_legals_moves(
    from: &Coord,
    color: &Color,
    board: &mut Board,
) -> Vec<(Coord, Coord)> {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut ret = Vec::new();

    for (dr, dc) in directions {
        let mut r = from.row as i8 + dr;
        let mut c = from.col as i8 + dc;

        while let Some(to) = Board::checked_coord(r, c) {
            let target = board.get(&to);

            if target.is_color(color) {
                break;
            }
            if let Some((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }

            r += dr;
            c += dc;
        }
    }
    return ret;
}

pub fn update_knight_legals_moves(
    from: &Coord,
    color: &Color,
    board: &mut Board,
) -> Vec<(Coord, Coord)> {
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
    let mut ret = Vec::new();

    for (dr, dc) in cells {
        let new_row = from.row as i8 + dr;
        let new_col = from.col as i8 + dc;
        if let Some(to) = Board::checked_coord(new_row, new_col) {
            if let Some((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }
        }
    }
    return ret;
}

pub fn update_bishop_legals_moves(
    from: &Coord,
    color: &Color,
    board: &mut Board,
) -> Vec<(Coord, Coord)> {
    let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
    let mut ret = Vec::new();

    for (dr, dc) in directions {
        let mut r = from.row as i8 + dr;
        let mut c = from.col as i8 + dc;

        while let Some(to) = Board::checked_coord(r, c) {
            let target = board.get(&to);

            if target.is_color(color) {
                break;
            }
            if let Some((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }

            r += dr;
            c += dc;
        }
    }
    return ret;
}

pub fn update_queen_legals_moves(
    from: &Coord,
    color: &Color,
    board: &mut Board,
) -> Vec<(Coord, Coord)> {
    let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
    let mut ret = Vec::new();

    for (dr, dc) in directions {
        let mut r = from.row as i8 + dr;
        let mut c = from.col as i8 + dc;

        while let Some(to) = Board::checked_coord(r, c) {
            let target = board.get(&to);

            if target.is_color(color) {
                break;
            }
            if let Some((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }

            r += dr;
            c += dc;
        }
    }
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (dr, dc) in directions {
        let mut r = from.row as i8 + dr;
        let mut c = from.col as i8 + dc;

        while let Some(to) = Board::checked_coord(r, c) {
            let target = board.get(&to);

            if target.is_color(color) {
                break;
            }
            if let Some((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }

            r += dr;
            c += dc;
        }
    }
    return ret;
}

//tester les roques
pub fn update_king_legals_moves(
    from: &Coord,
    color: &Color,
    board: &mut Board,
) -> Vec<(Coord, Coord)> {
    let cells: [(i8, i8); 8] = [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut ret = Vec::new();

    for (dr, dc) in cells {
        let new_row = from.row as i8 + dr;
        let new_col = from.col as i8 + dc;
        if let Some(to) = Board::checked_coord(new_row, new_col) {
            if let Some((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }
        }
    }
    //castle
    if let None = board.check {
        let little_castle = from.col as i8 + 2;
        let long_castle = from.col as i8 - 2;
        if let Some(to) = Board::checked_coord(from.row as i8, little_castle) {
            if let Some((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }
        }
        if let Some(to) = Board::checked_coord(from.row as i8, long_castle) {
            if let Some((_, _)) = board.test_and_push(from, &to, color) {
                ret.push((*from, to));
            }
        }
    }
    return ret;
}
