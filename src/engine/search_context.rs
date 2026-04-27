use crate::board::moves::move_structs::Move;
use crate::engine::search_stats::MAX_SEARCH_DEPTH;
use crate::engine::search_stats::SearchStats;
use crate::engine::ttentry::TtEntry;
use std::collections::HashMap;

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

    pub fn incremente_node(&mut self) {
        self.stats.nodes_per_depth[self.stats.depth] += 1;
        self.stats.total_node_depth += self.stats.depth;
        self.stats.nodes += 1;
    }
}

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
        //pourquoi saturating add ?
        self.table[from][to] = self.table[from][to].saturating_add((depth as u32) * (depth as u32));
    }

    pub fn get(&self, from: usize, to: usize) -> u32 {
        self.table[from][to]
    }
}
