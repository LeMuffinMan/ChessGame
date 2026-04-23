use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::board::move_gen::Move;
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
    // pub fn get_depth(&self) -> u8 {
    //     if let Bot(diff) = match self.current.active_player {
    //         White => self.settings.black_bot,
    //         Black => self.settings.white_bot,
    //     } {
    //         match diff {
    //             Easy => 0,
    //             Medium => MEDIUM_DEPTH,
    //             Hard => HARD_DEPTH,
    //         }
    //     } else {
    //         0
    //     }
    // }
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

    // pub fn is_bot_turn(&self) -> bool {
    //     match self.current.active_player {
    //         White => matches!(self.settings.white_bot, Bot(_)),
    //         Black => matches!(self.settings.black_bot, Bot(_)),
    //     }
    // }

    // pub fn start_bot_game(&mut self) {
    //     let snapshot = self.current.clone();
    //     self.history.snapshots.push(snapshot);
    //     self.replay_infos.index += 1;
    //     self.app_mode = Versus(None);
    //     self.timer.active = true;
    //     self.timer.start_of_turn.1 = Some(White);
    //     self.bot_pending = true;
    // }

    // pub fn play_bot_turn(&mut self) {
    //     let difficulty = match self.current.active_player {
    //         White => &self.settings.white_bot,
    //         Black => &self.settings.black_bot,
    //     };
    //     let performance = window().unwrap().performance().unwrap();
    //     self.stats.nodes = 0;
    //     self.stats.cutoffs = 0;
    //     self.stats.killer_moves = [[None; 2]; 64];
    //     let start = performance.now();
    //     let bot_move = get_bot_move(
    //         difficulty,
    //         &mut self.current.board,
    //         self.current.active_player,
    //         &mut self.stats,
    //     );
    //     let end = performance.now();
    //     self.stats.bot_time_thinking = end - start;
    //     self.stats.nps();
    //     if let Some(m) = bot_move {
    //         match difficulty {
    //             Bot(Easy) => {
    //                 let snapshot = self.current.clone();
    //                 self.apply_move(&m);
    //                 self.commit_move(snapshot, m, m.origin, m.dest);
    //                 if let Promotion(piece) = m.move_type {
    //                     self.current.board.grid[m.dest.row as usize][m.dest.col as usize] =
    //                         Cell::Occupied(piece, self.current.active_player);
    //                 }
    //                 self.switch_turn();
    //                 if self.current.end.is_none() && self.is_bot_turn() {
    //                     self.bot_pending = true;
    //                 }
    //             }
    //             Bot(Medium) | Bot(Hard) => {
    //                 let bot_color = self.current.active_player;
    //                 self.try_move(m.origin, m.dest);
    //                 if let Promotion(piece) = m.move_type {
    //                     self.current.board.grid[m.dest.row as usize][m.dest.col as usize] =
    //                         Cell::Occupied(piece, bot_color);
    //                     self.promoteinfo = None;
    //                     self.win = None;
    //                     if self.current.end.is_none() && self.is_bot_turn() {
    //                         self.bot_pending = true;
    //                     }
    //                 }
    //             }
    //             _ => {
    //                 unreachable!()
    //             }
    //         }
    //     }
    // }

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
