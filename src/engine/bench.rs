use crate::Board;
use crate::board::cell::Color;
use crate::engine::minimax::find_best_move;
use crate::engine::search_context::SearchContext;
use crate::engine::search_stats::MAX_SEARCH_DEPTH;
use wasm_bindgen::prelude::wasm_bindgen;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE_FEN: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

// --- Timer ---

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        * 1000.0
}

// --- BenchResult ---

pub struct BenchResult {
    pub nodes: u64,
    pub time_ms: f64,
    pub cutoffs: usize,
    pub leafs: usize,
    pub tt_hits: usize,
    pub quiescence_nodes: u64,
    pub aborted: bool,
    pub nodes_per_depth: [usize; MAX_SEARCH_DEPTH],
    pub cutoffs_per_depth: [usize; MAX_SEARCH_DEPTH],
}

// --- Core bench ---

// max_nodes = 0 means no limit.
// depth >= 1: depth=0 would underflow u8 in find_best_move.
pub fn bench_run(fen: &str, color: Color, depth: u8, max_nodes: u64) -> BenchResult {
    assert!(depth >= 1, "bench_run requires depth >= 1");
    let mut board = Board::board_from_fen(fen).board;
    let mut ctx = SearchContext::new();
    ctx.stats.max_nodes = max_nodes;
    let t0 = now_ms();
    find_best_move(&mut board, color, depth, &mut ctx);
    let time_ms = now_ms() - t0;
    BenchResult {
        nodes: ctx.stats.nodes,
        time_ms,
        cutoffs: ctx.stats.cutoffs,
        leafs: ctx.stats.leafs,
        tt_hits: ctx.stats.tt_hits,
        quiescence_nodes: ctx.stats.quiescence_nodes,
        aborted: ctx.stats.aborted,
        nodes_per_depth: ctx.stats.nodes_per_depth,
        cutoffs_per_depth: ctx.stats.cutoffs_per_depth,
    }
}

// --- Derived metrics ---

fn nps(nodes: u64, time_ms: f64) -> u64 {
    if time_ms > 0.0 {
        (nodes as f64 / (time_ms / 1000.0)) as u64
    } else {
        0
    }
}

fn cut_pct(cutoffs: usize, nodes: u64, leafs: usize) -> f64 {
    let interior = nodes.saturating_sub(leafs as u64);
    if interior == 0 {
        0.0
    } else {
        cutoffs as f64 / interior as f64 * 100.0
    }
}

fn tt_pct(tt_hits: usize, nodes: u64) -> f64 {
    if nodes == 0 {
        0.0
    } else {
        tt_hits as f64 / nodes as f64 * 100.0
    }
}

fn ebf(nodes: u64, depth: u8) -> f64 {
    if nodes == 0 || depth == 0 {
        0.0
    } else {
        (nodes as f64).powf(1.0 / depth as f64)
    }
}

fn entry_json(label: &str, depth: u8, r: &BenchResult, time_ms: f64) -> String {
    format!(
        r#"{{"label":"{}","depth":{},"nodes":{},"q_nodes":{},"time_ms":{:.1},"nps":{},"tt_pct":{:.1},"cut_pct":{:.1},"ebf":{:.2},"aborted":{}}}"#,
        label,
        depth,
        r.nodes,
        r.quiescence_nodes,
        time_ms,
        nps(r.nodes, time_ms),
        tt_pct(r.tt_hits, r.nodes),
        cut_pct(r.cutoffs, r.nodes, r.leafs),
        ebf(r.nodes, depth),
        r.aborted,
    )
}

// --- WASM export ---

// Called by bench.html once per depth (1..=max_depth).
// max_nodes = 0 means no limit; values like 500_000 prevent browser freeze.
// Runs 2 unmeasured warmup iterations then 5 measured; reports the minimum time.
#[wasm_bindgen]
pub fn run_bench(depth: u8, max_nodes: u32) -> String {
    if depth < 1 {
        return "[]".to_string();
    }
    let limit = max_nodes as u64;
    let (r1, t1) = run_n(START_FEN, Color::White, depth, limit);
    let (r2, t2) = run_n(KIWIPETE_FEN, Color::White, depth, limit);
    format!(
        "[{},{}]",
        entry_json("Start", depth, &r1, t1),
        entry_json("Kiwipete", depth, &r2, t2),
    )
}

fn run_n(fen: &str, color: Color, depth: u8, max_nodes: u64) -> (BenchResult, f64) {
    for _ in 0..2 {
        bench_run(fen, color, depth, max_nodes);
    }
    let first = bench_run(fen, color, depth, max_nodes);
    let mut min = first.time_ms;
    for _ in 1..5 {
        let r = bench_run(fen, color, depth, max_nodes);
        if r.time_ms < min {
            min = r.time_ms;
        }
    }
    (first, min)
}
