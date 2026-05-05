use crate::Board;
use crate::board::fen::FenInfo;
use crate::engine::minimax::{find_best_move, iterative_deepening};
use crate::engine::search_context::{SearchContext, SearchParams};
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub const QUICK_POSITIONS: &[(&str, &str)] = &[
    (
        "Start",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    ),
    (
        "Kiwipete",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    ),
];

pub const FULL_POSITIONS: &[(&str, &str)] = &[
    (
        "Start",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    ),
    (
        "Kiwipete",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    ),
    (
        "KingAttack",
        "4rrk1/pp1n3p/3q2pQ/2p1pb2/2PP4/2P3N1/P2B2PP/4RRK1 b - - 7 19",
    ),
    ("PawnEnding", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 11"),
    (
        "RookQueen",
        "3q2k1/pb3p1p/4pbp1/2r5/PpN2N2/1P2P2P/5PP1/Q2R2K1 b - - 4 26",
    ),
    (
        "RookPawns",
        "8/pp2r1k1/2p1p3/3pP2p/1P1P1P1P/P5KR/8/8 w - - 0 1",
    ),
    ("BishopEnding", "8/3p3B/5p2/5P2/p7/PP5b/k7/6K1 w - - 0 1"),
];

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

pub struct BenchResult {
    pub nodes: u64,
    pub time_ms: f64,
    pub cutoffs: usize,
    pub leafs: usize,
    pub tt_hits: usize,
    pub quiescence_nodes: u64,
    pub aborted: bool,
}

// max_nodes = 0 means no limit.
// Pre-warms the TT via ID up to depth-1 (unmeasured), then times only the final depth.
// This lets TT best_move, killers and history show their real impact without inflating the total time.
pub fn bench_run(fen: &str, depth: u8, max_nodes: u64) -> BenchResult {
    assert!(depth >= 1, "bench_run requires depth >= 1");
    let FenInfo {
        mut board,
        active_color,
        ..
    } = Board::board_from_fen(fen);
    let mut ctx = SearchContext::new();
    let hashmap: HashMap<u64, usize> = HashMap::new();

    if depth > 1 {
        let mut params = SearchParams::new(&mut ctx, &hashmap, 0);
        iterative_deepening(
            &mut board,
            active_color,
            depth - 1,
            &mut 0,
            0.0,
            &mut params,
        );
    }
    ctx.stats.reset();
    ctx.stats.max_nodes = max_nodes;
    let t0 = now_ms();
    {
        let mut params = SearchParams::new(&mut ctx, &hashmap, 0);
        find_best_move(
            &mut board,
            active_color,
            depth,
            i32::MIN,
            i32::MAX,
            &mut params,
        );
    }
    let time_ms = now_ms() - t0;
    BenchResult {
        nodes: ctx.stats.nodes,
        time_ms,
        cutoffs: ctx.stats.cutoffs,
        leafs: ctx.stats.leafs,
        tt_hits: ctx.stats.tt_hits,
        quiescence_nodes: ctx.stats.quiescence_nodes,
        aborted: ctx.stats.aborted,
    }
}

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

pub fn entry_json(label: &str, depth: u8, r: &BenchResult, time_ms: f64) -> String {
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run_bench_single(fen: &str, label: &str, depth: u8) -> String {
    if depth < 1 {
        return "{}".to_string();
    }
    let r = bench_run(fen, depth, 0);
    entry_json(label, depth, &r, r.time_ms)
}

pub fn run_bench(depth: u8, mode: u8) -> String {
    if depth < 1 {
        return "[]".to_string();
    }
    let positions = if mode == 0 {
        QUICK_POSITIONS
    } else {
        FULL_POSITIONS
    };
    let entries: Vec<String> = positions
        .iter()
        .map(|(label, fen)| {
            let r = bench_run(fen, depth, 0);
            entry_json(label, depth, &r, r.time_ms)
        })
        .collect();
    format!("[{}]", entries.join(","))
}
