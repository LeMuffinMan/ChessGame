mod threat;
use threat::get_threaten_cells;

mod board;
use crate::board::cell::Color;
use crate::board::cell::Coord;
use board::Board;

mod cli;
use crate::cli::get_inputs::run_cli;

mod gui;
use crate::gui::chessapp_struct::ChessApp;

mod pgn;

mod validate_move;

//TO DO
//- fmt + clippy
//- Unit tests ?
//- Pipeline de tests end to end ?
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
