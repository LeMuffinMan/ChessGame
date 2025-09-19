use crate::ChessApp;
use crate::Color::*;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::AppMode::*;
use crate::gui::chessapp_struct::End;
use crate::gui::chessapp_struct::WinDia::*;
use crate::gui::mobile_ui::hooks::End::TimeOut;
use egui::RichText;

impl ChessApp {
    pub fn mobile_replay_step(&mut self, ctx: &egui::Context) {
        if let Some(next_step) = self.replay_infos.next_step {
            let now = ctx.input(|i| i.time);
            if now >= next_step {
                if self.replay_infos.index + 1 < self.history.len() {
                    self.replay_infos.index += 1;
                    self.current = self.history[self.replay_infos.index].clone();
                    let delay = self.replay_infos.sec_per_frame;
                    self.replay_infos.next_step = Some(now + delay);
                } else {
                    self.replay_infos.index = self.history.len();
                    self.replay_infos.next_step = None;
                }
            }
        }
        ctx.request_repaint();
    }

    pub fn mobile_update_timer(&mut self, ctx: &egui::Context) {
        let now = ctx.input(|i| i.time);

        // Initialisation si jamais None
        if self.mobile_timer.start_of_turn.1.is_none() {
            self.mobile_timer.start_of_turn.1 = Some(self.current.active_player);
            self.mobile_timer.start_of_turn.0 = now;
        }

        // Si changement de joueur
        if self.mobile_timer.start_of_turn.1 != Some(self.current.active_player) {
            // Ajout de l'incrément
            match self.mobile_timer.start_of_turn.1 {
                Some(White) => self.mobile_timer.white_time += self.mobile_timer.increment,
                Some(Black) => self.mobile_timer.black_time += self.mobile_timer.increment,
                None => {}
            }
            // Switch
            self.mobile_timer.start_of_turn.1 = Some(self.current.active_player);
            self.mobile_timer.start_of_turn.0 = now;
        }

        // Calcul du delta depuis la dernière update
        let delta = now - self.mobile_timer.start_of_turn.0;
        self.mobile_timer.start_of_turn.0 = now; // reset pour le prochain tick

        // Décrémentation du joueur actif
        match self.current.active_player {
            White => {
                self.mobile_timer.white_time -= delta;
                if self.mobile_timer.white_time <= 0.0 {
                    self.mobile_timer.white_time = 0.0;
                    self.current.end = Some(TimeOut);
                }
            }
            Black => {
                self.mobile_timer.black_time -= delta;
                if self.mobile_timer.black_time <= 0.0 {
                    self.mobile_timer.black_time = 0.0;
                    self.current.end = Some(TimeOut);
                }
            }
        }
    }

    pub fn hook_win_diag(&mut self, ctx: &egui::Context) {
        if let Some(win) = &self.mobile_win {
            match win {
                Options => {
                    self.settings_win(ctx);
                }
                Resign => {
                    self.resign_win(ctx);
                }
                Draw => {
                    self.offer_draw_win(ctx);
                }
                Promote => {
                    self.promote_win(ctx);
                }
                Timer => {
                    self.set_timer(ctx);
                }
                Undo => {
                    self.ask_undo(ctx);
                }
                Pgn => {
                    self.pgn_win(ctx);
                }
            }
        }
    }

