// use crate::Color::*; //This import the enum Color and all its element
// use crate::Color; //Need to import the enum alone to resolve amiguous NONE (Pieces / Color)
use crate::Board;
use crate::Color;
use crate::Coord;
use crate::cell::Cell;
use crate::cell::Color::*;
use crate::cell::Piece;
use crate::get_threaten_cells::*;

//For each piece, we collect the cells it threatens, and we push it in the
//opponenent threats cell vector
//if a threaten cell has an ally piece, we still want to collect it, but we stop to
//seek in this direction
//if a king would try to take a protected pawn, iterating in this vec will be enough to
//reject the move
// println!("Checking cell ({}, {}) -> {:?}", row, col, cell.get_piece());
//
//Est-ce plus coherent d'en faire une impl de Board ?
pub fn update_threatens_cells(board: &mut Board, color: &Color) {
    board.threaten_cells.clear();
    // println!("call here");
    for row in 0..8 {
        for col in 0..8 {
            let coord = Coord {
                row: row as u8,
                col: col as u8,
            };
            let cell = board.get(&coord);
            if let Some(piece) = cell.get_piece()
                && !cell.is_color(color)
            {
                match piece {
                    Piece::Pawn => pawn_threats(&cell, &coord, color, board),
                    Piece::Rook => rook_threats(&coord, row, col, board),
                    Piece::Knight => knight_threats(&coord, color, &cell, board),
                    Piece::Bishop => bishop_threats(&coord, row, col, board),
                    Piece::Queen => { 
                        rook_threats(&coord, row, col, board);
                        bishop_threats(&coord, row, col, board);
                    },
                    Piece::King => king_threats(&coord, board),
                }
            } else {
                // println!("Skipping cell ({}, {}) -> {:?}", row, col, cell.get_piece());
                continue;
            }
        }
    }
}

fn pawn_threats(cell: &Cell, coord: &Coord, color: &Color, board: &mut Board) {
    let cells: [(i8, i8); 2] = if cell.is_color(&White) {
        [(1, -1), (1, 1)] 
    } else {
        [(-1, -1), (-1, 1)]
    };

    for (dr, dc) in cells {
        let new_row = coord.row as i8 + dr;
        let new_col = coord.col as i8 + dc;
        //clippy want me to do this instead comparing >= 0 and < 8
        if (0..8).contains(&new_row) && (0..8).contains(&new_col) {
            // println!("Pawn in ({}, {}) threats ({}, {})", coord.row, coord.col, new_row, new_col);
            if !cell.is_color(color) {
                board.threaten_cells.push(Coord {
                    row: new_row as u8,
                    col: new_col as u8,
                });
            }
            // println!("Pushing Coord  col: {} , row: {} in vec", new_row, new_col);
            // Coord { row: new_row as u8, col: new_col as u8 };
        }
    }
}
//                                      i8 or u8 or usize ?   too  many args, impl for board?
fn rook_threats(coord: &Coord, row: usize, col: usize, board: &mut Board) {
    if row < 7 {
        get_threaten_cells_in_line(coord, row as u8 + 1, col as u8, board);
    }
    if row > 0 {
        get_threaten_cells_in_line(coord, row as u8 - 1, col as u8, board);
    }
    if col < 7 {
        get_threaten_cells_in_line(coord, row as u8, col as u8 + 1, board);
    }
    if col > 0 {
        get_threaten_cells_in_line(coord, row as u8, col as u8 - 1, board);
    }
    //une recursive qui push dans les 4 directions si pas d'obstacle PUIS si
    //obstacle == advers
}

fn knight_threats(coord: &Coord, color: &Color, cell: &Cell, board: &mut Board) {
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

    for (dr, dc) in cells {
        let new_row = coord.row as i8 + dr;
        let new_col = coord.col as i8 + dc;

        //Need this to avoid panic on overflow
        if (0..8).contains(&new_row) && (0..8).contains(&new_col) {
            // println!("Pushing Coord  col: {} , row: {} in vec", new_row, new_col);
            if !cell.is_color(color) {

                board.threaten_cells.push(Coord {
                    row: new_row as u8,
                    col: new_col as u8,
                });
            }
        }
    }
}

fn bishop_threats(coord: &Coord, row: usize, col: usize, board: &mut Board) {
    if row < 7 && col < 7 {
        get_threaten_cells_in_diag(&coord, row as u8 + 1, col as u8 + 1, board);
    }
    if row < 7 && col > 0 {
        get_threaten_cells_in_diag(&coord, row as u8 + 1, col as u8 - 1, board);
    }
    if row > 0 && col < 7 {
        get_threaten_cells_in_diag(&coord, row as u8 - 1, col as u8 + 1, board);
    }
    if row > 0 && col > 0 {
        get_threaten_cells_in_diag(&coord, row as u8 - 1, col as u8 - 1, board);
    }
}

fn king_threats(coord: &Coord, board: &mut Board) {
    let cells: [(i8, i8); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dr, dc) in cells.iter() {
        let new_row = coord.row as i8 + dr;
        let new_col = coord.col as i8 + dc;

        if (0..8).contains(&new_row) && (0..8).contains(&new_col) {
            // println!("Pushing Coord  col: {} , row: {} in vec", new_row, new_col);
            board.threaten_cells.push(Coord {
                row: new_row as u8,
                col: new_col as u8,
            });
        }
    }
}

