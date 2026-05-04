use crate::board::moves::move_structs::Move;
use crate::engine::search_stats::MAX_SEARCH_DEPTH;
use crate::engine::search_stats::SearchStats;
use crate::engine::ttentry::TtEntry;
use std::collections::HashMap;

pub struct SearchParams<'a> {
    pub ctx: &'a mut SearchContext,
    pub game_history: &'a HashMap<u64, usize>,
    pub fifty_count: u32,
    pub null_move_allowed: bool,
}

impl<'a> SearchParams<'a> {
    pub fn new(
        ctx: &'a mut SearchContext,
        game_history: &'a HashMap<u64, usize>,
        fifty_count: u32,
    ) -> Self {
        Self {
            ctx,
            game_history,
            fifty_count,
            null_move_allowed: true,
        }
    }
}

pub const TT_SIZE: usize = 1 << 20; // 1 M entrées ≈ 24 MB

#[derive(Clone)]
pub struct SearchContext {
    pub killers: KillerTable,
    pub history: HistoryTable,
    pub tt: Vec<TtEntry>,
    pub tt_generation: u8,
    pub stats: SearchStats,
}

impl Default for SearchContext {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchContext {
    pub fn new() -> Self {
        Self {
            killers: KillerTable::new(),
            history: HistoryTable::new(),
            tt: vec![TtEntry::default(); TT_SIZE],
            tt_generation: 0,
            stats: SearchStats::new(),
        }
    }

    pub fn reset_for_new_game(&mut self) {
        self.reset_game_context();
        self.stats = SearchStats::new();
    }

    // TT are preserved between games, but we reinit game related infos (killers, history)
    // generation TT is incremented for fifity_count and game_history
    pub fn reset_game_context(&mut self) {
        self.killers = KillerTable::new();
        self.history = HistoryTable::new();
        self.tt_generation = self.tt_generation.wrapping_add(1);
    }

    pub fn reset_search_stats(&mut self) {
        self.killers = KillerTable::new();
        self.stats.reset();
    }

    pub fn incremente_node(&mut self) {
        self.stats.nodes_per_depth[self.stats.depth] += 1;
        self.stats.total_node_depth += self.stats.depth;
        self.stats.nodes += 1;
    }
}

#[derive(Clone)]
pub struct KillerTable {
    moves: [[Option<Move>; 2]; MAX_SEARCH_DEPTH],
}

impl Default for KillerTable {
    fn default() -> Self {
        Self::new()
    }
}

impl KillerTable {
    pub fn new() -> Self {
        Self {
            moves: [[None; 2]; MAX_SEARCH_DEPTH],
        }
    }

    pub fn update(&mut self, depth: usize, mv: Move) {
        let d = depth.min(MAX_SEARCH_DEPTH - 1);
        if self.moves[d][0] != Some(mv) {
            self.moves[d][1] = self.moves[d][0];
            self.moves[d][0] = Some(mv);
        }
    }

    pub fn get(&self, depth: usize) -> [Option<Move>; 2] {
        let d = depth.min(MAX_SEARCH_DEPTH - 1);
        [self.moves[d][0], self.moves[d][1]]
    }
}

#[derive(Clone)]
pub struct HistoryTable {
    table: [[u32; 64]; 64],
}

impl Default for HistoryTable {
    fn default() -> Self {
        Self::new()
    }
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
