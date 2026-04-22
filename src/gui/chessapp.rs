use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::board::move_gen::Move;
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
            stats: SearchStats {
                nodes: 0,
                bot_time_thinking: 0.0,
                cutoffs: 0,
                nps: 0.0,
                killer_moves: [[None; 2]; 64],
            },
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
        if self.current.threaten_cells.is_empty() {
            self.update_threaten_cells()
        }
        if self.current.legals_moves.is_empty() {
            self.update_legals_moves();
        }

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
            ctx.request_repaint_after(std::time::Duration::from_millis(300));
            self.play_bot_turn();
        }
    }
}

impl ChessApp {
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
