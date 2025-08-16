

// fn find_threat_on_path(color: &Color, board: &Board) -> bool {
    //trouver la direction du path
    //pour chaque case du path : checker si dans la liste des menacees par l'adverse
    //si on en trouve -> true
    //
// }

// fn is_cell_threaten(color: &Color, board: &Board) -> bool {
    //iterer sur toutes les cases menacees par la couleur adverse dans board : si un match => true
// }

fn get_threaten_cells_in_diag(from: &Coord, row : u8, col: u8, board: &Board)
{
    //Faire une fonction get_vec qui fait juste ce match ?
    let vec = match board.grid[from.row as usize][from.col as usize].color {
        WHITE => &mut board.white_threatening_cells,
        BLACK => &mut board.black_threatening_cells,
        NONE => {
            println!("Invalid from cell"); 
            return;
        }
        _ => {
            println!("Error : get_threaten_cells_in_line : Unexpected \"from\" color"); // Faire remonter l'erreur ?
            return;
        }
    };
    let target: Coord = { row, col };
    vec.push(target); //we want to add it in any situation

    match (
        row.cmp(&(from.row as u8)),
        col.cmp(&(from.col as u8)),
        board.grid[target.row as usize][target.col as usize].color,
    ) {
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater, WHITE | BLACK) => {
            return get_threaten_cells_in_diag(from, row + 1, col + 1, board);
        }
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Lower, WHITE | BLACK) => {
            return get_threaten_cells_in_diag(from, row + 1, col - 1, board);
        }
        (std::cmp::Ordering::Lower, std::cmp::Ordering::Greater, WHITE | BLACK) => {
            return get_threaten_cells_in_diag(from, row - 1, col + 1, board);
        }
        (std::cmp::Ordering::Lower, std::cmp::Ordering::Lower, WHITE | BLACK) => {
            return get_threaten_cells_in_diag(from, row - 1, col - 1, board);
        }
        _ => {
            println!("Error : get_threaten_cells_in_diag : Unexpecte case in seek the next cell call");
        }
    }
}

