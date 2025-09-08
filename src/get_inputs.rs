use crate::Board;
use crate::Color;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coord {
    pub col: u8, //declarer de base des u8 ?
    pub row: u8,
}

pub fn get_move_from_stdin(color: Color, board: &Board) -> (Coord, Coord) {
    use std::io::{self, BufRead}; // en haut ou dans la fonction ?

    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        line.clear();
        if stdin.lock().read_line(&mut line).unwrap() == 0 {
            println!("EOF");
            std::process::exit(0);
        }

        let input = line.trim();
        if input.len() != 4 {
            println!("Invalid move format, must be like e2e4");
            continue;
        }

        let from_str = &input[0..2];
        let to_str = &input[2..4];

        let from = match get_coord_from_string(from_str.to_string()) {
            Ok(c) => c,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        let to = match get_coord_from_string(to_str.to_string()) {
            Ok(c) => c,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        if !board.get(&from).is_color(&color) {
            println!("No {color:?} piece in {from_str}");
            continue;
        }
        if board.get(&to).is_color(&color) {
            println!("There is already a {color:?} piece in {to_str}");
            continue;
        }

        return (from, to);
    }
}

//une impl de Coord form string
pub fn get_coord_from_string(cell: String) -> Result<Coord, String> {
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
        return Err(format!("Invalid input: out of board : {row} {col}"));
    }
    let coord = Coord { col, row };
    Ok(coord)
}

