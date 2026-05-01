use chess_game::gui::{chessapp::ChessApp, layout::UiType};

fn main() {
    eframe::run_native(
        "ChessGame",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(ChessApp::new(UiType::Desktop)))),
    )
    .unwrap();
}
