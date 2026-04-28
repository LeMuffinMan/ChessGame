use crate::Coord;
use crate::engine::search_context::SearchContext;
use crate::game::End;
use crate::game::End::*;
use crate::game::Game;
use crate::gui::chessapp::AppMode::*;
use crate::gui::features::replay::ReplayInfos;
use crate::gui::features::settings::Settings;
use crate::gui::features::timer::GameMode;
use crate::gui::features::timer::Timer;
use crate::gui::hooks::promote::PromoteInfo;
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
    pub game: Game,
    pub last_move: Option<(Coord, Coord)>,
    pub history_san: String,
    pub bot_pending: bool,
    pub search_ctx: SearchContext,
    pub white_last_score: i32,
    pub black_last_score: i32,
}

impl ChessApp {
    pub fn new(ui_type: UiType) -> Self {
        Self {
            ui_type,
            history_san: String::new(),
            timer: Timer::new(0.0, 0.0, GameMode::NoTime),
            win: None,
            app_mode: Lobby,
            replay_infos: ReplayInfos::new(),
            game: Game::new(),
            last_move: None,
            settings: Settings::new(),
            promoteinfo: None,
            bot_pending: false,
            search_ctx: SearchContext::new(),
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
            && self.game.end.is_none()
            && self.app_mode != Replay
            && self.app_mode != Lobby
            && self.win.is_none()
        {
            self.bot_pending = false;
            ctx.request_repaint();
            self.play_bot_turn();
            if self.game.draw.draw_option.is_some() {
                self.game.end = Some(Draw);
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
            if self.timer.update_timer(ctx, &self.game.active_player) {
                self.game.end = Some(End::TimeOut);
            }
            ctx.request_repaint();
        }
        if matches!(self.app_mode, AppMode::Versus(_))
            && self.replay_infos.index == self.game.history.len()
            && self.promoteinfo.is_some()
        {
            self.get_promotion_input(ctx);
        }
    }
}
