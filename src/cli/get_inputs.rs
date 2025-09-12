use crate::Board;
use crate::Color;
use std::io::{self, BufRead};
use crate::validate_move::try_move::mat_or_pat;
use crate::validate_move;
use crate::Coord;


pub fn run_cli() {
    let mut board = Board::init_board();

    let mut i = 1;
    let mut turn = 1;
    loop {
        let color = if i % 2 != 0 {
            if i != 1 {
                turn += 1;
            }
            Color::White
        } else {
            Color::Black
        };
        let (end, _mate) = mat_or_pat(&mut board, &color);
        if end {
            break;
        }
        println!("Turn {turn}");
        turn_begin(&board, &color);
        let (from_coord, to_coord) = get_move_from_stdin(color, &board);
        // println!("From {from_coord:?} to {to_coord:?}");
        if board.is_legal_move(&from_coord, &to_coord, &color) {
            if !validate_move::is_king_exposed(&from_coord, &to_coord, &color, &board) {
                println!("Move validated");
                board.update_board(&from_coord, &to_coord, &color);
            } else {
                println!("King is exposed : illegal move");
                continue;
            }
        } else {
            println!("Illegal move : {from_coord:?} -> {to_coord:?}");
            continue;
        }
        i += 1;
        println!("---------------------------------");
    }
}



fn turn_begin(board: &Board, color: &Color) {
    board.print();
    println!("{:?} to move", color);
    if let Some(coord) = board.get_king(color) {
        if board.threaten_cells.contains(&coord) {
            println!("Check !");
        }
    }
}

pub fn get_move_from_stdin(color: Color, board: &Board) -> (Coord, Coord) {
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
