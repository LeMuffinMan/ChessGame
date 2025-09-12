mod threat;
use threat::get_threaten_cells;
use crate::board::cell::Color;
mod board;
use board::Board;
mod cli;
use crate::cli::get_inputs::Coord;
use crate::cli::get_inputs::run_cli;
mod gui;
mod validate_move;
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
//
//gui
//  side panel
//      pieces took
//      replay + flip to fix
//
//      import pgn
//          decoder pgn
//
//      versus
//          deux joueurs
//          serie de parties
//
//

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--cli".to_string()) {
        run_cli();
    } else {
        run_gui();
    }
}

fn run_gui() {
    let app = ChessApp::default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]) // window size
            .with_min_inner_size([500.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native("ChessGame", options, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
}


