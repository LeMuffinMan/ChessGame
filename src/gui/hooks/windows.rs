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
                    ui.hyperlink_to(
                        "Benchmark",
                        "https://lemuffinman.github.io/ChessGame/bench.html",
                    );
                    ui.hyperlink_to("Source code", "https://github.com/LeMuffinMan/ChessGame");
                    ui.hyperlink_to("Lichess", "https://lichess.org/@/LeMuffinBot");
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
                        style.spacing.icon_width = 40.0;
                        style.spacing.icon_spacing = 8.0;

                        ui.add_space(20.0);
                        if !self.game.history.is_empty() {
                            ui.horizontal(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(&self.history_san);
                                    ui.add_space(20.0);
                                    ui.text_edit_singleline(&mut self.settings.file_name);
                                    if ui.button(RichText::new("Download").size(30.0)).clicked() {
                                        self.export_pgn_any();
                                        self.win = None;
                                    }
                                    ui.add_space(20.0);
                                });
                            });
                            ui.separator();
                        }
                        self.pgn_import_section(ui);
                        ui.vertical_centered(|ui| {
                            if ui.button("Cancel").clicked() {
                                self.win = None;
                                self.settings.pgn_import_error = None;
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
                        if !self.game.history.is_empty() {
                            ui.horizontal(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(&self.history_san);
                                    ui.add_space(20.0);
                                    ui.text_edit_singleline(&mut self.settings.file_name);
                                    ui.add_space(20.0);
                                    if ui.button(RichText::new("Download").size(30.0)).clicked() {
                                        self.export_pgn_any();
                                        self.win = None;
                                    }
                                    ui.add_space(20.0);
                                });
                            });
                            ui.separator();
                        }
                        self.pgn_import_section(ui);
                        ui.vertical_centered(|ui| {
                            if ui.button("Cancel").clicked() {
                                self.win = None;
                                self.settings.pgn_import_error = None;
                            }
                        });
                        ui.add_space(20.0);
                    });
            }
        }
    }

    fn export_pgn_any(&mut self) {
        #[cfg(target_arch = "wasm32")]
        let _ = self.export_pgn();
        #[cfg(not(target_arch = "wasm32"))]
        let _ = self.export_pgn_native();
    }

    fn pgn_import_section(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label("Import a game (PGN)");
            ui.add_space(8.0);
            ui.add(
                egui::TextEdit::multiline(&mut self.settings.pgn_import_text)
                    .hint_text("Paste PGN text here")
                    .desired_rows(4)
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(8.0);
            if ui.button("Import").clicked() {
                match self.import_pgn(&self.settings.pgn_import_text.clone()) {
                    Ok(()) => self.win = None,
                    Err(e) => self.settings.pgn_import_error = Some(e),
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                ui.add_space(12.0);
                ui.text_edit_singleline(&mut self.settings.file_path_input);
                if ui.button("Load from file").clicked() {
                    match self.import_pgn_from_path(&self.settings.file_path_input.clone()) {
                        Ok(()) => self.win = None,
                        Err(e) => self.settings.pgn_import_error = Some(e),
                    }
                }
            }

            if let Some(err) = &self.settings.pgn_import_error {
                ui.add_space(8.0);
                ui.colored_label(egui::Color32::from_rgb(220, 80, 80), err);
            }
        });
        ui.add_space(20.0);
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
                        self.settings.from_cell = None;
                        self.settings.piece_legals_moves.clear();
                        self.last_move = self.game.history.last().map(|m| (m.origin, m.dest));
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
