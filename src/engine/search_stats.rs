use crate::board::move_gen::Move;

pub const MAX_SEARCH_DEPTH: usize = 16;

pub struct SearchStats {
    pub depth: usize,
    pub nodes: u64,
    pub bot_time_thinking: f64,
    pub cutoffs: usize,
    pub nps: f64,
    pub killer_moves: [[Option<Move>; 2]; MAX_SEARCH_DEPTH],
    pub leafs: usize,
    pub cutoffs_per_depth: [usize; MAX_SEARCH_DEPTH],
    pub nodes_per_depth: [usize; MAX_SEARCH_DEPTH],
    pub total_node_depth: usize,
    pub total_cutoffs_depth: usize,
    pub max_nodes: u64,
    pub aborted: bool,
}

impl SearchStats {
    pub fn new() -> Self {
        Self {
            depth: 0,
            nodes: 0,
            bot_time_thinking: 0.0,
            cutoffs: 0,
            nps: 0.0,
            killer_moves: [[None; 2]; MAX_SEARCH_DEPTH],
            leafs: 0,
            cutoffs_per_depth: [0; MAX_SEARCH_DEPTH],
            nodes_per_depth: [0; MAX_SEARCH_DEPTH],
            total_node_depth: 0,
            total_cutoffs_depth: 0,
            max_nodes: 0,
            aborted: false,
        }
    }

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
