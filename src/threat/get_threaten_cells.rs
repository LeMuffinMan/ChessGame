
//refacto : makes these recursive return a Vec<Coord> of all cells threaten : on the path, and once
//found an obstacle
fn get_threaten_cells_in_diag(from: &Coord, row : u8, col: u8, board: &mut Board)
{
    let vec = match board.grid[from.row as usize][from.col as usize].is_color(&White) {
        true => { &mut board.white_threatening_cells }
        false => { &mut board.black_threatening_cells }
    }; // i must refaco this 2 vectors in the structs 

    //Faire une fonction get_vec qui fait juste ce match ?
    let target: Coord = Coord { row, col };
    vec.push(Coord { row, col }); //we want to add it in any situation

    match (
        row.cmp(&(from.row as u8)),
        col.cmp(&(from.col as u8)),
        board.grid[target.row as usize][target.col as usize].is_empty(),
    ) {
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater, false) => {
            return get_threaten_cells_in_diag(from, row + 1, col + 1, board);
        }
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less, false) => {
            if col > 0 {
                return get_threaten_cells_in_diag(from, row + 1, col - 1, board);
            }
        }
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater, false) => {
            if row > 0 {
                return get_threaten_cells_in_diag(from, row - 1, col + 1, board);
            }
        }
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less, false) => {
            if row > 0 && col > 0 {
                return get_threaten_cells_in_diag(from, row - 1, col - 1, board);
            }
        }
        _ => {
            // println!("get_threaten_cells_in_diag : found obstacle in {} {}", target.row, target.col);
            return ;
        }
    }
}


fn get_threaten_cells_in_line(from: &Coord, row : u8, col: u8, board: &mut Board)
{
    if row > 7 || col > 7 { 
        return ;
    }
    //using the from cell we deduce in which vec we will push the new threaten cell
    let vec = match board.grid[from.row as usize][from.col as usize].is_color(&White) {
        true => { &mut board.white_threatening_cells }
        false => { &mut board.black_threatening_cells }
    }; // i must refaco this 2 vectors in the structs 


    let target: Coord = Coord { row, col };
    // println!("Pushing {:?} in vec", target);
    vec.push(Coord { row, col }); //we want to add it in any situation

    // destructured pattern matching :
    //This match compare a tuple of 3 elements: row diff, col diff and the color of the "from" cell
    //the enum returned by cmp gives us the direction to send the next recursive call
    //false filters the cases where the target is an obstacle : we stop the recursive
    //_ is here to ignore col or row and compare only one axe
    match (
        row.cmp(&(from.row as u8)), //We can use the returns of cmp : an enum { Greater, Less }
        col.cmp(&(from.col as u8)),
        board.grid[target.row as usize][target.col as usize].is_empty(),
    ) {
        (std::cmp::Ordering::Greater, _, true) => { 
            return get_threaten_cells_in_line(from, row + 1, col, board);
        }
        (std::cmp::Ordering::Less, _, true) => {
            if row > 0 {
                return get_threaten_cells_in_line(from, row - 1, col, board);
            }
        }
        (_, std::cmp::Ordering::Greater, true) => {
            return get_threaten_cells_in_line(from, row, col + 1, board);
        }
        (_, std::cmp::Ordering::Less, true) => {
            if row > 0 {
                return get_threaten_cells_in_line(from, row, col - 1, board);
            }
        }
        _ => {
            // println!("get_threaten_cells_in_line : found obstacle in {} {}", target.row, target.col);
            return ; //reaching this returns means we had an obstacle
        }
    }
}

