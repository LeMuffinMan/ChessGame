use crate::ChessApp;
use crate::Color::*;
use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::AppMode::*;
use crate::gui::chessapp_struct::End;
use crate::gui::chessapp_struct::UiType::*;
use crate::gui::hooks::End::Draw;
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

    pub fn hook_win(&mut self, ctx: &egui::Context) {
        if let Some(win) = &self.mobile_win {
            match win {
                WinDia::Settings => {
                    self.settings_win(ctx);
                }
                WinDia::Resign => {
                    self.resign_win(ctx);
                }
                WinDia::DrawRequest => {
                    self.offer_draw_win(ctx);
                }
                WinDia::Promote => {
                    // self.promote_win(ctx);
                }
                WinDia::Timer => {
                    self.set_timer(ctx);
                }
                WinDia::Undo => {
                    self.ask_undo(ctx);
                }
                WinDia::Pgn => {
                    self.pgn_win(ctx);
                }
            }
        }
    }

    pub fn get_promotion_input(&mut self, ctx: &egui::Context) {
        egui::Window::new("Promotion")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(140.0);
                    ui.vertical(|ui| {
                        ui.add_space(20.0);
                        ui.selectable_value(&mut self.current.board.promote, Some(Queen), "Queen");
                        ui.add_space(20.0);
                        ui.selectable_value(
                            &mut self.current.board.promote,
                            Some(Bishop),
                            "Bishop",
                        );
                        ui.add_space(20.0);
                        ui.selectable_value(
                            &mut self.current.board.promote,
                            Some(Knight),
                            "Knight",
                        );
                        ui.add_space(20.0);
                        ui.selectable_value(&mut self.current.board.promote, Some(Rook), "Rook");
                    });
                });
                ui.add_space(20.0);
            });
        self.update_promote();
    }

    pub fn ask_undo(&mut self, ctx: &egui::Context) {
        egui::Window::new("Accept undo ?") //opponent
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
            .show(ctx, |ui| {
                ui.add_space(40.0);
                ui.horizontal(|ui| {
                    ui.add_space(100.0);
                    if ui.button("Accept").clicked() {
                        self.mobile_win = None;
                        self.current = self.history[self.replay_infos.index - 2].clone();
                        if self.replay_infos.index == 2 {
                            self.replay_infos.index -= 2;
                            self.history.clear();
                        } else {
                            self.replay_infos.index -= 1;
                            self.history.pop();
                        }
                        //une promote ajoute 2 indexs a l'historique ! a fix
                        if self.current.board.pawn_to_promote.is_some() {
                            self.replay_infos.index -= 1;
                            self.history.pop();
                        }
                        //il faut supprimer les derniers hashs pour la triple repetition
                    }
                    ui.add_space(30.0);
                    if ui.button("Decline").clicked() {
                        self.mobile_win = None;
                    }
                    ui.add_space(20.0);
                });
                ui.add_space(40.0);
            });
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
            Desktop => {
                egui::Window::new("PGN")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
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
                                ui.add_space(20.0);
                                if ui.button(RichText::new("Download").size(30.0)).clicked() {
                                    let _ = self.export_pgn(); //Todo : handle error 
                                    self.mobile_win = None;
                                }
                                ui.add_space(20.0);
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
                self.highlight_checkboxes(ui);
                ui.add_space(20.0);
                // if !self.history.is_empty() {
                //     self.mobile_save_button(ui);
                // }
                ui.vertical_centered(|ui| {
                    self.side_panel_flip(ui);
                    ui.add_space(20.0);
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
                                self.current.end = Some(End::Resign);
                                self.mobile_win = None;
                                self.mobile_timer.active = false;
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
                                self.current.end = Some(End::Resign);
                                self.mobile_timer.active = false;
                                self.mobile_win = None;
                            }
                            ui.add_space(60.0);
                            if ui.button(RichText::new("Decline").size(40.0)).clicked() {
                                self.mobile_win = None;
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
                                self.current.end = Some(End::Draw);
                                self.mobile_timer.active = false;
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
                                self.current.end = Some(Draw);
                                self.mobile_timer.active = false;
                                self.mobile_win = None;
                                //window dialog
                            }
                            ui.add_space(100.0);
                            if ui.button(RichText::new("Decline").size(40.0)).clicked() {
                                self.mobile_win = None;
                            }
                            ui.add_space(40.0);
                        });
                        ui.add_space(40.0);
                    });
            }
        }
    }

    //Desktop hook
    //if a player promoted a pawn, try_move didnt finished it's work, so we do it here
    pub fn update_promote(&mut self) {
        if let Some(piece) = self.current.board.promote
            && let Some(coord) = self.current.board.pawn_to_promote
            && self.replay_infos.index == self.history.len()
        {
            //methods get opponent color
            let color = if self.current.active_player == White {
                Black
            } else {
                White
            };
            self.current.board.grid[coord.row as usize][coord.col as usize] =
                Cell::Occupied(piece, color);

            //methods
            let opponent = if self.current.active_player != White {
                White
            } else {
                Black
            };
            if let Some(k) = self.current.board.get_king(&opponent)
                && self.current.board.threaten_cells.contains(&k)
                && let Some(k) = self.current.board.get_king(&opponent)
            {
                self.current.board.check = Some(k);
                // println!("Check !");
            }
            self.check_endgame();
            if let Some(promoteinfo) = &self.promoteinfo {
                let from = promoteinfo.from;
                let to = promoteinfo.to;
                let prev_board = promoteinfo.prev_board.clone();
                self.history.push(self.current.clone());
                self.replay_infos.index += 1;
                // self.replay_infos.index += 1;
                self.encode_move_to_san(&from, &to, &prev_board);
            }
            self.current.board.pawn_to_promote = None;
            self.current.board.promote = None;
            self.mobile_win = None;
        }
    }
}
