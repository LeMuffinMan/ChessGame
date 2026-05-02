use chess_game::engine::bench::{FULL_POSITIONS, entry_json, run_n};

fn main() {
    let max_depth: u8 = std::env::args()
        .nth(1)
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
            let (r, t) = run_n(fen, depth, 0);

            let nps = if t > 0.0 {
                (r.nodes as f64 / (t / 1000.0)) as u64
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
                t,
                fmt_num(nps),
                tt_pct,
                cut_pct,
                ebf,
                if r.aborted { " [aborted]" } else { "" }
            );

            json_entries.push(entry_json(label, depth, &r, t));
        }
    }

    println!("[{}]", json_entries.join(",\n "));
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
