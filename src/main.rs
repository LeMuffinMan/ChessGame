mod threat;
use crate::threat::update_threatens_cells;
use crate::update_threatens_cells::update_threatens_cells;
use threat::get_threaten_cells;
mod board;
use board::Board;
use board::Color;
mod get_inputs;
use get_inputs::Coord;
mod validate_move;

//TO DO
//- merge sur main et bloquer les pushs
//- refacto TOUT
//- commenter les doutes etc
//      - Casts ? declarer un i32 le board ?
//      - Unit tests ?
//      - Tests avec cargo et mon stdin qui accepte les pipes ?
//          - separe les tests qui doivent etre valides / les autres
//      - Iterator : perfs ? (update threats peut iterer differemment ?)
//      - Quelles fonctions doivent etre des impl ?
//          - is legal comme wrapper ou comme impl ?
//      - rangement des structs ?
//
//++ implementer draw rules

fn main() {
    let mut board = Board::init_board();

    let mut i = 1;
    loop {
        let color = if i % 2 != 0 {
            Color::White
        } else {
            Color::Black
        };
        if firsts_checks(&mut board, &color) {
            break;
        }
        println!("Turn {i}");
        turn_begin(&board, &color);
        let (from_coord, to_coord) = get_inputs::get_move_from_stdin(color, &board);
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
        println!("-----------------");
    }
}

fn firsts_checks(board: &mut Board, color: &Color) -> bool {
    board.promote_pawn(&Color::White);
    board.promote_pawn(&Color::Black);
    update_threatens_cells(board, &color);
    board.update_legals_moves(&color);
    // for coord in &board.threaten_cells {
    //     println!("Cell threaten : ({}, {})", coord.row, coord.col);
    // }
    if board.legals_moves.is_empty() {
        board.print();
        let king_cell = board.get_king(&color);
        if let Some(coord) = king_cell {
            if board.threaten_cells.contains(&coord) {
                let winner = if *color == Color::White {
                    Color::Black
                } else {
                    Color::White
                };
                println!("Checkmate ! {:?} win", winner);
            } else {
                println!("Pat");
            }
        }
        return true;
    }
    return false;
}

fn turn_begin(board: &Board, color: &Color) {
    board.print();
    println!("{:?} to move", color);
    if let Some(coord) = board.get_king(&color) {
        if board.threaten_cells.contains(&coord) {
            println!("Check !");
        }
    }
}
