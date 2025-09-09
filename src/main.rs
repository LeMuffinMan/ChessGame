mod threat;
use crate::threat::update_threatens_cells;
use crate::update_threatens_cells::update_threatens_cells;
use threat::get_threaten_cells;
mod cell;
use cell::Color;
mod board;
use board::Board;
mod get_inputs;
use get_inputs::Coord;
mod validate_move;
mod gui;
use crate::gui::chessapp_struct::ChessApp;

//TO DO
//- fmt + clippy puis merge sur main et bloquer les pushs
//- refacto TOUT
//  - Comment organiser board.rs avec les enormes methodes ?
//  - TOUT en methodes ?
//- Unit tests ?
//- Pipeline de tests end to end ?
//- commenter les doutes etc
//      - Casts ? declarer un i32 le board / les coords ?
//      - Quelles fonctions doivent etre des impl ?
//          - is legal comme wrapper ou comme impl ?
//
//++ implementer draw rules
//++ roque : checker la verif de la menace sur les cases separant roi et tour

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--gui".to_string()) {
        run_gui();
    } else {
        run_cli();
    }
}

fn run_gui() {
    let app = ChessApp::default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 1000.0]) // fenêtre plus grande
            .with_min_inner_size([700.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native("ChessGame", options, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
}

fn run_cli() {
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
        if mat_or_pat(&mut board, &color) {
            break;
        }
        println!("Turn {turn}");
        turn_begin(&board, &color);
        let (from_coord, to_coord) = get_inputs::get_move_from_stdin(color, &board);
        // println!("From {from_coord:?} to {to_coord:?}");
        // comparer avec la liste des coups legaux
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

pub fn mat_or_pat(board: &mut Board, color: &Color) -> bool {
    if *color == Color::White {
        board.promote_pawn(&Color::Black);
    } else {
        board.promote_pawn(&Color::White);
    }
    update_threatens_cells(board, color);
    board.update_legals_moves(color);
    // for coord in &board.threaten_cells {
    //     println!("Cell threaten : ({}, {})", coord.row, coord.col);
    // }
    if board.legals_moves.is_empty() {
        board.print();
        let king_cell = board.get_king(color);
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
    false
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