    pub fn pgn_win(&mut self, ctx: &egui::Context) {
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
                    // ui.add_space(60.0);
                    ui.vertical_centered(|ui| {
                        ui.label(&self.history_san);
                        ui.add_space(20.0);
                        ui.text_edit_singleline(&mut self.widgets.file_name);
                        if ui.button(RichText::new("Download").size(30.0)).clicked() {
                            let _ = self.export_pgn(); //Todo : handle error 
                            self.mobile_win = None;
                        }
                        ui.add_space(40.0);
                    });
                });
                ui.vertical_centered(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.mobile_win = None;
                    }
                });
                //ajouter le nombre de undo max par joueur
                ui.add_space(20.0);
            });
    }

    pub fn settings_win(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
            .show(ctx, |ui| {
                let style = ui.style_mut();
                style.spacing.icon_width = 40.0; // largeur de la checkbox
                style.spacing.icon_spacing = 8.0; // espace entre checkbox et texte

                ui.add_space(20.0);
                self.highlight_checkboxes(ui);
                ui.add_space(20.0);
                // if !self.history.is_empty() {
                //     self.mobile_save_button(ui);
                // }
                ui.vertical_centered(|ui| {
                    if ui.button("Save options").clicked() {
                        self.mobile_win = None;
                    }
                });
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.hyperlink_to("Source code", "https://github.com/LeMuffinMan/ChessGame");
                });
                //ajouter le nombre de undo max par joueur
                ui.add_space(20.0);
            });
    }

    pub fn highlight_checkboxes(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(50.0);
            ui.vertical(|ui| {
                ui.checkbox(
                    &mut self.widgets.show_coordinates,
                    RichText::new("Coordinates").size(30.0),
                );
                ui.add_space(20.0);
                ui.checkbox(
                    &mut self.widgets.show_legals_moves,
                    RichText::new("Legals moves").size(30.0),
                );
                ui.add_space(20.0);
                ui.checkbox(
                    &mut self.widgets.show_threaten_cells,
                    RichText::new("Threaten cells").size(30.0),
                );
                ui.add_space(20.0);
                ui.checkbox(
                    &mut self.widgets.show_last_move,
                    RichText::new("Last move").size(30.0),
                );
            });
        });
    }

    pub fn resign_win(&mut self, ctx: &egui::Context) {
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
                        self.app_mode = Versus(Some(End::Resign));
                    }
                    ui.add_space(120.0);
                    if ui.button("Decline").clicked() {
                        self.mobile_win = None;
                    }
                    ui.add_space(40.0);
                });
                ui.add_space(40.0);
            });
    }

    pub fn offer_draw_win(&mut self, ctx: &egui::Context) {
        egui::Window::new(
            RichText::new(format!("{:?} offer a Draw", self.current.opponent)).size(50.0), // taille plus petite
        )
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
        .show(ctx, |ui| {
            self.win_dialog = true;
            ui.add_space(40.0);
            ui.horizontal(|ui| {
                ui.add_space(40.0);
                if ui.button("Accept").clicked() {
                    self.current.end = Some(End::Draw);
                    self.mobile_win = None;
                    self.app_mode = Versus(Some(End::Draw));
                }
                ui.add_space(120.0);
                if ui.button("Decline").clicked() {
                    self.mobile_win = None;
                }
                ui.add_space(40.0);
            });
            ui.add_space(40.0);
        });
    }

    pub fn promote_win(&mut self, ctx: &egui::Context) {
        egui::Window::new("Pawn to promote")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
            .show(ctx, |ui| {
                self.win_dialog = true;
                ui.add_space(40.0);
                ui.horizontal(|ui| {
                    ui.add_space(40.0);
                    ui.vertical(|ui| {
                        ui.radio_value(&mut self.current.board.promote, Some(Queen), "Queen");
                        ui.radio_value(&mut self.current.board.promote, Some(Bishop), "Bishop");
                        ui.radio_value(&mut self.current.board.promote, Some(Knight), "Knight");
                        ui.radio_value(&mut self.current.board.promote, Some(Rook), "Rook");
                    });
                    if ui.button("Promote").clicked() {
                        self.current.end = Some(End::Resign);
                        self.mobile_win = None;
                        self.app_mode = Versus(Some(End::Resign));
                    }
                    ui.add_space(120.0);
                    if ui.button("Decline").clicked() {
                        self.mobile_win = None;
                    }
                    ui.add_space(40.0);
                });
                ui.add_space(40.0);
            });
    }
}
