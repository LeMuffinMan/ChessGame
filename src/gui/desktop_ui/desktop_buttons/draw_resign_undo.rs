use crate::Color::*;
use crate::gui::chessapp_struct::End::*;
use crate::gui::game_state_struct::DrawOption::*;
use crate::gui::game_state_struct::DrawRule;
use crate::gui::hooks::WinDia;
use crate::gui::hooks::WinDia::*;
// use crate::gui::desktop_ui::desktop_buttons::draw_resign_undo::WinDia::DrawRequest;

use crate::ChessApp;

impl ChessApp {
    pub fn draw_resign_undo(&mut self, ui: &mut egui::Ui) {
        //shows the rule triggering the draw
        if let Some(draw) = &self.current.draw.draw_option {
            ui.separator();
            match draw {
                Available(DrawRule::TripleRepetition) => {
                    ui.label("Triple repetition :");
                }
                Available(DrawRule::FiftyMoves) => {
                    ui.label("50 moves : ");
                }
                //ajouter les situations impossibles
                _ => {}
            };
            //catch user inputs to ask for resign or draw to opponent using window_dialog
            if ui.button("Claim draw").clicked() {
                self.current.end = Some(Draw);
                self.current.draw.draw_option = None;
            }
        } else if ui
            .add_enabled(
                self.current.end.is_none() && self.win.is_none(),
                egui::Button::new("Draw"),
            )
            .clicked()
        {
            self.win = Some(DrawRequest);
        }
        ui.separator();
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    self.current.end.is_none() && self.win.is_none(),
                    egui::Button::new("Resign"),
                )
                .clicked()
            {
                self.win = Some(WinDia::Resign);
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
        });
    }
}
