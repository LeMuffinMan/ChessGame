use crate::board::move_gen::Move;

pub struct SearchStats {
    pub nodes: u64,
    pub bot_time_thinking: f64,
    //alpha beta pruning
    pub cutoffs: usize,
    pub nps: f64,
    //non capture move which already gave a cutoff
    pub killer_moves: [[Option<Move>; 2]; 64],
}

impl SearchStats {
    pub fn nps(&mut self) {
        self.nps = if self.bot_time_thinking == 0.0 {
            0.0
        } else {
            self.nodes as f64 / (self.bot_time_thinking / 1000.0)
        };
    }
    pub fn format_time(ms: f64) -> String {
        if ms < 1.0 {
            format!("{:.3} ms", ms)
        } else if ms < 1000.0 {
            format!("{:.1} ms", ms)
        } else {
            format!("{:.2} s", ms / 1000.0)
        }
    }
}
