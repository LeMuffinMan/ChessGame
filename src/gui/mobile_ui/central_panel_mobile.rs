use crate::ChessApp;
use crate::gui::chessapp_struct::AppMode;
use crate::gui::desktop_ui::bot_panels::format_time;
use crate::gui::hooks::WinDia::*;
use crate::gui::update_timer::GameMode::*;

use egui::Label;
use egui::RichText;
use egui::Sense;

//a remplacer

impl ChessApp {
    pub fn central_panel_mobile(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_width = ui.available_width();

            let bar_height = 40.0;
            let board_size = available_width.min(ui.available_height() - bar_height * 2.0);

            let total_height = bar_height * 2.0 + board_size;

            ui.vertical(|ui| {
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(available_width, total_height),
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        ui.add_space(20.0);
                        self.black_panel(ui);
                        ui.add_space(20.0);

                        self.mobile_board_display(ui, board_size);

                        ui.add_space(20.0);

                        self.white_panel(ui);

                        ui.add_space(40.0);
                        ui.separator();
                        ui.add_space(40.0);

                        self.mobile_turn_infos(ui);

                        ui.add_space(40.0);
                        ui.separator();

                        self.display_history(ui);

                        if matches!(self.app_mode, AppMode::Versus(_)) {
                            ui.add_space(20.0);
                        }
                        ui.add_space(100.0);
                        self.speed_replay_slider(ui);
                        self.display_bottom_buttons(ui);
                    },
                );
            });
        });
    }

    pub fn display_bottom_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| match &self.app_mode {
            AppMode::Replay => {
                self.replay_buttons(ui);
            }
            AppMode::Lobby => {
                self.lobby_buttons(ui);
            }
            AppMode::Versus(Some(_end)) => {
                self.draw_endgame_buttons(ui);
            }
            AppMode::Versus(None) => {
                self.draw_resign_undo_buttons(ui);
            }
        });
    }

    pub fn mobile_board_display(&mut self, ui: &mut egui::Ui, board_size: f32) {
        // plateau
        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(board_size, board_size),
            egui::Sense::click_and_drag(),
        );

        let rect = response.rect;
        let inner = if self.settings.show_coordinates {
            ChessApp::render_border(&painter, rect);
            rect.shrink(16.0)
        } else {
            rect
        };

        let sq = inner.width() / 8.0;

        if self.settings.show_coordinates {
            self.display_coordinates(&painter, inner, sq);
        }
        self.render_board(&painter, inner, sq);
        self.render_pieces(&painter, inner, sq);
        self.render_dragged_piece(&painter, inner);

        self.left_click(inner, sq, &response);
        self.right_click(&response);
        self.drag_and_drop(inner, sq, &response);
    }

    pub fn display_history(&mut self, ui: &mut egui::Ui) {
        ui.add_space(40.0);
        ui.horizontal(|ui| {
            let displayed_text: String = self
                .history
                .history_san
                .chars()
                .rev()
                .take(66)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();

            let response = ui.add(Label::new(displayed_text).sense(Sense::click()));

            if response.clicked() {
                self.win = Some(ExportPgn);
            }
        });
    }

    //impl pour timer
    pub fn black_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Black").size(50.0));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if self.timer.mode != NoTime {
                    if self.timer.increment == 0.0 {
                        ui.label(
                            RichText::new(format_time(self.timer.black_time) + " ⏱").size(50.0),
                        );
                    } else {
                        ui.label(
                            RichText::new(
                                format_time(self.timer.black_time)
                                    + " ⏱ + "
                                    + &format_time(self.timer.increment).to_string(),
                            )
                            .size(50.0),
                        );
                    }
                }
            });
        });
    }

    pub fn white_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("White").size(50.0));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if self.timer.mode != NoTime {
                    if self.timer.increment == 0.0 {
                        ui.label(
                            RichText::new(format_time(self.timer.white_time) + " ⏱").size(50.0),
                        );
                    } else {
                        ui.label(
                            RichText::new(
                                format_time(self.timer.white_time)
                                    + " ⏱ + "
                                    + &format_time(self.timer.increment).to_string(),
                            )
                            .size(50.0),
                        );
                    }
                }
            });
        });
    }
}
