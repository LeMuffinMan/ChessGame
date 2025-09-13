use crate::Board;
use crate::Color;
use crate::Coord;
use crate::gui::chessapp_struct::End::*;

use eframe::{App, egui};
use egui::Pos2;
// use egui_file_dialog::FileDialog;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

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

#[derive(Clone)]
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
    pub next_replay_time: Option<Instant>,
    pub flip: bool,
    pub autoflip: bool,
    pub replay_speed: u64,
}

pub struct ChessApp {
    //history undo / redo
    pub current: GameState,
    pub undo: Vec<GameState>,
    pub redo: Vec<GameState>,
    pub widgets: Widgets,
    pub highlight: Highlight,
    pub draw: Draw,
    pub promoteinfo: Option<PromoteInfo>,
    // pub file_dialog: FileDialog,
    pub file_path: Option<PathBuf>,
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
            undo: Vec::new(),
            redo: Vec::new(),
            widgets: Widgets {
                show_coordinates: false,
                show_legals_moves: true,
                show_last_move: true,
                show_threaten_cells: false,
                next_replay_time: None,
                flip: true,
                autoflip: false,
                replay_speed: 1000,
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
        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel_ui(ui);
        });
    }
}

//
// impl eframe::App for ChessApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.label("ChessGame fonctionne !");
//             ui.allocate_exact_size(egui::Vec2::new(800.0, 600.0), egui::Sense::hover());
//         });
//     }
// }

impl ChessApp {
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
