use std::io;

use crate::Board;
use crate::Color;

#[derive(Debug, PartialEq)]
pub struct Coord {
    pub col: u8,
    pub row: u8,
}

///translate pgn code into regular coordinates, with minimal error management
fn get_coord_from_string(cell: String) -> Result<Coord, String> {
    if cell.len() != 2 {
        return Err(format!("Invalid input size : {}", cell.len()));
    }
    let mut chars = cell.chars();
    let col = match chars.next() {
        Some(c) => c.to_ascii_lowercase() as u8 - b'a',
        None => return Err("Invalid input: error parsing column".to_string()),
    };
    let row = match chars.next() {
        Some(c) => c as u8 - b'0' - 1,
        None => return Err("Invalid input: error parsing row".to_string()),
    };
    if col > 7 || row > 7 {
        return Err(format!("Invalid input: out of board : {} {}", row, col));
    }
    let coord = Coord {col, row};
    Ok(coord)
}

///return a struct coord after reading the input from stdin
pub fn get_inputs(msg: &str, color: Color, board: &Board) -> Coord {
    return loop {
        let mut input = String::new();
        println!("{} cell :", msg);
        io::stdin().read_line(&mut input).expect("Error");
        let input = input.trim();
        match get_coord_from_string(input.to_string()) {
            Ok(coord) => {
                if msg == "from" && color != board.grid[coord.row as usize][coord.col as usize].color {
                    println!("No {:?} piece in {}", color, input);
                    continue;
                }
                if  msg == "to" && color == board.grid[coord.row as usize][coord.col as usize].color {
                    println!("There is already a {:?} piece in {}", color, input);
                    continue;
                }
                break coord
            }
            Err(e) => {
                println!("{e}");
                continue;
            }
        }
    }
}
