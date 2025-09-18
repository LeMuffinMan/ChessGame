use crate::Board;
use crate::Color;
use crate::Coord;
use crate::gui::chessapp_struct::DrawOption::*;
use crate::gui::widgets::replay::Timer;
use crate::gui::chessapp_struct::GameMode::NoTime;
use crate::gui::central_panel::central_panel_ui::*;
use crate::gui::chessapp_struct::GameMode::*;

use eframe::{App, egui};
use egui::RichText;
use egui::Pos2;
use egui::FontId;
use egui::TextStyle;
use std::collections::HashMap;
use std::path::PathBuf;

//a file for enums ?
#[derive(Clone, Copy, PartialEq)]
pub enum GameMode {
    Bullet(f64, f64), //(timer, increment)
    Blitz(f64, f64),
    Rapid(f64, f64),
    Custom(f64, f64),
    NoTime(f64, f64),
    Replay(f64, f64),
}

#[derive(Clone, PartialEq)]
pub enum DrawRule {
    TripleRepetition,
    FiftyMoves,
}

#[derive(Clone, PartialEq)]
pub enum DrawOption {
    Request,
    Available(DrawRule),
}

#[derive(Clone, PartialEq)]
pub enum End {
    Checkmate,
    TimeOut,
    Pat,
    Draw,
    Resign,
}

//a file for structs ?
#[derive(Clone)]
pub struct PromoteInfo {
    pub from: Coord,
    pub to: Coord,
    pub prev_board: Board,
}

#[derive(Clone, PartialEq)]
pub struct GameState {
    pub board: Board,
    pub active_player: Color,
    pub opponent: Color,
    pub end: Option<End>,
    pub last_move: Option<(Coord, Coord)>,
    pub turn: u32,
}

pub struct LateDraw {
    pub board_hashs: HashMap<u64, usize>,
    pub draw_option: Option<DrawOption>,
    pub draw_moves_count: u32,
    pub draw_hash: Option<u64>,
}

pub struct Highlight {
    pub from_cell: Option<Coord>,
    pub drag_from: Option<Coord>,
    pub drag_pos: Option<Pos2>,
    pub piece_legals_moves: Vec<Coord>,
}

pub struct Widgets {
    pub show_coordinates: bool,
    pub show_legals_moves: bool,
    pub show_last_move: bool,
    pub show_threaten_cells: bool,
    pub custom_timer: bool,
    pub next_replay_time: Option<f64>,
    pub flip: bool,
    pub autoflip: bool,
    pub replay_speed: f64,
    pub replay_index: usize,
    pub timer: Option<Timer>,
    pub game_mode: GameMode,
    pub file_name: String,
}

pub struct ChessApp {

    pub mobile: bool,
    pub mobile_timer: MobileTimer,
    //snapshots of all gamestates as history
    pub current: GameState,
    pub history: Vec<GameState>,
    pub history_san: String,

    //rendering and interface stuff
    pub widgets: Widgets,
    pub highlight: Highlight,
    pub white_name: String,
    pub black_name: String,

    //info to transmit to board / to move in board
    pub draw: LateDraw,
    pub promoteinfo: Option<PromoteInfo>,

    //Window dialog for special events
    //used to block any inputs and force user to choose an option
    pub win_dialog: bool,
    pub win_resign: bool,
    pub win_undo: bool,
    pub win_save: bool,

    pub file_path: Option<PathBuf>,
}

pub struct MobileTimer {
    start: f64,
    increment: f64,
    active: bool,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            mobile: false,
            mobile_timer: MobileTimer {
                start: 0.0,
                increment: 0.0,
                active: false,
            },
            current: GameState {
                board: Board::init_board(),
                active_player: Color::White,
                opponent: Color::Black,
                end: None,
                last_move: None,
                turn: 1,
            },
            history: Vec::new(),
            history_san: String::new(),
            widgets: Widgets {
                show_coordinates: false,
                show_legals_moves: true,
                show_last_move: true,
                show_threaten_cells: false,
                custom_timer: false,
                next_replay_time: None,
                flip: true,
                autoflip: false,
                replay_speed: 1.0,
                replay_index: 0,
                timer: None,
                game_mode: NoTime(0.0, 0.0),
                file_name: "chessgame.pgn".to_string(),
            },
            highlight: Highlight {
                from_cell: None,
                drag_from: None,
                drag_pos: None,
                piece_legals_moves: Vec::new(),
            },
            draw: LateDraw {
                board_hashs: HashMap::new(),
                draw_option: None,
                draw_moves_count: 0,
                draw_hash: None,
            },
            promoteinfo: None,
            // file_dialog: FileDialog::new(),
            file_path: None,
            white_name: "White".to_string(),
            black_name: "Black".to_string(),
            win_dialog: false,
            win_resign: false,
            win_undo: false,
            win_save: false,
        }
    }
}

