use crate::Board;
use crate::Coord;
// use crate::board::Cell;
// use crate::board::Color::*;

//refacto : makes these recursive return a Vec<Coord> of all cells threaten : on the path, and once
//found an obstacle
pub fn get_threaten_cells_in_diag(from: &Coord, row: u8, col: u8, board: &mut Board) {
    let target: Coord = Coord { row, col };
    board.threaten_cells.push(Coord { row, col }); //we want to add it in any situation

    match (
        row.cmp(&from.row),
        col.cmp(&from.col),
        board.grid[target.row as usize][target.col as usize].is_empty(),
    ) {
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater, false) => {
            get_threaten_cells_in_diag(from, row + 1, col + 1, board);
        }
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less, false) => {
            if col > 0 {
                get_threaten_cells_in_diag(from, row + 1, col - 1, board);
            }
        }
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater, false) => {
            if row > 0 {
                get_threaten_cells_in_diag(from, row - 1, col + 1, board);
            }
        }
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less, false) => {
            if row > 0 && col > 0 {
                get_threaten_cells_in_diag(from, row - 1, col - 1, board);
            }
        }
        _ => {}
    };
}

pub fn get_threaten_cells_in_line(from: &Coord, row: u8, col: u8, board: &mut Board) {
    if row > 7 || col > 7 {
        return;
    }

    let target: Coord = Coord { row, col };
    // println!("Pushing {:?} in vec", target);
    board.threaten_cells.push(Coord { row, col }); //we want to add it in any situation

    // destructured pattern matching :
    //This match compare a tuple of 3 elements: row diff, col diff and the color of the "from" cell
    //the enum returned by cmp gives us the direction to send the next recursive call
    //false filters the cases where the target is an obstacle : we stop the recursive
    //_ is here to ignore col or row and compare only one axe
    match (
        row.cmp(&from.row), //We can use the returns of cmp : an enum { Greater, Less }
        col.cmp(&from.col),
        board.grid[target.row as usize][target.col as usize].is_empty(),
    ) {
        (std::cmp::Ordering::Greater, _, true) => {
            get_threaten_cells_in_line(from, row + 1, col, board);
        }
        (std::cmp::Ordering::Less, _, true) => {
            if row > 0 {
                get_threaten_cells_in_line(from, row - 1, col, board);
            }
        }
        (_, std::cmp::Ordering::Greater, true) => {
            get_threaten_cells_in_line(from, row, col + 1, board);
        }
        (_, std::cmp::Ordering::Less, true) => {
            if row > 0 {
                get_threaten_cells_in_line(from, row, col - 1, board);
            }
        }
        _ => {}
    }
}
