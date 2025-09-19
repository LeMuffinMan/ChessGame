use crate::Board;
use crate::Color;
use crate::Coord;
use crate::gui::chessapp_struct::AppMode::*;
use crate::gui::chessapp_struct::DrawOption::*;

use eframe::{App, egui};
use egui::Pos2;

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
    pub flip: bool,
    pub autoflip: bool,
    pub file_name: String,
}

pub enum WinDia {
    Options,
    Promote,
    Draw,
    Resign,
    Timer,
    Undo,
    Pgn,
}

#[derive(PartialEq)]
pub enum MobileGameMode {
    Rapid,
    Blitz,
    Bullet,
    Custom,
    NoTime,
}

#[derive(PartialEq)]
pub struct MobileTimer {
    pub start: f64,
    pub increment: f64,
    pub active: bool,
    pub mode: MobileGameMode,
    pub white_time: f64,
    pub black_time: f64,
    pub start_of_turn: (f64, Option<Color>),
}

#[derive(PartialEq)]
pub enum AppMode {
    Versus(Option<End>),
    Replay,
    Lobby,
}

pub struct ReplayInfos {
    pub index: usize,
    pub sec_per_frame: f64,
    pub next_step: Option<f64>,
}

pub struct ChessApp {
    pub mobile: bool,
    pub mobile_timer: MobileTimer,
    pub mobile_win: Option<WinDia>,
    pub app_mode: AppMode,
    pub replay_infos: ReplayInfos,
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

    pub file_path: Option<PathBuf>,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            mobile: false,
            mobile_timer: MobileTimer {
                start: 0.0,
                increment: 0.0,
                active: false,
                mode: MobileGameMode::NoTime,
                white_time: 0.0,
                black_time: 0.0,
                start_of_turn: (0.0, None),
            },
            mobile_win: None,
            app_mode: Lobby,
            replay_infos: ReplayInfos {
                index: 0,
                sec_per_frame: 1.0,
                next_step: None,
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
                flip: true,
                autoflip: false,
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
        }
    }
}

impl ChessApp {
    pub fn new(mobile: bool) -> Self {
        Self {
            mobile,
            mobile_timer: MobileTimer {
                start: 0.0,
                increment: 0.0,
                active: false,
                mode: MobileGameMode::NoTime,
                white_time: 0.0,
                black_time: 0.0,
                start_of_turn: (0.0, None),
            },
            mobile_win: None,
            app_mode: Lobby,
            replay_infos: ReplayInfos {
                index: 0,
                sec_per_frame: 1.0,
                next_step: None,
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
                flip: true,
                autoflip: false,
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
        }
    }
}

//This App trait runs the eframe : fn update is the main loop, run for each frame
impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.hook_win_diag(ctx);
        if self.mobile_timer.mode != MobileGameMode::NoTime && self.mobile_timer.active {
            self.mobile_update_timer(ctx);
            ctx.request_repaint();
        }
        if self.mobile {
            self.ui_mobile(ctx, frame);
        } else {
            self.ui_desktop(ctx);
        }
    }
}

impl ChessApp {
    // pub fn ui_timer_win(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
    //     egui::Window::new("Timer")
    //         .collapsible(false)
    //         .resizable(false)
    //         .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
    //         .show(ctx, |ui| {
    //             self.ui_mobile_timers(ui);
    //         });
    // }
    pub fn ui_desktop(&mut self, ctx: &egui::Context) {
        // self.update_timer(ctx);

        //Promotion
        if self.replay_infos.index == self.history.len()
            && self.current.board.pawn_to_promote.is_some()
        {
            self.get_promotion_input(ctx);
        }
        //Undo confirmation
        // if self.win_undo {
        //     self.ask_to_undo(ctx);
        // }
        //resign confirmation
        // if self.win_resign {
        //     self.resign_confirm(ctx);
        // }
        //draw_offer
        if let Some(rq) = &self.draw.draw_option // a faire pour mobile
            && *rq == Request
        {
            self.offer_draw(ctx);
        }
        //save menu
        // if self.win_save {
        //     self.save_game(ctx);
        // }

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