impl ChessApp {
    pub fn new(mobile: bool) -> Self {
        Self {
            mobile: mobile,
            mobile_timer: MobileTimer {
                start: 0.0,
                increment: 0.0,
                active: false,
            },
            current: GameState {
                board: Board::init_board(),
                active_player: Color::White,
                opponent: Color::Black,
                end: None,
                last_move: None,
                turn: 1,
            },
            history: Vec::new(),
            history_san: String::new(),
            widgets: Widgets {
                show_coordinates: false,
                show_legals_moves: true,
                show_last_move: true,
                show_threaten_cells: false,
                custom_timer: false,
                next_replay_time: None,
                flip: true,
                autoflip: false,
                replay_speed: 1.0,
                replay_index: 0,
                timer: None,
                game_mode: NoTime(0.0, 0.0),
                file_name: "chessgame.pgn".to_string(),
            },
            highlight: Highlight {
                from_cell: None,
                drag_from: None,
                drag_pos: None,
                piece_legals_moves: Vec::new(),
            },
            draw: LateDraw {
                board_hashs: HashMap::new(),
                draw_option: None,
                draw_moves_count: 0,
                draw_hash: None,
            },
            promoteinfo: None,
            // file_dialog: FileDialog::new(),
            file_path: None,
            white_name: "White".to_string(),
            black_name: "Black".to_string(),
            win_dialog: false,
            win_resign: false,
            win_undo: false,
            win_save: false,
            // init de ton état
        }
    }
}

