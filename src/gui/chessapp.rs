use crate::gui::types::AppMode::*;
use crate::gui::game_state_struct::GameState;
use crate::gui::hooks::WinDia;
use crate::gui::replay::ReplayInfos;
use crate::gui::update_timer::GameMode;
use crate::gui::update_timer::Timer;
use crate::gui::types::AppMode;
use crate::gui::types::End;
use crate::UiType;
use crate::gui::components::Settings;
use crate::gui::history::History;
use crate::gui::components::PromoteInfo;

use eframe::App;

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
            && self.current.board.pawn_to_promote.is_some()
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
}
