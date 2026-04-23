use crate::gui::chessapp::ChessApp;

impl ChessApp {
    pub fn new_game_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(
                !self.history.snapshots.is_empty(),
                egui::Button::new("New game"),
            )
            .clicked()
        {
            //revoir : ne pas changer les settings !
            *self = ChessApp::new(self.ui_type.clone());
        }
    }
}
