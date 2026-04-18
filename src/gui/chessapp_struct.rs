use crate::Board;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::AppMode::*;
use crate::gui::game_state_struct::GameState;
use crate::gui::hooks::WinDia;
use crate::gui::replay::ReplayInfos;
use crate::gui::update_timer::GameMode;
use crate::gui::update_timer::Timer;
use eframe::{App, egui};
use egui::Pos2;
use std::path::PathBuf;

#[derive(Clone, PartialEq)]
pub enum End {
    Checkmate,
    TimeOut,
    Pat,
    Draw,
    Resign,
}

#[derive(PartialEq)]
pub enum AppMode {
    Versus(Option<End>),
    Replay,
    Lobby,
}

#[derive(Clone, PartialEq)]
pub enum UiType {
    Desktop,
    Mobile,
}

#[derive(Clone)]
pub struct PromoteInfo {
    pub from: Coord,
    pub to: Coord,
    pub prev_board: Board,
    pub pawn_to_promote: Option<Coord>,
    pub promote: Option<Piece>,
}

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
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

pub struct History {
    pub snapshots: Vec<GameState>,
    //pub coord: Vec<Option<coord>, Option<coord>>,
    pub history_san: String,
}

impl History {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            history_san: String::new(),
        }
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ChessApp {
    pub ui_type: UiType,
    pub app_mode: AppMode,
    pub settings: Settings,
    pub win: Option<WinDia>,
    pub timer: Timer,
    pub replay_infos: ReplayInfos,
    pub promoteinfo: Option<PromoteInfo>,
    pub current: GameState,
    pub history: History,
}

impl ChessApp {
    pub fn new(ui_type: UiType) -> Self {
        Self {
            ui_type,
            history: History::new(),
            timer: Timer::new(),
            win: None,
            app_mode: Lobby,
            replay_infos: ReplayInfos::new(),
            current: GameState::new(),
            settings: Settings::new(),
            promoteinfo: None,
        }
    }
}

//This App trait runs the eframe : fn update is the main loop, run for each frame
impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.current.threaten_cells.is_empty() {
            self.current.threaten_cells = self
                .current
                .board
                .update_threatens_cells(&self.current.active_player)
        }
        if self.current.legals_moves.is_empty() {
            self.current.legals_moves = self
                .current
                .board
                .update_legals_moves(&self.current.active_player, &self.current.threaten_cells)
        }
        self.hooks(ctx);
        match &self.ui_type {
            UiType::Mobile => {
                self.apply_styles(ctx);
                self.top_title_panel(ctx);
                self.central_panel_mobile(ctx);
            }
            UiType::Desktop => {
                self.apply_desktop_styles(ctx);
                self.top_title_panel(ctx);
                self.bot_source_code_panel_desktop(ctx);
                self.left_panel_desktop(ctx);
                self.right_panel_desktop(ctx);
                self.top_black_panel_desktop(ctx);
                self.bot_white_panel_desktop(ctx);
                self.central_panel_desktop(ctx);
            }
        }
    }
}

impl ChessApp {
    pub fn hooks(&mut self, ctx: &egui::Context) {
        self.hook_win(ctx);
        if self.app_mode == Replay {
            self.mobile_replay_step(ctx);
        }
        if self.timer.mode != GameMode::NoTime && self.timer.active {
            if self.timer.update_timer(ctx, &self.current.active_player) {
                self.current.end = Some(End::TimeOut);
            }
            ctx.request_repaint();
        }
        if matches!(self.app_mode, AppMode::Versus(_))
            && self.replay_infos.index == self.history.snapshots.len()
            && self.promoteinfo.is_some()
        {
            self.get_promotion_input(ctx);
        }
    }

    pub fn top_title_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("title").show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.heading("ChessGame");
                },
            );
        });
    }
    //Since we need player input to know in which piece promote a pawn, i need to
    //store the coord of the pawn to promote and stop the try move process
    //the GUI will hook on the coord position stored and force player to input a desired promotion
    //Then this hook process the end of try move we skipped earlier
    pub fn promote_pawn(
        &mut self,
        color: &Color,
        from: &Coord,
        to: &Coord,
        prev_board: &Board,
    ) -> Option<PromoteInfo> {
        let promote_row = if *color == White { 7 } else { 0 };
        for y in 0..8 {
            if self.current.board.grid[promote_row][y].is_color(color)
                && let Some(piece) = self.current.board.grid[promote_row][y].get_piece()
                && *piece == Pawn
            {
                // let coord = Coord {
                //     row: promote_row as u8,
                //     col: y as u8,
                // };
                return Some(PromoteInfo {
                    from: *from,
                    to: *to,
                    prev_board: prev_board.clone(), //le clone est problematique ici ?
                    pawn_to_promote: Some(*to),
                    promote: None, // on attend l'input du user fournie par le hook
                });
            }
        }
        None
    }
}
