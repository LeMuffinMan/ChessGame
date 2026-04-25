use std::collections::HashMap;
use crate::board::moves::move_structs::Move;
use crate::engine::ttentry::TtEntry;

pub const MAX_SEARCH_DEPTH: usize = 16;

pub struct KillerTable {
    moves: [[Option<Move>; 2]; MAX_SEARCH_DEPTH],
}

impl KillerTable {
    pub fn new() -> Self {
        Self {
            moves: [[None; 2]; MAX_SEARCH_DEPTH],
        }
    }

    pub fn update(&mut self, depth: usize, mv: Move) {
        if self.moves[depth][0] != Some(mv) {
            self.moves[depth][1] = self.moves[depth][0];
            self.moves[depth][0] = Some(mv);
        }
    }

    pub fn get(&self, depth: usize) -> [Option<Move>; 2] {
        [self.moves[depth][0], self.moves[depth][1]]
    }
}

pub struct HistoryTable {
    table: [[u32; 64]; 64],
}

impl HistoryTable {
    pub fn new() -> Self {
        Self {
            table: [[0; 64]; 64],
        }
    }

    pub fn update(&mut self, from: usize, to: usize, depth: u8) {
        self.table[from][to] = self.table[from][to].saturating_add((depth as u32) * (depth as u32));
    }

    pub fn get(&self, from: usize, to: usize) -> u32 {
        self.table[from][to]
    }
}

pub struct SearchStats {
    pub depth: usize,
    pub nodes: u64,
    pub bot_time_thinking: f64,
    pub cutoffs: usize,
    pub nps: f64,
    pub leafs: usize,
    pub cutoffs_per_depth: [usize; MAX_SEARCH_DEPTH],
    pub nodes_per_depth: [usize; MAX_SEARCH_DEPTH],
    pub total_node_depth: usize,
    pub total_cutoffs_depth: usize,
    pub max_nodes: u64,
    pub aborted: bool,
    pub tt_hits: usize,
    pub tt_stores: usize,
}

impl SearchStats {
    pub fn new() -> Self {
        Self {
            depth: 0,
            nodes: 0,
            bot_time_thinking: 0.0,
            cutoffs: 0,
            nps: 0.0,
            leafs: 0,
            cutoffs_per_depth: [0; MAX_SEARCH_DEPTH],
            nodes_per_depth: [0; MAX_SEARCH_DEPTH],
            total_node_depth: 0,
            total_cutoffs_depth: 0,
            max_nodes: 0,
            aborted: false,
            tt_hits: 0,
            tt_stores: 0,
        }
    }

    pub fn nps(&mut self) {
        self.nps = if self.bot_time_thinking == 0.0 {
            0.0
        } else {
            self.nodes as f64 / (self.bot_time_thinking / 1000.0)
        };
    }

    pub fn reset(&mut self) {
        self.depth = 0;
        self.nodes = 0;
        self.cutoffs = 0;
        self.nps = 0.0;
        self.leafs = 0;
        self.cutoffs_per_depth = [0; MAX_SEARCH_DEPTH];
        self.nodes_per_depth = [0; MAX_SEARCH_DEPTH];
        self.total_node_depth = 0;
        self.total_cutoffs_depth = 0;
        self.aborted = false;
        self.tt_hits = 0;
        self.tt_stores = 0;
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

pub struct SearchContext {
    pub killers: KillerTable,
    pub history: HistoryTable,
    pub tt: HashMap<u64, TtEntry>,
    pub stats: SearchStats,
}

impl SearchContext {
    pub fn new() -> Self {
        Self {
            killers: KillerTable::new(),
            history: HistoryTable::new(),
            tt: HashMap::new(),
            stats: SearchStats::new(),
        }
    }

    pub fn reset_for_new_game(&mut self) {
        self.killers = KillerTable::new();
        self.history = HistoryTable::new();
        self.tt.clear();
        self.stats = SearchStats::new();
    }

    pub fn reset_stats(&mut self) {
        self.killers = KillerTable::new();
        self.stats.reset();
    }
}
