use crate::ChessApp;
use crate::Color;
use crate::Color::*;
use crate::engine::bot::BotDifficulty::*;
use crate::engine::bot::PlayerType::*;
use crate::engine::search_stats::SearchStats;
use crate::gui::chessapp::AppMode::*;
use crate::gui::layout::UiType::*;

impl ChessApp {
    pub fn engine_infos(&self, ui: &mut egui::Ui, color: &Color) {
        if self.app_mode == Versus(None)
            && self.settings.black_bot != Human
            && self.settings.black_bot != Bot(Easy)
        {
            match self.ui_type {
                Desktop => {
                    if self.app_mode == Versus(None)
                        && self.settings.black_bot != Human
                        && self.settings.black_bot != Bot(Easy)
                    //revoir ce if
                    {
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("Minimax").strong());
                                ui.label(
                                    egui::RichText::new(format!("depth: {}", self.get_depth()))
                                        .monospace(),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Time: {}",
                                        SearchStats::format_time(self.stats.bot_time_thinking)
                                    ))
                                    .monospace(),
                                );
                                ui.label(
                                    egui::RichText::new(format!("Nodes: {}", self.stats.nodes))
                                        .monospace(),
                                );
                                ui.label(
                                    egui::RichText::new(format!("Cutoffs: {}", self.stats.cutoffs))
                                        .monospace(),
                                );
                                ui.label(
                                    egui::RichText::new(format!("NPS: {:.0}", self.stats.nps))
                                        .monospace(),
                                );

                                ui.label(
                                    egui::RichText::new(format!(
                                        "Score: {:.0}",
                                        self.current.board.score
                                    ))
                                    .monospace(),
                                );
                            });
                        });
                    }
                }
                Mobile => {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("Depth: {}", self.get_depth())).small(),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "Time: {}",
                                    SearchStats::format_time(self.stats.bot_time_thinking)
                                ))
                                .small(),
                            );
                            match color {
                                White => ui.label(
                                    egui::RichText::new(format!(
                                        "Score: {:.0}",
                                        self.white_last_score
                                    ))
                                    .small(),
                                ),
                                Black => ui.label(
                                    egui::RichText::new(format!(
                                        "Score: {:.0}",
                                        self.black_last_score
                                    ))
                                    .small(),
                                ),
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("Nodes: {}", self.stats.nodes)).small(),
                            );
                            ui.label(
                                egui::RichText::new(format!("Cutoffs: {}", self.stats.cutoffs))
                                    .small(),
                            );
                            ui.label(
                                egui::RichText::new(format!("NPS: {:.0}", self.stats.nps)).small(),
                            );
                        });
                    });
                }
            };
        }
    }
    // pub fn engine_infos(&self, ui: &mut egui::Ui) {

    // }
}
