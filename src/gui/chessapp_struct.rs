use crate::Board;
use crate::Color;
use crate::Color::*;
use crate::Coord;
// use crate::gui::chessapp_struct::LateDraw;
use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::DrawOption::*;
use crate::gui::chessapp_struct::End::*;
use crate::gui::widgets::undo_redo_replay::Timer;

use eframe::{App, egui};
use egui::Pos2;
// use egui_file_dialog::FileDialog;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Copy, PartialEq)]
pub enum GameMode {
    Bullet(f64, f64), //(timer, increment)
    Blitz(f64, f64),
    Rapid(f64, f64),
    Custom(f64, f64),
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
    pub game_mode: Option<GameMode>,
}

pub struct ChessApp {
    //history undo / redo
    pub current: GameState,
    pub history: Vec<GameState>,
    pub history_san: String,
    pub widgets: Widgets,
    pub highlight: Highlight,
    pub draw: LateDraw,
    pub promoteinfo: Option<PromoteInfo>,
    // pub file_dialog: FileDialog,
    pub file_path: Option<PathBuf>,
    pub white_name: String,
    pub black_name: String,
    pub win_dialog: bool,
    pub win_resign: bool,
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
                game_mode: None,
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
        }
    }
}

//This App trait runs the eframe : fn update is the main loop, run for each frame
impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_timer(ctx);

        //Promotion window dialog
        if self.widgets.replay_index == self.history.len()
            && self.current.board.pawn_to_promote.is_some()
        {
            self.win_dialog = true;
            egui::Window::new("Promotion")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.radio_value(&mut self.current.board.promote, Some(Queen), "Queen");
                        ui.radio_value(&mut self.current.board.promote, Some(Bishop), "Bishop");
                        ui.radio_value(&mut self.current.board.promote, Some(Knight), "Knight");
                        ui.radio_value(&mut self.current.board.promote, Some(Rook), "Rook");
                    });
                });
            self.update_promote();
        }

        if self.win_resign {
            egui::Window::new("Resignation ?")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    self.win_dialog = true;
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.add_space(20.0);
                        if ui.button("Accept").clicked() {
                            self.current.end = Some(Resign);
                            self.win_resign = false;
                            self.win_dialog = false;
                        }
                        ui.add_space(30.0);
                        if ui.button("Decline").clicked() {
                            self.win_resign = false;
                            self.win_dialog = false;
                        }
                        ui.add_space(20.0);
                    });
                    ui.add_space(10.0);
                });
        }

        if let Some(rq) = &self.draw.draw_option
            && *rq == Request
        {
            egui::Window::new(format!("{:?} offers a draw", self.current.active_player))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.add_space(20.0);
                        if ui.button("Accept").clicked() {
                            self.current.end = Some(Draw);
                            self.draw.draw_option = None;
                        }
                        ui.add_space(30.0);
                        if ui.button("Decline").clicked() {
                            self.draw.draw_option = None;
                        }
                        ui.add_space(20.0);
                    });
                    ui.add_space(10.0);
                });
        }

        self.top_title_panel(ctx);
        self.bot_source_code_panel(ctx);

        self.left_panel_ui(ctx);
        self.right_panel_ui(ctx);

        self.top_black_panel(ctx);
        self.bot_white_panel(ctx);

        self.central_panel_ui(ctx);
    }
}

impl ChessApp {
    pub fn draw_request(&mut self, ui: &mut egui::Ui) {
        ui.label("Accept draw offer ?");
        ui.horizontal(|ui| {
            if ui.button("Accept").clicked() {
                self.current.end = Some(Draw);
                self.draw.draw_option = None;
            }
            if ui.button("Reject").clicked() {
                self.draw.draw_option = None;
            }
        });
    }
    pub fn update_promote(&mut self) {
        if self.widgets.replay_index == self.history.len()
            && let Some(coord) = self.current.board.pawn_to_promote
            && let Some(piece) = self.current.board.promote
        {
            let color = if self.current.active_player == Color::White {
                Black
            } else {
                White
            };
            self.current.board.grid[coord.row as usize][coord.col as usize] =
                Cell::Occupied(piece, color);

            let opponent = if self.current.active_player != White {
                White
            } else {
                Black
            };
            if let Some(k) = self.current.board.get_king(&opponent)
                && self.current.board.threaten_cells.contains(&k)
                && let Some(k) = self.current.board.get_king(&opponent)
            {
                self.current.board.check = Some(k);
                // println!("Check !");
            }
            self.check_endgame();
            if let Some(promoteinfo) = &self.promoteinfo {
                let from = promoteinfo.from;
                let to = promoteinfo.to;
                let prev_board = promoteinfo.prev_board.clone();
                self.history.push(self.current.clone());
                self.widgets.replay_index += 1;
                self.encode_move_to_san(&from, &to, &prev_board);
            }
            self.current.board.pawn_to_promote = None;
            self.current.board.promote = None;
            self.win_dialog = false;
        }
    }
    pub fn is_active_player_piece(&mut self, coord: &Coord) -> bool {
        let cell = self.current.board.get(coord);
        cell.is_color(&self.current.active_player)
    }
}
