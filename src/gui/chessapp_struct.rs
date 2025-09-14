use crate::Board;
use crate::Color;
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
    pub next_replay_time: Option<f64>,
    pub flip: bool,
    pub autoflip: bool,
    pub replay_speed: u64,
    pub replay_index: usize,
    pub timer: Option<f64>,
    pub start_time: f64,
    pub total_time: f64,
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
                next_replay_time: None,
                flip: true,
                autoflip: false,
                replay_speed: 1000,
                replay_index: 0,
                timer: None,
                start_time: 0.0,
                total_time: 600.0, 
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
        egui::TopBottomPanel::top("spacer_top")
            .show(ctx, |ui| {
                self.update_timer(ui);
                if let Some(time) = self.widgets.timer {
                    let elapsed_time = self.widgets.total_time - time;
                    if elapsed_time > 60.0 {
                        let min = (elapsed_time / 60.0).floor();
                        let sec = (elapsed_time % 60.0).floor();
                        let t = min.to_string() + ":" + &sec.to_string();
                        ui.label(t.to_string());
                    } else {
                        ui.label(elapsed_time.to_string());
                    }
                } else if ui.button("start timer").clicked() {
                    self.widgets.timer = Some(ui.input(|i| i.time));
                }
                // if let Some(time) = self.widgets.timer {
                //     ui.label("Timer = ");
                //     ui.label(time.to_string());
                // }
                ui.label("White");
                // ui.add(TextEdit::singleline(&mut self.white_name));
            });
        egui::TopBottomPanel::bottom("spacer_bottom")
            .show(ctx, |ui| {
                ui.label("Black");
                // ui.add(TextEdit::singleline(&mut self.black_name));
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel_ui(ui);
        });
    }
}


impl ChessApp {
    fn update_timer(&mut self, ui: &mut egui::Ui) {
        if let Some(_) = self.widgets.timer {
            self.widgets.timer = Some(ui.input(|i| i.time));
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
