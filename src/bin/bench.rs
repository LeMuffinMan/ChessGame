use chess_game::engine::bench::{FULL_POSITIONS, bench_run, entry_json};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;

const DEFAULT_THRESHOLD: u8 = 10;

#[derive(Deserialize)]
struct BenchResult {
    label: String,
    depth: u8,
    nodes: u64,
    q_nodes: u64,
    aborted: bool,
}

fn main() {
    if let Some(mode) = std::env::args().nth(1) {
        match mode.as_str() {
            "measure" => {
                let max_depth: u8 = std::env::args()
                    .nth(2)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(8);

                let header = format!(
                    "{:<12} {:>3}  {:>10}  {:>10}  {:>8}  {:>10}  {:>5}  {:>5}  {:>5}",
                    "Pos", "D", "Nodes", "Q-Nodes", "ms", "NPS", "TT%", "Cut%", "EBF"
                );
                eprintln!("{}", header);
                eprintln!("{}", "-".repeat(header.len()));

                let mut json_entries: Vec<String> = Vec::new();

                for depth in 1..=max_depth {
                    for (label, fen) in FULL_POSITIONS {
                        let r = bench_run(fen, depth, 0);

                        let nps = if r.time_ms > 0.0 {
                            (r.nodes as f64 / (r.time_ms / 1000.0)) as u64
                        } else {
                            0
                        };
                        let cut_pct = {
                            let interior = r.nodes.saturating_sub(r.leafs as u64);
                            if interior == 0 {
                                0.0
                            } else {
                                r.cutoffs as f64 / interior as f64 * 100.0
                            }
                        };
                        let tt_pct = if r.nodes == 0 {
                            0.0
                        } else {
                            r.tt_hits as f64 / r.nodes as f64 * 100.0
                        };
                        let ebf = if r.nodes == 0 || depth == 0 {
                            0.0
                        } else {
                            (r.nodes as f64).powf(1.0 / depth as f64)
                        };

                        eprintln!(
                            "{:<12} {:>3}  {:>10}  {:>10}  {:>8.1}  {:>10}  {:>5.1}  {:>5.1}  {:>5.2}{}",
                            label,
                            depth,
                            fmt_num(r.nodes),
                            fmt_num(r.quiescence_nodes),
                            r.time_ms,
                            fmt_num(nps),
                            tt_pct,
                            cut_pct,
                            ebf,
                            if r.aborted { " [aborted]" } else { "" }
                        );

                        json_entries.push(entry_json(label, depth, &r, r.time_ms));
                    }
                }

                println!("[{}]", json_entries.join(",\n "));
            }
            "compare" => {
                let mut args = std::env::args().skip(2);

                let (baseline_filename, current_filename, threshold) =
                    match (args.next(), args.next(), args.next()) {
                        (Some(b), Some(c), Some(t)) => (b, c, t),
                        _ => {
                            eprintln!(
                                "Usage: bench compare <baseline.json> <current.json> <threshold_pct>"
                            );
                            std::process::exit(1);
                        }
                    };

                let threshold: f64 = threshold
                    .parse::<u8>()
                    .unwrap_or(DEFAULT_THRESHOLD) as f64;

                match (
                    get_file_content(&baseline_filename),
                    get_file_content(&current_filename),
                ) {
                    (Some(baseline), Some(current)) => {
                        if compare(&baseline, &current, threshold) {
                            std::process::exit(1);
                        }
                    }
                    _ => std::process::exit(1),
                }
            }
            _ => {
                eprintln!("Usage: bench measure <depth>");
                eprintln!("       bench compare <baseline.json> <current.json> <threshold_pct>");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Usage: bench measure <depth>");
        eprintln!("       bench compare <baseline.json> <current.json> <threshold_pct>");
    }
}

fn get_file_content(filename: &str) -> Option<Vec<BenchResult>> {
    match File::open(filename) {
        Ok(file) => match serde_json::from_reader(file) {
            Ok(res) => Some(res),
            Err(e) => {
                eprintln!("Error parsing {filename}: {e}");
                None
            }
        },
        Err(e) => {
            eprintln!("Error opening {filename}: {e}");
            None
        }
    }
}

fn compare(baseline: &[BenchResult], current: &[BenchResult], threshold: f64) -> bool {
    let current_map: HashMap<(&str, u8), &BenchResult> =
        current.iter().map(|r| ((r.label.as_str(), r.depth), r)).collect();

    let mut regression = false;

    eprintln!(
        "{:<12} {:>3}  {:>12}  {:>12}  {:>8}",
        "Position", "D", "Nodes base", "Nodes cur", "Nodes Δ%"
    );
    eprintln!("{}", "-".repeat(55));

    for entry in baseline {
        let Some(cur) = current_map.get(&(entry.label.as_str(), entry.depth)) else {
            eprintln!("WARN: {} d{}: missing in current — ignored", entry.label, entry.depth);
            continue;
        };

        if cur.aborted {
            eprintln!("WARN: {} d{}: aborted in current — ignored", entry.label, entry.depth);
            continue;
        }

        let base_total = entry.nodes + entry.q_nodes;
        let cur_total = cur.nodes + cur.q_nodes;

        let nodes_delta = if base_total > 0 {
            (cur_total as f64 - base_total as f64) / base_total as f64 * 100.0
        } else {
            0.0
        };

        let flag = if nodes_delta > threshold { " ❌" } else { "" };

        eprintln!(
            "{:<12} {:>3}  {:>12}  {:>12}  {:>+7.1}%{}",
            entry.label,
            entry.depth,
            fmt_num(base_total),
            fmt_num(cur_total),
            nodes_delta,
            flag,
        );

        if nodes_delta > threshold {
            regression = true;
        }
    }

    eprintln!("{}", "-".repeat(55));
    if regression {
        eprintln!("✗ Régression (seuil : {}%)", threshold);
    } else {
        eprintln!("✓ No regression (seuil : {}%)", threshold);
    }

    regression
}

fn fmt_num(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(' ');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}
