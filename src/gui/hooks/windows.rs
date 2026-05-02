use crate::ChessApp;
use crate::board::cell::Color::{Black, White};
use crate::engine::bot::PlayerType::*;
use crate::game::End::{self, Draw};
use crate::gui::chessapp::AppMode::*;
use crate::gui::layout::UiType::*;
use egui::RichText;

//Hooks ?
pub enum WinDia {
    Settings,
    Promote,
    DrawRequest,
    Resign,
    Timer,
    Undo,
    Pgn,
}

impl ChessApp {
    pub fn hook_win(&mut self, ctx: &egui::Context) {
        if let Some(win) = &self.win {
            match win {
                WinDia::Settings => self.settings_win(ctx),
                WinDia::Resign => self.resign_win(ctx),
                WinDia::DrawRequest => self.offer_draw_win(ctx),
                WinDia::Promote => {} // self.promote_win(ctx),
                WinDia::Timer => self.timer_window(ctx),
                WinDia::Undo => self.ask_undo(ctx),
                WinDia::Pgn => self.pgn_win(ctx),
            }
        }
    }
    pub fn settings_win(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
            .show(ctx, |ui| {
                let style = ui.style_mut();
                style.spacing.icon_width = 40.0;
                style.spacing.icon_spacing = 8.0;

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    ui.add_space(50.0);
                    ui.vertical(|ui| {
                        self.highlight_checkboxes(ui);
                    });
                });
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if self.settings.allow_undo {
                        ui.add_space(20.0);
                    } else {
                        ui.add_space(40.0);
                    }
                    self.undo_limit_hint(ui);
                });
                ui.add_space(20.0);
                ui.vertical_centered(|ui| {
                    self.flip_buttons(ui);
                    ui.add_space(40.0);
                    if ui.button("Save settings").clicked() {
                        self.win = None;
                    }
                });
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.hyperlink_to("Source code", "https://github.com/LeMuffinMan/ChessGame");
                });
                ui.add_space(20.0);
            });
    }

    pub fn resign_win(&mut self, ctx: &egui::Context) {
        match self.ui_type {
            Mobile => {
                egui::Window::new("Resignation ?")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
                    .show(ctx, |ui| {
                        ui.add_space(40.0);
                        ui.horizontal(|ui| {
                            ui.add_space(40.0);
                            if ui.button("Accept").clicked() {
                                self.game.end = Some(End::Resign);
                                self.win = None;
                                self.timer.active = false;
                                self.app_mode = Versus(Some(End::Resign));
                            }
                            ui.add_space(120.0);
                            if ui.button("Decline").clicked() {
                                self.win = None;
                            }
                            ui.add_space(40.0);
                        });
                        ui.add_space(40.0);
                    });
            }
            Desktop => {
                egui::Window::new("Resignation ?")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        ui.add_space(30.0);
                        ui.horizontal(|ui| {
                            ui.add_space(20.0);
                            if ui.button(RichText::new("Accept").size(40.0)).clicked() {
                                self.game.end = Some(End::Resign);
                                self.timer.active = false;
                                self.win = None;
                            }
                            ui.add_space(60.0);
                            if ui.button(RichText::new("Decline").size(40.0)).clicked() {
                                self.win = None;
                            }
                            ui.add_space(20.0);
                        });
                        ui.add_space(30.0);
                    });
            }
        }
    }

    pub fn offer_draw_win(&mut self, ctx: &egui::Context) {
        match &self.ui_type {
            Mobile => {
                egui::Window::new(RichText::new("Draw offer").size(50.0))
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
                    .show(ctx, |ui| {
                        ui.add_space(40.0);
                        ui.horizontal(|ui| {
                            ui.add_space(40.0);
                            if ui.button("Accept").clicked() {
                                self.game.end = Some(End::Draw);
                                self.timer.active = false;
                                self.win = None;
                                self.app_mode = Versus(Some(End::Draw));
                            }
                            ui.add_space(120.0);
                            if ui.button("Decline").clicked() {
                                self.win = None;
                            }
                            ui.add_space(40.0);
                        });
                        ui.add_space(40.0);
                    });
            }
            Desktop => {
                egui::Window::new(RichText::new("Draw offer").size(50.0))
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        ui.add_space(40.0);
                        ui.horizontal(|ui| {
                            ui.add_space(40.0);
                            if ui.button(RichText::new("Accept").size(40.0)).clicked() {
                                self.game.end = Some(Draw);
                                self.timer.active = false;
                                self.win = None;
                                //window dialog
                            }
                            ui.add_space(100.0);
                            if ui.button(RichText::new("Decline").size(40.0)).clicked() {
                                self.win = None;
                            }
                            ui.add_space(40.0);
                        });
                        ui.add_space(40.0);
                    });
            }
        }
    }

    pub fn pgn_win(&mut self, ctx: &egui::Context) {
        match self.ui_type {
            Mobile => {
                egui::Window::new("PGN")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
                    .show(ctx, |ui| {
                        let style = ui.style_mut();
                        style.spacing.icon_width = 40.0; // largeur de la checkbox
                        style.spacing.icon_spacing = 8.0; // espace entre checkbox et texte

                        ui.add_space(20.0);
                        ui.horizontal(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(&self.history_san);
                                ui.add_space(20.0);
                                ui.text_edit_singleline(&mut self.settings.file_name);
                                if ui.button(RichText::new("Download").size(30.0)).clicked() {
                                    #[cfg(target_arch = "wasm32")]
                                    let _ = self.export_pgn();
                                    self.win = None;
                                }
                                ui.add_space(40.0);
                            });
                        });
                        ui.vertical_centered(|ui| {
                            if ui.button("Cancel").clicked() {
                                self.win = None;
                            }
                        });
                        ui.add_space(20.0);
                    });
            }
            Desktop => {
                egui::Window::new("PGN")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        let style = ui.style_mut();
                        style.spacing.icon_width = 40.0;
                        style.spacing.icon_spacing = 8.0;

                        ui.add_space(20.0);
                        ui.horizontal(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(&self.history_san);
                                ui.add_space(20.0);
                                ui.text_edit_singleline(&mut self.settings.file_name);
                                ui.add_space(20.0);
                                if ui.button(RichText::new("Download").size(30.0)).clicked() {
                                    #[cfg(target_arch = "wasm32")]
                                    let _ = self.export_pgn(); //Todo : handle error
                                    self.win = None;
                                }
                                ui.add_space(20.0);
                            });
                        });
                        ui.vertical_centered(|ui| {
                            if ui.button("Cancel").clicked() {
                                self.win = None;
                            }
                        });
                        ui.add_space(20.0);
                    });
            }
        }
    }
    pub fn ask_undo(&mut self, ctx: &egui::Context) {
        egui::Window::new("Accept undo ?")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
            .show(ctx, |ui| {
                ui.add_space(40.0);
                ui.horizontal(|ui| {
                    ui.add_space(100.0);
                    if ui.button("Accept").clicked() {
                        self.win = None;
                        self.game.history.pop();
                        if self.promoteinfo.is_some() {
                            self.game.history.pop();
                        }
                        if self.settings.white_bot != Human || self.settings.black_bot != Human {
                            self.game.history.pop();
                            if self.promoteinfo.is_some() {
                                self.game.history.pop();
                            }
                        }
                        self.replay_infos.index = self.game.history.len();
                        self.game.board = self.game.board_at(self.replay_infos.index);
                        self.game.active_player = if self.replay_infos.index.is_multiple_of(2) {
                            White
                        } else {
                            Black
                        };
                        self.update_threaten_cells();
                        self.update_legals_moves();
                    }
                    ui.add_space(30.0);
                    if ui.button("Decline").clicked() {
                        self.win = None;
                    }
                    ui.add_space(20.0);
                });
                ui.add_space(40.0);
            });
    }
}
