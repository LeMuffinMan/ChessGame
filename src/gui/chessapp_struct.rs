use crate::Board;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::gui::chessapp_struct::End::*;

use eframe::{App, egui};
use egui::Pos2;
// use egui_file_dialog::FileDialog;
use std::collections::HashMap;
use std::path::PathBuf;

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
    pub history_san: String,
    pub turn: u32,
}

pub struct Draw {
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
    pub show_timer: bool,
    pub next_replay_time: Option<f64>,
    pub flip: bool,
    pub autoflip: bool,
    pub replay_speed: u64,
    pub replay_index: usize,
    pub timer: Option<Timer>,
}

pub struct ChessApp {
    //history undo / redo
    pub current: GameState,
    pub history: Vec<GameState>,
    pub widgets: Widgets,
    pub highlight: Highlight,
    pub draw: Draw,
    pub promoteinfo: Option<PromoteInfo>,
    // pub file_dialog: FileDialog,
    pub file_path: Option<PathBuf>,
    pub white_name: String,
    pub black_name: String,
}

pub struct Timer {
    pub white: (Option<f64>, f64), //start of turn, remaining time
    pub black: (Option<f64>, f64),
    pub increment: f64,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            white: (None, 600.0),
            black: (None, 600.0),
            increment: 0.0,
        }
    }
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            current: GameState {
                board: Board::init_board(),
                active_player: Color::White,
                opponent: Color::Black,
                end: None,
                last_move: None,
                history_san: String::new(),
                turn: 1,
            },
            history: Vec::new(),
            widgets: Widgets {
                show_coordinates: false,
                show_legals_moves: true,
                show_last_move: true,
                show_threaten_cells: false,
                show_timer: false,
                next_replay_time: None,
                flip: true,
                autoflip: false,
                replay_speed: 1000,
                replay_index: 0,
                timer: None,
            },
            highlight: Highlight {
                from_cell: None,
                drag_from: None,
                drag_pos: None,
                piece_legals_moves: Vec::new(),
            },
            draw: Draw {
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
        }
    }
}

//This App trait runs the eframe : fn update is the main loop, run for each frame
impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_timer(ctx);
        egui::TopBottomPanel::top("title")
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                    ui.heading("ChessGame");
                });
            });
        egui::TopBottomPanel::bottom("source code")
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                    ui.label("source code : https://github.com/LeMuffinMan/ChessGame");
                });
            });
        egui::SidePanel::left("left_panel")
            .default_width(180.0)
            .show(ctx, |ui| {
                self.side_panel_ui(ui, ctx);
            });
        egui::SidePanel::right("right_panel")
            .default_width(180.0)
            .show(ctx, |ui| {
                self.right_panel_ui(ui, ctx);
            });

                // ui.add(TextEdit::singleline(&mut self.white_name));
        egui::TopBottomPanel::top("spacer_top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let now = ctx.input(|i| i.time);
                if let Some(timer) = &self.widgets.timer {
                    let rem = {
                        if let Some(start) = timer.black.0 {
                            timer.black.1 - (now - start)
                        } else {
                            timer.black.1
                        }
                    }.max(0.0);
                    if rem == 0.0 {
                        self.current.end = Some(TimeOut);
                        self.widgets.timer = None;
                    }
                    ui.label(format_time(rem));
                }
                ui.label("Black");
            });
        });
        egui::TopBottomPanel::bottom("spacer_bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let now = ctx.input(|i| i.time);
                if let Some(timer) = &self.widgets.timer {
                    let rem = {
                        if let Some(start) = timer.white.0 {
                            timer.white.1 - (now - start)
                        } else {
                            timer.white.1
                        }
                    }.max(0.0);
                    if rem == 0.0 {
                        self.current.end = Some(TimeOut);
                    }
                    ui.label(format_time(rem));
                } 
                ui.label("White");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel_ui(ui);
        });
    }
}

fn format_time(seconds_f64: f64) -> String {
    let total_secs = seconds_f64.max(0.0).floor() as i64;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    if mins > 0 {
        format!("{}:{:02}", mins, secs) 
    } else {
        format!("0:{:02}", secs)        
    }
}


impl ChessApp {
    fn update_timer(&mut self, ctx: &egui::Context) {
        let now = ctx.input(|i| i.time);

        if let Some(timer) = &mut self.widgets.timer {
            if timer.white.0.is_none() && self.current.active_player == White && !self.history.is_empty() {
                timer.white.0 = Some(now);
                if let Some(black_start) = timer.black.0 {
                    timer.black.1 += timer.increment;
                    timer.black.1 -= now - black_start; 
                }
                timer.black.0 = None;
            }
            else if timer.black.0.is_none() && self.current.active_player == Black {
                timer.black.0 = Some(now);
                if let Some(white_start) = timer.white.0 {
                    timer.white.1 += timer.increment;
                    timer.white.1 -= now - white_start ;
                }
                timer.white.0 = None;
            }
            if timer.white.1 < 0.0 { 
                timer.white.1 = 0.0;
                self.current.end = Some(TimeOut);
            }
            if timer.black.1 < 0.0 {
                timer.black.1 = 0.0;
                self.current.end = Some(TimeOut);
            }
        }
    }   
    pub fn check_endgame(&mut self) {
        self.current
            .board
            .update_threatens_cells(&self.current.active_player);
        self.current
            .board
            .update_legals_moves(&self.current.active_player);
        // for coord in &self.current.board.threaten_cells {
        //     println!("Cell threaten : ({}, {})", coord.row, coord.col);
        // }
        if self.current.board.legals_moves.is_empty() {
            self.current.board.print();
            let king_cell = self.current.board.get_king(&self.current.active_player);
            if let Some(coord) = king_cell {
                if self.current.board.threaten_cells.contains(&coord) {
                    self.current.end = Some(Checkmate);
                } else {
                    self.current.end = Some(Pat);
                }
            }
        }
    }

    pub fn is_active_player_piece(&mut self, coord: &Coord) -> bool {
        let cell = self.current.board.get(coord);
        cell.is_color(&self.current.active_player)
    }
}
