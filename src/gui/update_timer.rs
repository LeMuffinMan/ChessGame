use crate::Color;
use crate::Color::*;

#[derive(PartialEq)]
pub struct Timer {
    pub start: f64,
    pub increment: f64,
    pub active: bool,
    pub mode: GameMode,
    pub white_time: f64,
    pub black_time: f64,
    pub start_of_turn: (f64, Option<Color>),
}

#[derive(PartialEq)]
pub enum GameMode {
    Rapid,
    Blitz,
    Bullet,
    Custom,
    NoTime,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: 0.0,
            increment: 0.0,
            active: false,
            mode: GameMode::NoTime,
            white_time: 0.0,
            black_time: 0.0,
            start_of_turn: (0.0, None),
        }
    }

    pub fn update_timer(&mut self, ctx: &egui::Context, active_player: &Color) -> bool {
        let now = ctx.input(|i| i.time);

        if self.start_of_turn.1.is_none() {
            self.init_timer(now, active_player);
        }

        if self.start_of_turn.1 != Some(*active_player) {
            self.switch_timer(now, active_player);
        }

        return self.decrement_timer(now, active_player);
    }

    pub fn init_timer(&mut self, now: f64, active_player: &Color) {
        self.start_of_turn.1 = Some(*active_player);
        self.start_of_turn.0 = now;
    }

    pub fn switch_timer(&mut self, now: f64, active_player: &Color) {
        match self.start_of_turn.1 {
            Some(White) => self.white_time += self.increment,
            Some(Black) => self.black_time += self.increment,
            None => {}
        }
        self.start_of_turn.1 = Some(*active_player);
        self.start_of_turn.0 = now;
    }

    pub fn decrement_timer(&mut self, now: f64, active_player: &Color) -> bool {
        let delta = now - self.start_of_turn.0;
        self.start_of_turn.0 = now; // reset pour le prochain tick

        match active_player {
            White => {
                self.white_time -= delta;
                if self.white_time <= 0.0 {
                    self.white_time = 0.0;
                    return true;
                }
            }
            Black => {
                self.black_time -= delta;
                if self.black_time <= 0.0 {
                    self.black_time = 0.0;
                    return true;
                }
            }
        }
        false
    }
}
