use crate::gui::chessapp::ChessApp;
use crate::gui::features::gamestate::DrawOption::*;
use crate::gui::features::gamestate::DrawRule;
use crate::gui::hooks::windows::End::Draw;
use crate::gui::hooks::windows::WinDia::*;

impl ChessApp {
    pub fn draw_buttons(&mut self, ui: &mut egui::Ui) {
        if self.current.draw.draw_option.is_some() {
            self.claim_draw_buttons(ui);
        } else if self.ask_draw(ui) {
            self.win = Some(DrawRequest);
        }
    }
    fn draw_rule(&mut self, ui: &mut egui::Ui, rule: &str) {
        if self.is_bot_turn() {
            self.current.end = Some(Draw);
            self.current.draw.draw_option = None;
        }
        ui.label(rule);
    }

    fn claim_draw_buttons(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        match self.current.draw.draw_option {
            Some(Available(DrawRule::TripleRepetition)) => {
                self.draw_rule(ui, "Triple repetition :")
            }
            Some(Available(DrawRule::FiftyMoves)) => self.draw_rule(ui, "50 moves :"),
            _ => {}
        };
        if ui.button("Claim draw").clicked() {
            self.current.end = Some(Draw);
            self.current.draw.draw_option = None;
        }
    }

    fn ask_draw(&mut self, ui: &mut egui::Ui) -> bool {
        ui.add_enabled(
            self.current.end.is_none() && self.win.is_none(),
            egui::Button::new("Draw"),
        )
        .clicked()
    }
}
