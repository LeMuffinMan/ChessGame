use crate::ChessApp;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::End::*;
use egui::TextEdit;

//i use window dialog as popup to ask player for input or confirmation

impl ChessApp {
    pub fn draw_request(&mut self, ui: &mut egui::Ui) {
        ui.label("Accept draw offer ?");
        ui.horizontal(|ui| {
            if ui.button("Accept").clicked() {
                self.current.end = Some(Draw);
                self.draw.draw_option = None;
            }
            if ui.button("Reject").clicked() {
                self.draw.draw_option = None;
            }
        });
    }

    pub fn save_game(&mut self, ctx: &egui::Context) {
        egui::Window::new("Save game")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.add(TextEdit::singleline(&mut self.widgets.file_name));
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    if ui.button("Download").clicked() {
                        let _ = self.export_pgn(); //Todo : handle error 
                        self.win_save = false;
                    }
                    ui.add_space(130.0);
                    if ui.button("Cancel").clicked() {
                        self.win_save = false;
                    }
                    ui.add_space(20.0);
                });
                ui.add_space(10.0);
            });
    }

    pub fn offer_draw(&mut self, ctx: &egui::Context) {
        egui::Window::new(format!("{:?} offers a draw", self.current.active_player))
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    if ui.button("Accept").clicked() {
                        self.current.end = Some(Draw);
                        self.draw.draw_option = None;
    //window dialog
                    }
                    ui.add_space(30.0);
                    if ui.button("Decline").clicked() {
                        self.draw.draw_option = None;
                    }
                    ui.add_space(20.0);
                });
                ui.add_space(10.0);
            });
    }

    pub fn resign_confirm(&mut self, ctx: &egui::Context) {
        egui::Window::new("Resignation ?")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                self.win_dialog = true;
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    if ui.button("Accept").clicked() {
                        self.current.end = Some(Resign);
                        self.win_resign = false;
                        self.win_dialog = false;
                    }
                    ui.add_space(30.0);
                    if ui.button("Decline").clicked() {
                        self.win_resign = false;
                        self.win_dialog = false;
                    }
                    ui.add_space(20.0);
                });
                ui.add_space(10.0);
            });
    }

    pub fn ask_to_undo(&mut self, ctx: &egui::Context) {
        egui::Window::new(format!("{:?} ask to undo", self.current.opponent))
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                self.win_dialog = true;
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    if ui.button("Accept").clicked() {
                        match self.history.len() {
                            1 | 2 => {
                                *self = ChessApp::default();
                            }
                            _ => {
                                if let Some(last) = self.history.pop() {
                                    #[warn(clippy::collapsible_if)]
                                    // to fix to handle properly undo
                                    if last == self.current {
                                        if let Some(before_last) = self.history.pop() {
                                            self.current = before_last;
                                        } else {
                                            self.current = last;
                                        }
                                        self.widgets.replay_index = self.history.len();
                                    }
                                }
                                //effacer historique
                                //ne pas toucher aux timers
                            }
                        }
                        self.win_dialog = false;
                        self.win_undo = false;
                    }
                    ui.add_space(30.0);
                    if ui.button("Decline").clicked() {
                        self.win_dialog = false;
                        self.win_undo = false;
                    }
                    ui.add_space(20.0);
                });
                ui.add_space(10.0);
            });
    }

    pub fn get_promotion_input(&mut self, ctx: &egui::Context) {
        self.win_dialog = true;
        egui::Window::new("Promotion")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.radio_value(&mut self.current.board.promote, Some(Queen), "Queen");
                    ui.radio_value(&mut self.current.board.promote, Some(Bishop), "Bishop");
                    ui.radio_value(&mut self.current.board.promote, Some(Knight), "Knight");
                    ui.radio_value(&mut self.current.board.promote, Some(Rook), "Rook");
                });
            });
        self.update_promote();
    }
}
