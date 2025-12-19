use crate::Board;
use crate::Coord;
use crate::Color::*;
use crate::gui::chessapp_struct::AppMode::*;
use crate::board::board_struct::End;
use crate::board::board_struct::End::*;
use crate::gui::hooks::WinDia;
use crate::gui::replay::ReplayInfos;
use crate::gui::update_timer::GameMode;
use crate::gui::update_timer::Timer;

use eframe::{App, egui};
use egui::Pos2;
use std::path::PathBuf;

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
    pub snapshots: Vec<Board>,
    pub headers: Vec<String>,
    //pub coord: Vec<Option<coord>, Option<coord>>,
    pub history_san: String,
}

impl History {
    pub fn new() -> History {
        History {
            headers: Vec::new(),
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
    pub board: Board,
    pub history: History,
    pub pgn_input: String,
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
            board: Board::new(),
            settings: Settings::new(),
            promoteinfo: None,
            pgn_input: String::new(),
        }
    }
}

//This App trait runs the eframe : fn update is the main loop, run for each frame
impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            if self.timer.update_timer(ctx, &self.board.active_player) {
                self.board.end = Some(End::TimeOut);
            }
            ctx.request_repaint();
        }
        if matches!(self.app_mode, AppMode::Versus(_))
            && self.replay_infos.index == self.history.snapshots.len()
            && self.board.pawn_to_promote.is_some()
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

    pub fn apply_move(&mut self, from: &Coord, to: &Coord) {
        self.update_history();
        //it triggers a draw if true, before update board for pawn detection in case of promotion
        self.board.fifty_moves_draw_check(&from, &to);
        //This apply the move on the board
        self.board
            .update_board(&from, &to);
        //it triggers a draw if the board match an impossible mat situation
        if self.board.impossible_mate_check() {
            self.board.end = Some(Draw);
            self.app_mode = Versus(Some(Draw));
        }
        //update castles bool state for both player
        self.board.update_castles(&to);
        //This add a hash for the 3 repetition draw
        //it takes player on trait, the grid, the castle and en_passant state
        //hash gives us the info if this exact situation happened
        self.board.add_hash();

        self.board.last_move = Some((*from, *to));

        if self.settings.autoflip {
            self.settings.flip = !self.settings.flip;
        }
        self.incremente_turn();

        //checks for promotion
        //switch player color
        //check for mate, or pat and finaly for check situation
        self.events_check();

        //since we must end this function to allow gui to ask for promotion input, we store infos
        //needed here, and we skip the "normal end" so the gui will do it after getting the input
        let prev_board = self.history.snapshots[self.replay_infos.index - 1]
            .clone();
        if self.board.pawn_to_promote.is_some() {
            self.promoteinfo = Some(PromoteInfo {
                from : *from,
                to : *to,
                prev_board: prev_board.clone(),
            });
        } else {
            //if there were no promotion, we add the actual board in history, and inc the index
            self.history.snapshots.push(self.board.clone());
            self.replay_infos.index += 1;
            self.encode_move_to_san(&from, &to, &prev_board);
        }
        self.settings.from_cell = None;
    }

    pub fn update_history(&mut self) {
        //if it's the very first move, we setup the history and timers if needed
        if self.history.snapshots.is_empty() {
            self.history.snapshots.push(self.board.clone());
            self.replay_infos.index += 1;
            //for mobile test
            self.app_mode = Versus(None);
            self.timer.active = true;
            self.timer.start_of_turn.1 = Some(White);
            //Setup les timers ici ?
        }
    }

    fn incremente_turn(&mut self) {
        if self.board.active_player == Black {
            self.board.turn += 1;
        }
    }

 
    //update threats and legals moves to determine if it's a draw or a mat
    pub fn check_endgame(&mut self) {
        self.board
            .update_threatens_cells();
        self.board
            .update_legals_moves();
        //if there is no legal moves : it's a endgame
        //  if the king is threaten : its a mat
        //  else its a pat
        if self.board.legals_moves.is_empty() {
            self.board.print(); //souvenir of the cli version ..
            let king_cell = self.board.get_king(&self.board.active_player);
            if let Some(coord) = king_cell {
                if self.board.threaten_cells.contains(&coord) {
                    self.board.end = Some(Checkmate);
                    self.timer.active = false;
                    self.app_mode = Versus(Some(Checkmate));
                } else {
                    self.board.end = Some(Pat);
                    self.app_mode = Versus(Some(Pat));
                    self.timer.active = false;
                }
            }
        }
    }

    fn events_check(&mut self) {
        self.board.promote_pawn();
        self.board.switch_players_color();
        self.check_endgame();
        // println!("{:?} to move", self.board.active_player);

        if let Some(k) = self.board.get_king(&self.board.active_player)
            && self.board.threaten_cells.contains(&k)
            && let Some(k) = self.board.get_king(&self.board.active_player)
        {
            self.board.check = Some(k);
            // println!("Check !");
        }
    }

}
