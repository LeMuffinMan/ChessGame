use crate::ChessApp;
use crate::gui::chessapp_struct::End;
use crate::gui::chessapp_struct::AppMode::*;
use crate::gui::chessapp_struct::WinDia::*;
use egui::RichText;

impl ChessApp {

    pub fn hook_win_diag(&mut self, ctx: &egui::Context) {
        if let Some(win) = &self.mobile_win {
            match win {
                Options => {
                    egui::Window::new("Options")
                    .collapsible(false)
                    .resizable(false)
                        .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
                    .show(ctx, |ui| {
                        let style = ui.style_mut();
                        style.spacing.icon_width = 40.0; // largeur de la checkbox
                        style.spacing.icon_spacing = 8.0; // espace entre checkbox et texte

                        ui.add_space(20.0);
                        // Checkboxes avec texte réduit
                        ui.checkbox(&mut self.widgets.show_coordinates, RichText::new("Coordinates").size(30.0));
                        ui.checkbox(&mut self.widgets.show_legals_moves, RichText::new("Legals moves").size(30.0));
                        ui.checkbox(&mut self.widgets.show_threaten_cells, RichText::new("Threaten cells").size(30.0));
                        ui.checkbox(&mut self.widgets.show_last_move, RichText::new("Last move").size(30.0));

                        ui.add_space(20.0);
                        ui.vertical_centered(|ui| {
                            if ui.button("Save options").clicked() {
                                self.mobile_win = None;
                            }
                        });
                    });
                },
                Resign => {
                    egui::Window::new("Resignation ?")
                        .collapsible(false)
                        .resizable(false)
                        .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
                        .show(ctx, |ui| {
                            self.win_dialog = true;
                            ui.add_space(40.0);
                            ui.horizontal(|ui| {
                                ui.add_space(40.0);
                                if ui.button("Accept").clicked() {
                                    self.current.end = Some(End::Resign);
                                    self.mobile_win = None;
                                    self.app_mode = Lobby;
                                }
                                ui.add_space(120.0);
                                if ui.button("Decline").clicked() {
                                    self.mobile_win = None;
                                }
                                ui.add_space(40.0);
                            });
                            ui.add_space(40.0);
                    });
                },
                Draw => {
                    self.offer_draw(ctx);
                },
                Promote => {
                    self.get_promotion_input(ctx);
                },
                Timer => {
                    self.set_timer(ctx);
                }
            }
        }
    }

}
