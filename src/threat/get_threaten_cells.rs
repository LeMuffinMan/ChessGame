use crate::Board;
use crate::Coord;
// use crate::board::Cell;
// use crate::board::Color::*;

//refacto : makes these recursive return a Vec<Coord> of all cells threaten : on the path, and once
//found an obstacle

pub fn get_threaten_cells_in_diag(from: &Coord, row: u8, col: u8, board: &mut Board) {
    if row > 7 || col > 7 {
        return;
    }

    let target = Coord { row, col };
    board.threaten_cells.push(target);

    if !board.get(&target).is_empty() {
        return;
    }

    match (row.cmp(&from.row), col.cmp(&from.col)) {
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
            if row < 7 && col < 7 {
                get_threaten_cells_in_diag(from, row + 1, col + 1, board);
            }
        }
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
            if row < 7 && col > 0 {
                get_threaten_cells_in_diag(from, row + 1, col - 1, board);
            }
        }
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
            if row > 0 && col < 7 {
                get_threaten_cells_in_diag(from, row - 1, col + 1, board);
            }
        }
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => {
            if row > 0 && col > 0 {
                get_threaten_cells_in_diag(from, row - 1, col - 1, board);
            }
        }
        _ => {}
    }
}



pub fn get_threaten_cells_in_line(from: &Coord, row: u8, col: u8, board: &mut Board) {
    if row > 7 || col > 7 {
        return;
    }

    let target = Coord { row, col };
    board.threaten_cells.push(target);

    // si on a trouvé une pièce, on s'arrête
    if !board.get(&target).is_empty() {
        return;
    }

    match (
        row.cmp(&from.row),
        col.cmp(&from.col),
    ) {
        (std::cmp::Ordering::Greater, _) => {
            if row < 7 {
                get_threaten_cells_in_line(from, row + 1, col, board);
            }
        }
        (std::cmp::Ordering::Less, _) => {
            if row > 0 {
                get_threaten_cells_in_line(from, row - 1, col, board);
            }
        }
        (_, std::cmp::Ordering::Greater) => {
            if col < 7 {
                get_threaten_cells_in_line(from, row, col + 1, board);
            }
        }
        (_, std::cmp::Ordering::Less) => {
            if col > 0 {
                get_threaten_cells_in_line(from, row, col - 1, board);
            }
        }
        _ => {}
    }
}

