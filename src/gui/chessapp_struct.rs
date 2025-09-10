use crate::Board;
use crate::Color;
use crate::Coord;

use eframe::{App, egui};
use egui::Pos2;
use std::time::Instant;

#[derive(Clone)]
pub struct GameState {
    pub board: Board,
    pub active_player: Color,
    pub checkmate: bool,
    pub pat: bool,
    pub last_move: Option<(Coord, Coord)>,
    pub last_move_pgn: String,
    pub history_pgn: String,
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
    //gui cell to highlight
    pub from_cell: Option<Coord>,
    pub drag_from: Option<Coord>,
    pub drag_pos: Option<Pos2>,
    pub piece_legals_moves: Vec<Coord>,
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
                last_move_pgn: String::new(),
                history_pgn: String::new(),
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
            from_cell: None,
            drag_from: None,
            drag_pos: None,
            piece_legals_moves: Vec::new(),
        }
    }
}

//This App trait runs the eframe : fn update is the main loop, run for each frame
impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .default_width(180.0)
            .show(ctx, |ui| {
                self.side_panel_ui(ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel_ui(ui);
        });
    }
}
