use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::board::moves::move_structs::Move;
// use crate::engine::evaluator::Evaluator;
// use crate::engine::evaluator::PositionalEvaluator;
// use crate::engine::minimax::get_bot_move;
// use crate::engine::minimax::{HARD_DEPTH, MEDIUM_DEPTH};
use crate::engine::search_stats::SearchStats;
use crate::gui::chessapp::AppMode::*;
use crate::gui::features::gamestate::DrawOption::Available;
use crate::gui::features::gamestate::DrawRule::FiftyMoves;
use crate::gui::features::gamestate::GameState;
use crate::gui::features::history::History;
use crate::gui::features::replay::ReplayInfos;
use crate::gui::features::settings::Settings;
use crate::gui::features::timer::GameMode;
use crate::gui::features::timer::Timer;
use crate::gui::hooks::promote::PromoteInfo;
use crate::gui::hooks::windows::End;
use crate::gui::hooks::windows::End::*;
use crate::gui::hooks::windows::WinDia;
use crate::gui::layout::UiType;
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
    pub stats: SearchStats,
    pub white_last_score: i32,
    pub black_last_score: i32,
}

impl ChessApp {
    pub fn new(ui_type: UiType) -> Self {
        Self {
            ui_type,
            history: History::new(),
            timer: Timer::new(0.0, 0.0, GameMode::NoTime),
            win: None,
            app_mode: Lobby,
            replay_infos: ReplayInfos::new(),
            current: GameState::new(),
            settings: Settings::new(),
            promoteinfo: None,
            bot_pending: false,
            stats: SearchStats::new(),
            white_last_score: 0,
            black_last_score: 0,
        }
    }
}

#[derive(PartialEq)]
pub enum AppMode {
    Versus(Option<End>),
    Replay,
    Lobby,
}

//This App trait runs the eframe : fn update is the main loop, run for each frame
impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.hooks(ctx);
        match &self.ui_type {
            UiType::Mobile => {
                self.mobile_layout(ctx);
            }
            UiType::Desktop => {
                self.desktop_layout(ctx);
            }
        }
        if self.bot_pending
            && self.current.end.is_none()
            && self.app_mode != Replay
            && self.app_mode != Lobby
            && self.win.is_none()
        {
            self.bot_pending = false;
            // ctx.request_repaint_after(std::time::Duration::from_millis(300));
            ctx.request_repaint();
            self.play_bot_turn();
            if self.current.draw.draw_option.is_some() {
                self.current.end = Some(Draw);
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

    pub fn fifty_moves_draw_check(&mut self, m: &Move) {
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
