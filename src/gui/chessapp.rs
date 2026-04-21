use crate::Board;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::board::move_gen::Move;
use crate::board::move_gen::MoveType::*;
use crate::engine::minimax::get_bot_move;
use crate::gui::appmode::AppMode;
use crate::gui::appmode::AppMode::*;
use crate::gui::bot_difficulty::BotDifficulty::*;
use crate::gui::end::End;
use crate::gui::end::End::*;
use crate::gui::gamestate::DrawOption::Available;
use crate::gui::gamestate::DrawRule::FiftyMoves;
use crate::gui::gamestate::GameState;
use crate::gui::history::History;
use crate::gui::hooks::WinDia;
use crate::gui::player_type::PlayerType::*;
use crate::gui::promote_info::PromoteInfo;
use crate::gui::replay::ReplayInfos;
use crate::gui::settings::Settings;
use crate::gui::ui_type::UiType;
use crate::gui::update_timer::GameMode;
use crate::gui::update_timer::Timer;
use eframe::{App, egui};

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
    pub bot_pending: bool,
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
            bot_pending: false,
        }
    }
}

//This App trait runs the eframe : fn update is the main loop, run for each frame
impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.current.threaten_cells.is_empty() {
            self.update_threaten_cells()
        }
        if self.current.legals_moves.is_empty() {
            self.update_legals_moves();
        }

        self.hooks(ctx);
        if self.bot_pending && self.current.end.is_none() {
            self.bot_pending = false;
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
            self.play_bot_turn();
        }
        match &self.ui_type {
            UiType::Mobile => {
                self.mobile_layout(ctx);
            }
            UiType::Desktop => {
                self.desktop_layout(ctx);
            }
        }
    }
}

impl ChessApp {
    pub fn mobile_layout(&mut self, ctx: &egui::Context) {
        self.apply_styles(ctx);
        self.top_title_panel(ctx);
        self.central_panel_mobile(ctx);
    }
    pub fn desktop_layout(&mut self, ctx: &egui::Context) {
        self.apply_desktop_styles(ctx);
        self.top_title_panel(ctx);
        self.bot_source_code_panel_desktop(ctx);
        self.left_panel_desktop(ctx);
        self.right_panel_desktop(ctx);
        self.top_black_panel_desktop(ctx);
        self.bot_white_panel_desktop(ctx);
        self.central_panel_desktop(ctx);
    }
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

    pub fn is_bot_turn(&self) -> bool {
        match self.current.active_player {
            White => matches!(self.settings.white_bot, Bot(_)),
            Black => matches!(self.settings.black_bot, Bot(_)),
        }
    }

    pub fn start_bot_game(&mut self) {
        let snapshot = self.current.clone();
        self.history.snapshots.push(snapshot);
        self.replay_infos.index += 1;
        self.app_mode = Versus(None);
        self.timer.active = true;
        self.timer.start_of_turn.1 = Some(White);
        self.bot_pending = true;
    }

    pub fn play_bot_turn(&mut self) {
        let difficulty = match self.current.active_player {
            White => &self.settings.white_bot,
            Black => &self.settings.black_bot,
        };
        if let Some(m) = get_bot_move(
            difficulty,
            &mut self.current.board,
            self.current.active_player,
        ) {
            match difficulty {
                Bot(Easy) => {
                    let snapshot = self.current.clone();
                    self.apply_move(&m);
                    self.commit_move(snapshot, m, m.origin, m.dest);
                    if let Promotion(piece) = m.move_type {
                        self.current.board.grid[m.dest.row as usize][m.dest.col as usize] =
                            Cell::Occupied(piece, self.current.active_player);
                    }
                    self.switch_turn();
                    if self.current.end.is_none() && self.is_bot_turn() {
                        self.bot_pending = true;
                    }
                }
                Bot(Medium) | Bot(Hard) => {
                    self.try_move(m.origin, m.dest);
                }
                _ => {
                    unreachable!()
                }
            }
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
    pub fn fifty_moves_draw_check(&mut self, m: &Move) {
        //if a pawn moved, the counter reset
        if let Some(p) = self.current.board.get(&m.dest).get_piece()
            && p == &Pawn
        {
            self.current.draw.draw_moves_count = 0;
            return;
        }
        if m.capture != Cell::Free {
            self.current.draw.draw_moves_count = 0;
            return;
        }
        self.current.draw.draw_moves_count += 1;
        if self.current.draw.draw_moves_count >= 50 {
            if self.is_bot_turn() {
                self.current.end = Some(Draw);
                self.current.draw.draw_option = None;
            } else {
                self.current.draw.draw_option = Some(Available(FiftyMoves));
            }
        } else {
            self.current.draw.draw_option = None;
        }
    }
}
