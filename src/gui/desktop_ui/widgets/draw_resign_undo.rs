use crate::gui::chessapp_struct::DrawOption::*;
use crate::gui::chessapp_struct::DrawRule::*;
use crate::gui::chessapp_struct::End::*;

use crate::ChessApp;

impl ChessApp {
    pub fn draw_resign_undo(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            //shows the rule triggering the draw
            if let Some(draw) = &self.draw.draw_option {
                match draw {
                    Available(TripleRepetition) => {
                        ui.label("Triple repetition :");
                    }
                    Available(FiftyMoves) => {
                        ui.label("%50 moves : ");
                    }
                    //ajouter les situations impossibles
                    _ => {}
                };
                //catch user inputs to ask for resign or draw to opponent using window_dialog
                if ui.button("Claim draw").clicked() {
                    self.current.end = Some(Draw);
                    self.draw.draw_option = None;
                }
            } else if ui
                .add_enabled(
                    self.current.end.is_none() && !self.win_dialog,
                    egui::Button::new("Draw"),
                )
                .clicked()
            {
                self.draw.draw_option = Some(Request);
            }
            if ui
                .add_enabled(
                    self.current.end.is_none() && !self.win_dialog,
                    egui::Button::new("Resign"),
                )
                .clicked()
            {
                self.win_resign = true;
            }
        });
    }
}
