use crate::ChessApp;
use crate::Color::*;
use crate::board::cell::Cell;
use crate::gui::chessapp_struct::End::TimeOut;
use crate::gui::chessapp_struct::GameMode;

impl ChessApp {
    //Hook to keep update the replay in real time
    pub fn replay_step(&mut self, ctx: &egui::Context) {
        if let Some(next_time) = self.widgets.next_replay_time {
            let now = ctx.input(|i| i.time);
            if now >= next_time {
                if self.widgets.replay_index + 1 < self.history.len() {
                    self.widgets.replay_index += 1;
                    // log::debug!("Replay index = {}", self.widgets.replay_index);
                    self.current = self.history[self.widgets.replay_index].clone();
                    let delay = self.widgets.replay_speed;
                    self.widgets.next_replay_time = Some(now + delay);
                } else {
                    self.widgets.replay_index = self.history.len();
                    self.widgets.next_replay_time = None;
                }
            }
        }
        ctx.request_repaint();
    }

    //Hook to keep timers update or start it if needed
    pub fn update_timer(&mut self, ctx: &egui::Context) {
        let now = ctx.input(|i| i.time);

        //Switching timers for each turn
        if let Some(timer) = &mut self.widgets.timer {
            if timer.white.0.is_none()
                && self.current.active_player == White
                && !self.history.is_empty()
            {
                timer.white.0 = Some(now);
                if let Some(black_start) = timer.black.0 {
                    timer.black.1 += timer.increment;
                    timer.black.1 -= now - black_start;
                }
                timer.black.0 = None;
                //start the timer at the beginning
            } else if timer.black.0.is_none() && self.current.active_player == Black {
                if self.history.len() == 2 {
                    #[allow(clippy::collapsible_if)]
                    if let Some(game_mode) = &self.widgets.game_mode {
                        match game_mode {
                            GameMode::Bullet(max_time, inc)
                            | GameMode::Blitz(max_time, inc)
                            | GameMode::Rapid(max_time, inc)
                            | GameMode::Custom(max_time, inc) => {
                                timer.white.1 = *max_time;
                                timer.black.1 = *max_time;
                                timer.increment = *inc;
                            }
                        }
                    }
                }
                timer.black.0 = Some(now);
                if let Some(white_start) = timer.white.0 {
                    timer.white.1 += timer.increment;
                    timer.white.1 -= now - white_start;
                }
                timer.white.0 = None;
            }
            if timer.white.1 < 0.0 {
                timer.white.1 = 0.0;
                self.current.end = Some(TimeOut);
            }
            if timer.black.1 < 0.0 {
                timer.black.1 = 0.0;
                self.current.end = Some(TimeOut);
            }
        }
    }

    //if a player promoted a pawn, try_move didnt finished it's work, so we do it here
    pub fn update_promote(&mut self) {
        if let Some(piece) = self.current.board.promote
            && let Some(coord) = self.current.board.pawn_to_promote
            && self.widgets.replay_index == self.history.len()
        {
            //methods get opponent color
            let color = if self.current.active_player == White {
                Black
            } else {
                White
            };
            self.current.board.grid[coord.row as usize][coord.col as usize] =
                Cell::Occupied(piece, color);

            //methods
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
}
