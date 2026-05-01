use crate::ChessApp;
use crate::board::cell::Color::*;
use crate::gui::chessapp::AppMode;
use crate::gui::hooks::windows::WinDia::*;
use egui::Label;
use egui::Sense;

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
                        self.turn_infos(ui);

                        ui.separator();
                        self.player_bar(ui, &Black);
                        ui.add_space(30.0);

                        self.mobile_board_display(ui, board_size, ctx);

                        ui.add_space(30.0);
                        self.player_bar(ui, &White);
                        ui.add_space(20.0);

                        ui.separator();

                        let color = self.game.active_player;
                        self.engine_infos(ui, &color);
                        self.display_history(ui);

                        ui.add_space(70.0);
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
                self.draw_resign_undo_mobile(ui);
            }
        });
    }

    pub fn mobile_board_display(
        &mut self,
        ui: &mut egui::Ui,
        board_size: f32,
        ctx: &egui::Context,
    ) {
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

        self.left_click(inner, sq, &response, ctx);
        self.right_click(&response);
        self.drag_and_drop(inner, sq, &response);
    }

    pub fn display_history(&mut self, ui: &mut egui::Ui) {
        ui.add_space(40.0);
        ui.horizontal(|ui| {
            let displayed_text: String = self
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
                self.win = Some(Pgn);
            }
        });
    }
}
