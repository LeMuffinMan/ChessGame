use crate::Board;
use crate::Color;
use crate::Coord;

use eframe::{App, egui};
use egui::Pos2;
use std::time::Instant;
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

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
    pub checkmate: bool,
    pub pat: bool,
    pub last_move: Option<(Coord, Coord)>,
    // pub last_move_san: String,
    pub history_san: String,
    pub turn: u32,
}

pub struct ChessApp {
    //history undo / redo
    pub current: GameState,
    pub undo: Vec<GameState>,
    pub redo: Vec<GameState>,
    pub next_replay_time: Option<Instant>,
    //gui options
    pub flip: bool,
    pub autoflip: bool,
    pub replay_speed: u64,
    pub show_coordinates: bool,
    pub show_legals_moves: bool,
    pub show_last_move: bool,
    pub show_threaten_cells: bool,
    pub promoteinfo: Option<PromoteInfo>,
    //gui cell to highlight
    pub from_cell: Option<Coord>,
    pub drag_from: Option<Coord>,
    pub drag_pos: Option<Pos2>,
    pub piece_legals_moves: Vec<Coord>,
    pub file_dialog: FileDialog,
    pub file_path: Option<PathBuf>,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            current: GameState {
                board: Board::init_board(),
                active_player: Color::White,
                checkmate: false,
                pat: false,
                last_move: None,
                // last_move_san: String::new(),
                history_san: String::new(),
                turn: 1,
            },
            undo: Vec::new(),
            redo: Vec::new(),
            next_replay_time: None,
            replay_speed: 1000,
            flip: true,
            autoflip: false,
            show_coordinates: false,
            show_legals_moves: true,
            show_last_move: true,
            show_threaten_cells: false,
            promoteinfo: None,
            from_cell: None,
            drag_from: None,
            drag_pos: None,
            piece_legals_moves: Vec::new(),
            file_dialog: FileDialog::new(),
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
