

fn find_threat_on_path(color: &Color, board: &Board) -> bool {
    //trouver la direction du path
    //pour chaque case du path : checker si dans la liste des menacees par l'adverse
    //si on en trouve -> true
    //
}

fn is_cell_threaten(color: &Color, board: &Board) -> bool {
    //iterer sur toutes les cases menacees par la couleur adverse dans board : si un match => true
}


fn get_threaten_cells_in_line(from: &Coord, row : u8, col: u8, board: &Board)
{
    //using the from cell we deduce in which vec we will push the new threaten cell
    let vec = if board.grid[from.row][from.col].color == WHITE {
        baord.white_threatening_cells;
    } else {
        board.black_threatening_cells;
    }
    let target: Coord = { row, col };
    vec.push(target); //we want to add it in any situation
    if board.grid[from.row][from.col].color != NONE { //Only the none case calls the recursive
        if row > from.row {
            return get_threaten_cells_in_line(from, row + 1, col, board); 
        } else if row < from.row {
            return get_threaten_cells_in_line(from, row - 1, col, board); 
        } else if col > from.col {
            return get_threaten_cells_in_line(from, row, col + 1, board); 
        } else if col < from.col {
            return get_threaten_cells_in_line(from, row, col - 1, board); 
        }
    } else { //If its a WHITE or BLACK cell : it's an obstacle, no need to seek further
        return ;
    }
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

                    //possibilite de faire des macros pour ces lignes ?
                    get_threaten_cells_in_line(&coord, row + 1, col, &board);
                    get_threaten_cells_in_line(&coord, row - 1, col, &board);
                    get_threaten_cells_in_line(&coord, row, col + 1, &board);
                    get_threaten_cells_in_line(&coord, row, col + 1, &board);
                    get_threaten_cells_in_diag(&coord, row + 1, col + 1, &board);
                    get_threaten_cells_in_diag(&coord, row + 1, col - 1, &board);
                    get_threaten_cells_in_diag(&coord, row - 1, col + 1, &board);
                    get_threaten_cells_in_diag(&coord, row - 1, col - 1, &board);
                    //apeler les fcts pour bishop et rook

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
