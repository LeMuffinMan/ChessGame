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
            && self.settings.black_bot != Bot(Random)
        {
            match self.ui_type {
                Desktop => {
                    // ui.group(|ui| {
                    ui.vertical(|ui| {
                        let time_ms = self.search_ctx.stats.bot_time_thinking;

                        let time_color = if time_ms < 300.0 {
                            egui::Color32::LIGHT_GREEN
                        } else if time_ms < 1000.0 {
                            egui::Color32::KHAKI
                        } else {
                            egui::Color32::from_rgb(255, 100, 100)
                        };

                        let score_color = if self.game.board.score >= 0 {
                            egui::Color32::from_rgb(140, 255, 140)
                        } else {
                            egui::Color32::from_rgb(255, 140, 140)
                        };

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Minimax engine").weak());
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let (rect, _) = ui.allocate_exact_size(
                                        egui::vec2(8.0, 8.0),
                                        egui::Sense::hover(),
                                    );
                                    ui.painter().circle_filled(rect.center(), 4.0, time_color);
                                },
                            );
                        });

                        ui.add_space(2.0);
                        ui.separator();
                        ui.add_space(4.0);

                        egui::Grid::new("main_stats")
                            .num_columns(2)
                            .spacing([32.0, 6.0])
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("Search Depth").small());
                                ui.label(
                                    egui::RichText::new(format!("{}", self.get_depth()))
                                        .strong()
                                        .small(),
                                );
                                ui.end_row();

                                ui.label(egui::RichText::new("Thinking Time").small());
                                ui.label(
                                    egui::RichText::new(SearchStats::format_time(time_ms))
                                        .color(time_color)
                                        .strong()
                                        .small(),
                                );
                                ui.end_row();

                                ui.label(egui::RichText::new("Board Score").small());
                                ui.label(
                                    egui::RichText::new(format!("{:.2}", self.game.board.score))
                                        .color(score_color)
                                        .strong()
                                        .small(),
                                );
                                ui.end_row();
                            });

                        ui.add_space(10.0);

                        ui.label(egui::RichText::new("PERFORMANCE").weak().small());
                        ui.add_space(2.0);

                        egui::Frame::new()
                            .fill(ui.visuals().faint_bg_color)
                            // .rounding(4.0)
                            .inner_margin(6.0)
                            .show(ui, |ui| {
                                egui::Grid::new("perf_grid")
                                    .num_columns(2)
                                    .spacing([10.0, 4.0])
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new("Nodes").small());
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "{} ({:.0} n/ms)",
                                                self.search_ctx.stats.nodes,
                                                self.search_ctx.stats.nps / 1000.0
                                            ))
                                            .small(),
                                        );
                                        ui.end_row();

                                        let prun_rate = (self.search_ctx.stats.cutoffs as f64
                                            / self.search_ctx.stats.nodes.max(1) as f64)
                                            * 100.0;
                                        ui.label(egui::RichText::new("Pruning").small());
                                        ui.label(
                                            egui::RichText::new(format!("{:.1}%", prun_rate))
                                                .small(),
                                        );
                                        ui.end_row();

                                        let tt_hit_pct = self.search_ctx.stats.tt_hits as f64
                                            / self.search_ctx.stats.nodes.max(1) as f64
                                            * 100.0;
                                        ui.label(egui::RichText::new("TT Hits").small());
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "{} ({:.1}%)",
                                                self.search_ctx.stats.tt_hits, tt_hit_pct
                                            ))
                                            .small(),
                                        );
                                        ui.end_row();
                                    });
                            });

                        ui.add_space(8.0);

                        egui::CollapsingHeader::new(egui::RichText::new("Depth Tree").small())
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.add_space(4.0);
                                egui::ScrollArea::vertical()
                                    .max_height(120.0)
                                    .auto_shrink([false, true])
                                    .show(ui, |ui| {
                                        egui::Grid::new("depth_tree").striped(true).show(
                                            ui,
                                            |ui| {
                                                for depth in 1..=self.game.depth {
                                                    ui.label(
                                                        egui::RichText::new(format!("D {}", depth))
                                                            .weak()
                                                            .small(),
                                                    );
                                                    ui.label(
                                                        egui::RichText::new(format!(
                                                            "{} n",
                                                            self.search_ctx.stats.nodes_per_depth
                                                                [depth as usize]
                                                        ))
                                                        .small(),
                                                    );
                                                    ui.label(
                                                        egui::RichText::new(format!(
                                                            "{} cut",
                                                            self.search_ctx.stats.cutoffs_per_depth
                                                                [depth as usize]
                                                        ))
                                                        .small(),
                                                    );
                                                    ui.end_row();
                                                }
                                            },
                                        );
                                    });

                                ui.add_space(4.0);
                                ui.separator();

                                let avg_depth = self.search_ctx.stats.total_node_depth as f64
                                    / self.search_ctx.stats.nodes.max(1) as f64;
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Avg Node Depth: {:.2}",
                                        avg_depth
                                    ))
                                    .small(),
                                );
                            });
                    });

                    // });
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
                                    SearchStats::format_time(
                                        self.search_ctx.stats.bot_time_thinking
                                    )
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
                                egui::RichText::new(format!(
                                    "Nodes: {}",
                                    self.search_ctx.stats.nodes
                                ))
                                .small(),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "Cutoffs: {}",
                                    self.search_ctx.stats.cutoffs
                                ))
                                .small(),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "NPS: {:.0}",
                                    self.search_ctx.stats.nps
                                ))
                                .small(),
                            );
                        });
                    });
                }
            };
        }
    }
}
