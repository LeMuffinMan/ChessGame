use crate::gui::chessapp_struct::WinDia::*;
use crate::gui::chessapp_struct::AppMode::*;
use crate::gui::chessapp_struct::AppMode;
use crate::gui::chessapp_struct::GameState;
use crate::gui::chessapp_struct::MobileGameMode::*;
use crate::gui::desktop_ui::top_bot_panels::bot_panels::format_time;
use crate::ChessApp;
use crate::Board;
use crate::Color;

use egui::TextStyle;
use egui::FontId;


//a remplacer 
use crate::gui::desktop_ui::central_panel::central_panel_ui::render_border;

impl ChessApp {
    pub fn ui_mobile(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(70.0, egui::FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(50.0, egui::FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(22.0, egui::FontFamily::Monospace)),
            (TextStyle::Button, FontId::new(40.0, egui::FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(18.0, egui::FontFamily::Proportional)),
        ]
        .into();
        ctx.set_style(style);

        self.top_title_panel(ctx);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_width = ui.available_width();

            // Hauteurs des barres et du plateau
            let bar_height = 40.0;
            let board_size = available_width.min(ui.available_height() - bar_height * 2.0);

            // Hauteur totale du bloc (barres + plateau)
            let total_height = bar_height * 2.0 + board_size;

            // Coin supérieur gauche du bloc
            ui.vertical(|ui| {
                ui.allocate_ui_with_layout(
                egui::Vec2::new(available_width, total_height),
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {


                    ui.add_space(20.0);
                    // barre joueur noir
                    ui.horizontal(|ui| {
                        ui.label("Black");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if self.mobile_timer.mode != NoTime {
                                ui.label(format_time(self.mobile_timer.start) + " ⏱ + " + &format_time(self.mobile_timer.increment).to_string());
                            }
                        });
                    });

                    ui.add_space(20.0);

                    // plateau
                    let (response, painter) = ui.allocate_painter(
                        egui::Vec2::new(board_size, board_size),
                        egui::Sense::click_and_drag(),
                    );

                    let rect = response.rect;
                    let inner = if self.widgets.show_coordinates {
                        render_border(&painter, rect);
                        rect.shrink(16.0)
                    } else {
                        rect
                    };

                    let sq = inner.width() / 8.0;

                    if self.widgets.show_coordinates {
                        self.display_coordinates(&painter, inner, sq);
                    }
                    self.render_board(&painter, inner, sq);
                    self.render_pieces(&painter, inner, sq);
                    self.render_dragged_piece(&painter, inner);

                    self.left_click(inner, sq, &response);
                    self.right_click(&response);
                    self.drag_and_drop(inner, sq, &response);

                    ui.add_space(20.0);
                    // barre joueur blanc
                    ui.horizontal(|ui| {
                        ui.label("White");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if self.mobile_timer.mode != NoTime {
                                ui.label(format_time(self.mobile_timer.start) + " ⏱ + " + &format_time(self.mobile_timer.increment).to_string());
                            }
                        });
                    });
                    ui.add_space(40.0);


                    if !self.mobile_timer.active {
                        ui.separator();
                        ui.add_space(40.0);
                        self.turn_infos(ui);
                        ui.add_space(40.0);
                        ui.separator();
                    }
                    // ui.separator();
                    ui.add_space(40.0);
                    ui.horizontal(|ui| {
                        let displayed_text: String = self
                            .history_san
                            .chars()
                            .rev()                 // inverse pour prendre depuis la fin
                            .take(73)              // prend les 73 derniers
                            .collect::<Vec<_>>()   // collect en vecteur de chars
                            .into_iter()
                            .rev()                 // remettre dans l’ordre correct
                            .collect();            // collect en String
                        ui.monospace(displayed_text);
                    });
                    ui.add_space(40.0);
                    // ui.add_space(40.0);

                    if self.app_mode != Versus {
                        // self.ui_timer_win(ui, ctx);
                    } else {
                        ui.add_space(50.0);
                    }
                    ui.add_space(100.0);
                    ui.horizontal(|ui| {
                        ui.add_space(60.0);
                        if ui.add_enabled(self.mobile_win.is_none(), egui::Button::new("Options"))
                        .clicked() {
                            self.mobile_win = Some(Options);
                        }
                        match self.app_mode {
                            AppMode::Replay => {
                                ui.add_space(200.0);
                                if ui.button("|<").clicked() {
                                    log::info!("Ouverture options !");
                                }
                                ui.add_space(40.0);
                                if ui.button("<").clicked() {
                                    log::info!("Ouverture options !");
                                }
                                ui.add_space(40.0);
                                if ui.button("Play").clicked() {
                                    log::info!("Ouverture options !");
                                }
                                ui.add_space(40.0);
                                if ui.button(">").clicked() {
                                    log::info!("Ouverture options !");
                                }
                                ui.add_space(40.0);
                                if ui.button(">|").clicked() {
                                    log::info!("Ouverture options !");
                                }
                            },
                            AppMode::Lobby => {
                                ui.add_space(180.0);
                                if ui.add_enabled(self.mobile_win.is_none(), egui::Button::new("Timer"))
                                .clicked() {
                                    self.mobile_win = Some(Timer);
                                }
                                ui.add_space(180.0);
                                if ui.add_enabled(!self.history.is_empty() && self.mobile_win.is_none(), egui::Button::new("New Game"))
                                .clicked()
                                    {
                                        self.history.clear();
                                        self.history_san.clear();
                                        self.current = GameState { // faire un builder
                                            board: Board::init_board(),
                                            active_player: Color::White,
                                            opponent: Color::Black,
                                            end: None,
                                            last_move: None,
                                            turn: 1,
                                        }
                                    }
                            }
                            AppMode::Versus => {
                                ui.add_space(400.0);
                                if ui.button("Draw").clicked() {
                                   self.mobile_win = Some(Draw);
                                }
                                ui.add_space(40.0);
                                if ui.button("Resign").clicked() {
                                   self.mobile_win = Some(Resign);
                                }
                            },
                        }
                    });
                });
            });
        });
    }
}
