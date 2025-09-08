// use crate::Color::*; //This import the enum Color and all its element
// use crate::Color; //Need to import the enum alone to resolve amiguous NONE (Pieces / Color)
use crate::Board;
use crate::Color;
use crate::Coord;
use crate::cell::Cell;
use crate::cell::Piece;
use crate::cell::Color::*;
use crate::get_threaten_cells::*;

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

fn pawn_threats(cell: &Cell, coord: &Coord, color: &Color, board: &mut Board) {
    let cells: [(i8, i8); 2] = if cell.is_color(&White) {
        [(1, -1), (1, 1)] // blanc : vers le haut
    } else {
        [(-1, -1), (-1, 1)] // noir : vers le bas
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

//Est-ce plus coherent d'en faire une impl de Board ?
///for each piece
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
                //For each piece, we collect the cells it threatens, and we push it in the
                //opponenent threats cell vector
                //if a threaten cell has an ally piece, we still want to collect it, but we stop to
                //seek in this direction
                //if a king would try to take a protected pawn, iterating in this vec will be enough to
                //reject the move
                // println!("Checking cell ({}, {}) -> {:?}", row, col, cell.get_piece());
                match piece {
                    Piece::Pawn => pawn_threats(&cell, &coord, color, board),
                    Piece::Rook => rook_threats(&coord, row, col, board),
                    Piece::Knight => {
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

                                    // board.threaten_cells.push(Coord {
                                    //     row: new_row as u8,
                                    //     col: new_col as u8,
                                    // });
                                }
                            }
                        }
                    }
                    Piece::Bishop => {
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
                    Piece::Queen => {
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
                            // if let Some(last) = board.threaten_cells.last() {
                            // println!("pushed ({}, {})", last.row, last.col);
                            // println!("1 6 = {:?}", board.grid[1][6]);
                            // } else {
                            // println!("Le vecteur est vide");
                            // }
                        }
                        if col < 7 {
                            get_threaten_cells_in_line(&coord, row as u8, col as u8 + 1, board);
                        }
                        if col > 0 {
                            get_threaten_cells_in_line(&coord, row as u8, col as u8 - 1, board);
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
                    Piece::King => {
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
                }
            } else {
                // println!("Skipping cell ({}, {}) -> {:?}", row, col, cell.get_piece());
                continue;
            }
        }
    }
}
