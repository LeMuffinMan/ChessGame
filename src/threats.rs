

fn find_threat_on_path(color: &Color, board: &Board) -> bool {
    //trouver la direction du path
    //pour chaque case du path : checker si dans la liste des menacees par l'adverse
    //si on en trouve -> true
    //
}

fn is_cell_threaten(color: &Color, board: &Board) -> bool {
    //iterer sur toutes les cases menacees par la couleur adverse dans board : si un match => true
}

//je vais avoir besoin de le passer en mutable partout 
fn update_threatens_cells(board: &mut Board) {
    board.white_threatening_cells.clear();
    board.black_threatening_cells.clear();
    for row in 0..8 {
        for col in 0..8 {
            let cell = &board.grid[row][col];
            let mut coord = Coord { row, col }
            match cell.piece {
                PAWN => {
                    if cell.color == WHITE {
                       coord.col += 1;
                       board.white_threatening_cells.push(Coord);
                       coord.col -= 2;
                       board.white_threatening_cells.push(Coord);
                    }
                }
                ROOK => {
                    //une recursive qui ajoute dans les 4 directions si pas d'obstacle PUIS si
                    //obstacle == advers

                }
                KNIGHT => {
                    //hardocer les 8 possibilites

                }
                BISHOP => {
                    //une recursive qui ajoute dans les 4 directions si pas d'obstacle PUIS si
                    //obstacle == advers

                }
                QUEEN => {
                    //apeler les fcts pour bishop et rook

                }
                KING => {
                    //hardcoder les 8 possibilites
                }

            }
        }
    }
}
