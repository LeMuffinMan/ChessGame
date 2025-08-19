// use crate::Color::*; //This import the enum Color and all its element
// use crate::Color; //Need to import the enum alone to resolve amiguous NONE (Pieces / Color)
use crate::Coord;
use crate::Board;
use crate::board::Cell;
// use crate::Piece::*;
use crate::board::Piece;
use crate::board::{Color, Color::*, Piece::*};


// fn find_threat_on_path(color: &Color, board: &Board) -> bool {
    //trouver la direction du path
    //pour chaque case du path : checker si dans la liste des menacees par l'adverse
    //si on en trouve -> true
    //
// }

// fn is_cell_threaten(color: &Color, board: &Board) -> bool {
    //iterer sur toutes les cases menacees par la couleur adverse dans board : si un match => true
// }

// pub fn find_threat_on_path(from: &Coord, to: &Coord, board: &Board) -> bool {
    //Savoir dans quel direction on va
    //iterer dans cette direction
    //au premier vec.contains(target) == true : on stop on return true
    //sinon return false
//     false
// }

//either we give the accurate vector to each recursive so they add in it
//or i collect from them vectors, that i push in last move ?


fn pawn_threats(cell: &Cell, coord: &Coord) -> Vec<Coord> {
    let mut threatened_cells = Vec::new();
    let cells: [(i8, i8); 2] = if cell.is_color(&White) {
            [(1, -1), (1, 1)]   // blanc : vers le haut
        } else {
            [(-1, -1), (-1, 1)] // noir : vers le bas
        };

    for (dr, dc) in cells {
        let new_row = coord.row as i8 + dr;
        let new_col = coord.col as i8 + dc;

        if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
            threatened_cells.push(Coord { row: new_row as u8, col: new_col as u8 });
            // println!("Pushing Coord  col: {} , row: {} in vec", new_row, new_col);
            // Coord { row: new_row as u8, col: new_col as u8 };
        }
    threatened_cells
    }
}

fn rook_threats(cell: &Cell, coord: &Coord) -> Vec<Coord> {
    let mut threatened_cells = Vec::new();
    if row < 7 {
        get_threaten_cells_in_line(&coord, row as u8 + 1, col as u8, board);
    }
    if row > 0 {
        get_threaten_cells_in_line(&coord, row as u8 - 1, col as u8, board);
    }
    if col < 7 {
        get_threaten_cells_in_line(&coord, row as u8, col as u8 + 1, board);
    }
    if col > 0 {
        get_threaten_cells_in_line(&coord, row as u8, col as u8 - 1, board);
    }
    //une recursive qui push dans les 4 directions si pas d'obstacle PUIS si
    //obstacle == advers
}




//Est-ce plus coherent d'en faire une impl de Board ?
pub fn update_threatens_cells(board: &mut Board) {
    board.white_threatening_cells.clear();
    board.black_threatening_cells.clear();
    for row in 0..8 {
        for col in 0..8 {
            let cell = &board.grid[row][col];
            //we skip the empty cells
            if cell.is_empty() { continue; }
            // println!("\n[Updating threathens in cell {} {} containing {:?}]", row, col, board.grid[row][col].piece);
            let coord = Coord { row: row as u8, col: col as u8 };
            //we want 2 maps of the threaten cells
            let vec = match board.grid[row as usize][col as usize].is_color(&White) {
                true => { &mut board.white_threatening_cells }
                false => { &mut board.black_threatening_cells }
            }; // i must refaco this 2 vectors in the structs 

            //For each piece, we collect the cells this piece threatens, and we push it in the
            //opponenent threats cell vector
            //if a threaten cell has an ally piece, we still want to collect it, but we stop to
            //seek in this direction
            //if a king would try to take a protected pawn, iterating in this vec will be enough to
            //reject the move
            match cell.get_piece() {
                Some(Piece::Pawn) => vec.extend(pawn_threats(&cell, &coord)),
                Some(Piece::Rook) => {


                }
                Some(Piece::Knight) => {
                    let cells: [(i8, i8); 8] = [
                        (2, 1), (2, -1),
                        (-2, 1), (-2, -1),
                        (1, 2), (1, -2),
                        (-1, 2), (-1, -2),
                    ];

                    for (dr, dc) in cells {
                        let new_row = coord.row as i8 + dr;
                        let new_col = coord.col as i8 + dc;

                        //Need this to avoid panic on overflow
                        if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                            // println!("Pushing Coord  col: {} , row: {} in vec", new_row, new_col);
                            vec.push(Coord { row: new_row as u8, col: new_col as u8 });
                        }
                    }
                }
                Some(Piece::Bishop) => {
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
                    //une recursive qui ajoute dans les 4 directions si pas d'obstacle PUIS si
                    //obstacle == advers

                }
                Some(Piece::Queen) => {

                    // let line_dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                    // let diag_dirs = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
                    //
                    // for (dr, dc) in line_dirs {
                    //     get_threaten_cells_in_line(&coord, row.wrapping_add(dr as u8), col.wrapping_add(dc as u8), &board);
                    // }
                    //
                    // for (dr, dc) in diag_dirs {
                    //     get_threaten_cells_in_diag(&coord, row.wrapping_add(dr as u8), col.wrapping_add(dc as u8), &board);
                    // }


                    //possibilite de faire des macros pour ces lignes ?
                    if row < 7 {
                        get_threaten_cells_in_line(&coord, row as u8 + 1, col as u8, board);
                    }
                    if row > 0 {
                        get_threaten_cells_in_line(&coord, row as u8 - 1, col as u8, board);
                    }
                    if col < 7 {
                        get_threaten_cells_in_line(&coord, row as u8, col as u8 + 1, board);
                    }
                    if col > 0 {
                        get_threaten_cells_in_line(&coord, row as u8, col as u8 + 1, board);
                    }
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
                Some(Piece::King) => {
                    let cells: [(i8, i8); 8] = [
                        (-1, -1), (-1, 0), (-1, 1),
                        (0, -1),           (0, 1),
                        (1, -1),  (1, 0),  (1, 1),
                    ];

                    for (dr, dc) in cells.iter() {
                        let new_row = coord.row as i8 + dr;
                        let new_col = coord.col as i8 + dc;

                        if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                            // println!("Pushing Coord  col: {} , row: {} in vec", new_row, new_col);
                            vec.push(Coord { row: new_row as u8, col: new_col as u8 });
                        }
                    }
                }
                None => { continue; }
            }
        }
    }
}
