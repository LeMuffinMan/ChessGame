use crate::Coord;
use crate::engine::bot::BotDifficulty::*;
use crate::engine::bot::PlayerType;
use crate::engine::bot::PlayerType::*;
use egui::Pos2;
use std::path::PathBuf;

pub struct Settings {
    pub from_cell: Option<Coord>,
    pub drag_from: Option<Coord>,
    pub drag_pos: Option<Pos2>,
    pub piece_legals_moves: Vec<Coord>,
    pub show_coordinates: bool,
    pub show_legals_moves: bool,
    pub show_last_move: bool,
    pub show_threaten_cells: bool,
    pub flip: bool,
    pub autoflip: bool,
    pub file_name: String,
    pub white_name: String,
    pub black_name: String,
    pub file_path: Option<PathBuf>,
    pub allow_undo: bool,
    pub white_undo: u8,
    pub black_undo: u8,
    pub undo_limit: u8,
    pub white_bot: PlayerType,
    pub black_bot: PlayerType,
    pub white_bot_depth: u8,
    pub black_bot_depth: u8,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            show_coordinates: false,
            show_legals_moves: true,
            show_last_move: true,
            show_threaten_cells: false,
            flip: true,
            autoflip: false,
            file_name: "chessgame.pgn".to_string(),
            from_cell: None,
            drag_from: None,
            drag_pos: None,
            piece_legals_moves: Vec::new(),
            white_name: "White".to_string(),
            black_name: "Black".to_string(),
            file_path: None,
            allow_undo: false,
            white_undo: 0,
            black_undo: 0,
            undo_limit: 0,
            white_bot: Human,
            black_bot: Bot(Medium),
            white_bot_depth: 6,
            black_bot_depth: 6,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
