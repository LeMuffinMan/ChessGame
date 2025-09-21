use crate::ChessApp;
use crate::Color::*;
use crate::gui::chessapp_struct::End::Draw;
use crate::gui::game_state_struct::DrawOption;
use crate::gui::hooks::WinDia;
use crate::gui::hooks::WinDia::*;
use crate::gui::mobile_ui::mobile_buttons::versus_buttons::DrawOption::Available;

impl ChessApp {
    pub fn draw_resign_undo_buttons(&mut self, ui: &mut egui::Ui) {
        ui.add_space(60.0);
        self.settings_button(ui);
        ui.add_space(150.0);
        #[allow(clippy::collapsible_else_if)]
        if let Some(option) = &self.current.draw.draw_option {
            #[allow(clippy::collapsible_else_if)]
            if let Available(_) = option {
                #[allow(clippy::collapsible_else_if)]
                if ui.button("Claim draw").clicked() {
                    self.current.end = Some(Draw);
                }
            };
        } else {
            if ui.button("Draw").clicked() {
                self.win = Some(DrawRequest);
            }
        }
        ui.add_space(20.0);
        if ui.button("Resign").clicked() {
            self.win = Some(Resign);
        }
        if let Some(option) = &self.current.draw.draw_option
            && let DrawOption::Available(_) = option
        {
            ui.add_space(60.0);
        } else {
            ui.add_space(150.0);
        }
        #[allow(clippy::collapsible_if)]
        if self.settings.allow_undo
            && (self.settings.white_undo > 0 || self.settings.black_undo > 0)
        {
            #[allow(clippy::collapsible_if)]
            if ui
                .add_enabled(
                    self.current.end.is_none()
                        && self.can_undo()
                        && self.win.is_none()
                        && (self.history.snapshots.len() > 1
                            || self.history.snapshots.len() == 2
                                && self.current.active_player == White),
                    egui::Button::new("Undo"),
                )
                .clicked()
            {
                self.win = Some(WinDia::Undo);
                match self.current.opponent {
                    White => {
                        self.settings.white_undo -= 1;
                    }
                    Black => {
                        self.settings.black_undo -= 1;
                    }
                }
            }
        }
    }
}