//This App trait runs the eframe : fn update is the main loop, run for each frame
impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.mobile {
            self.ui_mobile(ctx, frame);
        } else {
            self.ui_desktop(ctx);
        }
    }
}

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
        
        // egui::TopBottomPanel::top("title").show(ctx, |ui| {
        //     ui.label("1. e4 e5  2. Nf3 Nc6  3. Bb5 a6  4. Ba4 Nf6  5. O-O Be7");
        // });
        
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
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                        ui.label(
                            RichText::new("1. e4 e5  2. Nf3 Nc6  3. Bb5 a6  4. Ba4 Nf6  5. O-O Be7")
                            .text_style(TextStyle::Monospace),
                            );
                        });
                    });
                    // barre joueur noir
                    ui.horizontal(|ui| {
                        ui.label("Black");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("⏱ 05:32");
                        });
                    });

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

                    // barre joueur blanc
                    ui.horizontal(|ui| {
                        ui.label("White");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("⏱ 06:10");
                        });
                    });
                    ui.add_space(40.0);
                    self.ui_mobile_timers(ui);
                    ui.horizontal(|ui| {
                        if ui.button("Nouvelle Partie").clicked() {
                            log::info!("Nouvelle partie !");
                        }
                        if ui.button("Options").clicked() {
                            log::info!("Ouverture options !");
                        }
                        if ui.button("Undo").clicked() {
                            log::info!("Undo move !");
                        }
                    });
                });
            });
        });
    }
    pub fn ui_mobile_timers(&mut self, ui: &mut egui::Ui) {

        if self.mobile_timer.active == false {
            if ui.add_enabled(!self.mobile_timer.active, egui::Button::new("Timer"))
            .clicked()
            {
                self.mobile_timer.active = !self.mobile_timer.active; 
            }
        } else {
            if ui.button("Timer OFF").clicked() {
                self.mobile_timer.active = !self.mobile_timer.active; 
            }
        }
        let total_width = ui.available_width();
        let col_width = total_width / 3.0;
       
        if self.mobile_timer.active {
            ui.horizontal(|ui| {
                let total_width = ui.available_width();
                let col_width = total_width / 3.0;

                ui.add_space(col_width / 3.0);
                // Bullet
                ui.allocate_ui(egui::Vec2::new(col_width, ui.available_height()), |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("  Bullet").size(40.0));
                        ui.add_space(20.0);
                        ui.selectable_value(&mut self.widgets.game_mode, Bullet(20.0, 1.0), "0:20 + 1");
                        ui.selectable_value(&mut self.widgets.game_mode, Bullet(30.0, 0.0), "0:30 + 0");
                        ui.selectable_value(&mut self.widgets.game_mode, Bullet(60.0, 0.0), "1:00 + 0");
                        ui.selectable_value(&mut self.widgets.game_mode, Bullet(60.0, 1.0), "1:00 + 1");
                    });
                });
                ui.add_space(col_width / 6.0);
                ui.separator();
                ui.add_space(col_width / 6.0);
                // Blitz
                ui.allocate_ui(egui::Vec2::new(col_width, ui.available_height()), |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("   Blitz").size(40.0));
                        ui.add_space(20.0);
                        ui.selectable_value(&mut self.widgets.game_mode, Blitz(180.0, 0.0), "3:00 + 0");
                        ui.selectable_value(&mut self.widgets.game_mode, Blitz(180.0, 2.0), "3:00 + 2");
                        ui.selectable_value(&mut self.widgets.game_mode, Blitz(300.0, 0.0), "5:00 + 0");
                        ui.selectable_value(&mut self.widgets.game_mode, Blitz(300.0, 5.0), "5:00 + 5");
                        ui.selectable_value(&mut self.widgets.game_mode, Blitz(300.0, 2.0), "5:00 + 2");
                    });
                });
                ui.add_space(col_width / 6.0);
                ui.separator();
                ui.add_space(col_width / 6.0);
                // Rapid
                ui.allocate_ui(egui::Vec2::new(col_width, ui.available_height()), |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("    Rapid").size(40.0));
                        ui.add_space(20.0);
                        ui.selectable_value(&mut self.widgets.game_mode, Rapid(600.0, 0.0), "10:00 + 0");
                        ui.selectable_value(&mut self.widgets.game_mode, Rapid(600.0, 5.0), "10:00 + 5");
                        ui.selectable_value(&mut self.widgets.game_mode, Rapid(900.0, 10.0), "15:00 + 10");
                        ui.selectable_value(&mut self.widgets.game_mode, Rapid(1200.0, 0.0), "20:00 + 0");
                        ui.selectable_value(&mut self.widgets.game_mode, Rapid(1800.0, 0.0), "30:00 + 0");
                        ui.selectable_value(&mut self.widgets.game_mode, Rapid(3600.0, 5.0), "60:00 + 0");
                    });
                });
                ui.add_space(col_width / 3.0);
            });
        }
    }
    pub fn ui_desktop(&mut self, ctx: &egui::Context) {

        self.update_timer(ctx);

        //Promotion
        if self.widgets.replay_index == self.history.len()
            && self.current.board.pawn_to_promote.is_some()
        {
            self.get_promotion_input(ctx);
        }
        //Undo confirmation
        if self.win_undo {
            self.ask_to_undo(ctx);
        }
        //resign confirmation
        if self.win_resign {
            self.resign_confirm(ctx);
        }
        //draw_offer
        if let Some(rq) = &self.draw.draw_option
            && *rq == Request
        {
            self.offer_draw(ctx);
        }
        //save menu
        if self.win_save {
            self.save_game(ctx);
        }

        self.top_title_panel(ctx);
        self.bot_source_code_panel(ctx);

        self.left_panel_ui(ctx);
        self.right_panel_ui(ctx);

        self.top_black_panel(ctx);
        self.bot_white_panel(ctx);

        self.central_panel_ui(ctx);
    }

    pub fn is_active_player_piece(&mut self, coord: &Coord) -> bool {
        let cell = self.current.board.get(coord);
        cell.is_color(&self.current.active_player)
    }
}