fn get_threaten_cells_in_line(from: &Coord, row : u8, col: u8, board: &Board)
{
    //using the from cell we deduce in which vec we will push the new threaten cell
    let vec = match board.grid[from.row as usize][from.col as usize].color {
        WHITE => &mut board.white_threatening_cells,
        BLACK => &mut board.black_threatening_cells,
        NONE => {
            println!("Invalid from cell"); 
            return;
        }
        _ => {
            println!("Error : get_threaten_cells_in_line : Unexpected \"from\" color"); // Faire remonter l'erreur ?
            return;
        }
    };
    let target: Coord = { row, col };
    vec.push(target); //we want to add it in any situation

    // destructured pattern matching :
    //This match compare a tuple of 3 elements: row diff, col diff and the color of the "from" cell
    //the enum returned by cmp gives us the direction to send the next recursive call
    //WHITE | BLACK filters the cases where the target is an obstacle : we stop the recursive
    //_ is here to ignore col or row and compare only one axe
    match (
        row.cmp(&(from.row as u8)), //We can use the returns of cmp : an enum { Greater, Less }
        col.cmp(&(from.col as u8)),
        board.grid[target.row as usize][target.col as usize].color,
    ) {
        (std::cmp::Ordering::Greater, _, WHITE | BLACK) => { 
            return get_threaten_cells_in_line(from, row + 1, col, board);
        }
        (std::cmp::Ordering::Less, _, WHITE | BLACK) => {
            return get_threaten_cells_in_line(from, row - 1, col, board);
        }
        (_, std::cmp::Ordering::Greater, WHITE | BLACK) => {
            return get_threaten_cells_in_line(from, row, col + 1, board);
        }
        (_, std::cmp::Ordering::Less, WHITE | BLACK) => {
            return get_threaten_cells_in_line(from, row, col - 1, board);
        }
        _ => {
            println!("Error : get_threaten_cells_in_line : Unexpecte case in seek the next cell call");
        }
    }
    return ; //reaching this returns means we had an ostacle
}
//Est-ce plus coherent d'en faire une impl de Board ?
//je vais avoir besoin de le passer en mutable partout 
fn update_threatens_cells(board: &mut Board) {
    board.white_threatening_cells.clear();
    board.black_threatening_cells.clear();
    for row in 0..8 {
        for col in 0..8 {
            let cell = &board.grid[row][col];
            //we skip the empty cells
            if cell.piece == NONE { continue; }
            let mut coord = Coord { row, col }
            //we want 2 maps of the threaten cells
            let vec = if cell.color == WHITE {
                board.white_threatening_cells;
            } else {
                board.black_threatening_cells;
            }
            //For each piece, we collect threaten cells
            //if a threaten cell has an ally piece, we still want to collect it
            //if a king would try to take a protected pawn, iterating in this vec will be enough to
            //reject the move
            match cell.piece {
                PAWN => {
                    if cell.color == WHITE {
                        coord.row += 1;
                    } else {
                        coord.row -= 1;
                    }
                    coord.col += 1;
                    vec.push(coord);
                    coord.col -= 2;
                    vec.push(coord);
                }
                ROOK => {
                    //possibilite de faire des macros pour ces lignes ?
                    get_threaten_cells_in_line(&coord, row + 1, col, &board);
                    get_threaten_cells_in_line(&coord, row - 1, col, &board);
                    get_threaten_cells_in_line(&coord, row, col + 1, &board);
                    get_threaten_cells_in_line(&coord, row, col + 1, &board);
                    //une recursive qui ajoute dans les 4 directions si pas d'obstacle PUIS si
                    //obstacle == advers

                }
                KNIGHT => {
                    coord.row += 2;
                    coord.col += 1;
                    vec.push(coord);
                    coord.col -= 2;
                    vec.push(coord);
                    coord.row -= 4;
                    vec.push(coord);
                    coord.col += 2;
                    vec.push(coord);
                    coord.col += 1;
                    coord.row += 1;
                    vec.push(coord);
                    coord.row += 2;
                    vec.push(coord);
                    coord.col -= 4;
                    vec.push(coord);
                    coord.col -= 1;
                    vec.push(coord);
                    //trouver plus propre
                }
                BISHOP => {
                    get_threaten_cells_in_diag(&coord, row + 1, col + 1, &board);
                    get_threaten_cells_in_diag(&coord, row + 1, col - 1, &board);
                    get_threaten_cells_in_diag(&coord, row - 1, col + 1, &board);
                    get_threaten_cells_in_diag(&coord, row - 1, col - 1, &board);
                    //une recursive qui ajoute dans les 4 directions si pas d'obstacle PUIS si
                    //obstacle == advers

                }
                QUEEN => {

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
                    get_threaten_cells_in_line(&coord, row + 1, col, &board);
                    get_threaten_cells_in_line(&coord, row - 1, col, &board);
                    get_threaten_cells_in_line(&coord, row, col + 1, &board);
                    get_threaten_cells_in_line(&coord, row, col + 1, &board);
                    get_threaten_cells_in_diag(&coord, row + 1, col + 1, &board);
                    get_threaten_cells_in_diag(&coord, row + 1, col - 1, &board);
                    get_threaten_cells_in_diag(&coord, row - 1, col + 1, &board);
                    get_threaten_cells_in_diag(&coord, row - 1, col - 1, &board);

                }
                KING => {
                    coord.row -= 1; //si il est au bord ? 
                    vec.push(coord)
                    coord.col -= 1;
                    vec.push(coord)
                    coord.col += 2;
                    vec.push(coord)
                    coord.row += 1;
                    vec.push(coord)
                    coord.col -= 2;
                    vec.push(coord)
                    coord.row += 1;
                    vec.push(coord);
                    coord.col += 1;
                    vec.push(coord);
                    coord.col += 1;
                    vec.push(coord);
                    //solution plus propre ?
                    //hardcoder les 8 possibilites
                }

            }
        }
    }
}
