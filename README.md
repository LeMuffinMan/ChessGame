# ChessGame ♟️

[![CI](https://github.com/LeMuffinMan/ChessGame/actions/workflows/deploy.yml/badge.svg)](https://github.com/LeMuffinMan/ChessGame/actions/workflows/deploy.yml)
[![Live Demo](https://img.shields.io/badge/demo-live-brightgreen)](https://lemuffinman.github.io/ChessGame/)

**[▶ Play in browser](https://lemuffinman.github.io/ChessGame/)**

---

I built this project to learn Rust on something real and not exercises or tutorials. The rules are complex enough to punish bad design (and they did), the algorithms are well-documented,  the Chess Programming Wiki, which I discovered exists and is enormous, became my bible over the two intense sprints I spent on this project. Also, seeing how simple evaluation criteria (material value and position) can lead to natural openings and an already decent bot, before improving it until you get mated by the algorithm you built… that's quite exciting. I now aim to integrate UCI to measure its Elo against other engines.

Since I haven't yet implemented parallelism on WASM or threads on native, the goal was to push my bots to a decent difficulty level at the highest depth possible, trying to limit the time thinking to 300ms time thinking to prevent the ui freeze to be perceptible. Implementing a Transposition Table was probably the most valuable gain. But speed is not enough, the other major challenge was making the bot actually close out a won endgame. King activity changes completely in late game, and the evaluator needed to reflect that. The ladder mate logic might be where I'm spending too many resources now (following tests and benchmarks), but at least the bot doesn't miss an easy rook-and-king ladder mate.

The Rust was itself a journey. I realize now how much good data structures from the start pay off later. After almost 9,000 lines, I'm considering switching to bitboards for one of the lasts big performance gains still on the table. The path went from evaluating the board through a generic trait at every leaf, to maintaining an incremental score directly inside `apply` and `undo`, then fitting a 1M-entry transposition table into a fixed `Vec` with generation counters to isolate games. I was celebrating when my bot took 3 secondes to play at depth 5 in first day running my minimax without even alpha beta pruning for the first time. Now i aim to have less 300ms response in depth 12, and depth 5 takes less than 30ms to process.

---

## What it does

- **Play chess** — human vs human, human vs bot, or bot vs bot, with standard time controls (blitz, rapid…)
- **AI opponent** — alpha-beta engine at configurable depth, three difficulty levels with iterative deepening
- **Hint system** — ask for the engine's best move suggestion at any point
- **Replay & PGN** — move replay and PGN export
- **Bench page** — standalone `bench.html` comparing engine performance across positions and depths (native vs WASM too if running localy)
- **Portable** — WASM build runs in any browser on any device; native binary available for desktop and benchmarking
- **Responsive** — UI for desktop and mobile

<p align="center">
  <img src="assets/screenshot_demo1.png" width="750" alt="Desktop demo" />
  <img src="assets/mobile_demo.gif" width="225" alt="Mobile demo" />
</p>

---

## Under the hood

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the full module breakdown.

```
src/
├── board/       — board representation, move generation, legality, pin detection
├── engine/      — alpha-beta, evaluators, TT, Zobrist, move ordering
├── game/        — GameState (turn, castling rights, en passant, draw conditions)
├── gui/         — egui panels and components
└── bin/
    └── bench.rs — native benchmark binary
lib.rs           — WASM entry point
main.rs          — native entry point
```

The `engine/` module has zero dependency on `egui`. It allows us to run a separate benchmark binary with it, and open the path for UCI implementation.

---

## Algorithms

See [docs/ALGORITHMS.md](docs/ALGORITHMS.md) for context and implementation notes on each technique.

**Search**

| Technique | Reference |
|---|---|
| Alpha-Beta Pruning | [CPW](https://www.chessprogramming.org/Alpha-Beta) |
| Principal Variation Search (PVS) | [CPW](https://www.chessprogramming.org/Principal_Variation_Search) |
| Iterative Deepening | [CPW](https://www.chessprogramming.org/Iterative_Deepening) |
| Aspiration Windows | [CPW](https://www.chessprogramming.org/Aspiration_Windows) |
| Null Move Pruning | [CPW](https://www.chessprogramming.org/Null_Move_Pruning) |
| Late Move Reductions (LMR) | [CPW](https://www.chessprogramming.org/Late_Move_Reductions) |
| Futility Pruning | [CPW](https://www.chessprogramming.org/Futility_Pruning) |
| Check Extensions | [CPW](https://www.chessprogramming.org/Check_Extensions) |
| Quiescence Search | [CPW](https://www.chessprogramming.org/Quiescence_Search) |
| Delta Pruning (in quiescence) | [CPW](https://www.chessprogramming.org/Delta_Pruning) |

**Move ordering**

| Technique | Reference |
|---|---|
| MVV-LVA | [CPW](https://www.chessprogramming.org/MVV-LVA) |
| Killer Move Heuristic | [CPW](https://www.chessprogramming.org/Killer_Move) |
| History Heuristic | [CPW](https://www.chessprogramming.org/History_Heuristic) |
| TT move ordering | [CPW](https://www.chessprogramming.org/Transposition_Table) |

**Hashing & memory**

| Technique | Reference |
|---|---|
| Zobrist Hashing (incremental) | [CPW](https://www.chessprogramming.org/Zobrist_Hashing) |
| Transposition Table (fixed Vec 1M + generation counter) | [CPW](https://www.chessprogramming.org/Transposition_Table) |

**Evaluation**

| Technique | Reference |
|---|---|
| Piece-Square Tables (opening/endgame interpolated) | [CPW](https://www.chessprogramming.org/Piece-Square_Tables) |
| King Safety (pawn shield + attacker proximity) | [CPW](https://www.chessprogramming.org/King_Safety) |
| Pin Detection | [CPW](https://www.chessprogramming.org/Pin) |
| Mop-up Evaluation | [CPW](https://www.chessprogramming.org/Mop-up_Evaluation) |
| King Corner Pressure | [CPW](https://www.chessprogramming.org/King_Centralization) |
| Rook & Bishop Cut Bonus (ladder mate) | [CPW](https://www.chessprogramming.org/Rook_Endgames) |

---

## Benchmarks

The project includes a standalone benchmark page (`bench.html`) that runs the engine against standard positions at increasing depths and reports nodes per second, effective branching factor, and quiescence node ratio.

The native vs WASM comparison requires a local setup — run `just bench-all` after cloning to generate `public/native_bench.json` and open `bench.html` in your browser. This comparison is not available on the live demo.

> **Note on reliability:** comparing native and WASM numbers is only meaningful on the same machine. Even then, WASM performance is harder to measure accurately: the browser introduces scheduling noise, lacks SIMD optimizations, and runs single-threaded in a sandboxed environment. Treat WASM NPS as a relative indicator across depths or positions, not an absolute measure — and never compare it directly to native figures from a different machine.

<p align="center">
  <img src="assets/260502_20h39m56s_screenshot.png" width="750" alt="Bench depth 10 WASM - Native" />
</p>
---

## Running locally

### Prerequisites

```bash
# Rust (https://rustup.rs)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# WASM target
rustup target add wasm32-unknown-unknown

# Trunk (WASM bundler)
cargo install --locked trunk

# just (optional — recommended for convenience)
cargo install just
```

### Commands

`just` wraps the underlying `cargo` commands to handle the different compilation targets (WASM vs native, debug vs release) and pipelines multi-step workflows like generating `native_bench.json` before launching the bench viewer. You can always use raw `cargo` commands instead.

```bash
just          # list all available commands

just wasm     # trunk serve → http://127.0.0.1:8080 (WASM, hot-reload)
just native   # cargo run --features=native (debug)
just n        # cargo run --release --features=native

just test     # cargo test
just clippy   # clippy for both native and wasm32 targets

just bench-native   # cargo run --release --bin bench → public/native_bench.json
just bench-all      # bench-native (if missing) then just wasm → bench.html compares both
```

See [docs/JUSTFILE.md](docs/JUSTFILE.md) for the full command reference.

### Dependencies

| Tool | Install | Purpose |
|---|---|---|
| `rustup` | [rustup.rs](https://rustup.rs) | Rust toolchain |
| `wasm32-unknown-unknown` | `rustup target add wasm32-unknown-unknown` | WASM compilation target |
| `trunk` | `cargo install --locked trunk` | WASM bundler and dev server |
| `just` *(optional)* | `cargo install just` | Task runner — QoL wrapper around cargo |

Key Rust dependencies: `eframe` / `egui` (GUI), `wasm-bindgen` + `web-sys` (WASM bridge), `chrono` (timers).

---

## Roadmap

### Next steps
- Deployment on Cloudflare Pages if it enable as I hope WASM parallelism via WebWorkers. Or a VPS as fallback, but the simpliest serverless setup is preffered.
- Separate UI loop and engine search into dedicated workers — bot thinking time is currently capped to avoid blocking the UI
- PGN import (SAN decoder)
- Optimise more and more following performances measures :
  - SEE (Static Exchange Evaluation) — evaluate capture sequences before exploring, significant move ordering gain
  - Lazy sort — score moves on demand instead of full upfront sort
  - ...

### Backlog
- UCI protocol to plug my bot with tournament tools, Lichess bot and giving him a ELO
- Switch to use bitboard to get the last huge performance bump before :
- Native multithreading for deeper search and WASM parrallelism using WebWorkers.

---

## Resources

**Rust**
- [The Rust Book](https://doc.rust-lang.org/book/) — the reference
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — concise, hands-on
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — unsafe Rust and advanced internals

**WebAssembly**
- [Rust and WebAssembly book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen docs](https://rustwasm.github.io/docs/wasm-bindgen/)

**Chess programming**
- [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page) — the bible
- [Minimax](https://www.chessprogramming.org/Minimax) · [Alpha-Beta](https://www.chessprogramming.org/Alpha-Beta) · [Quiescence Search](https://www.chessprogramming.org/Quiescence_Search)
- [Transposition Table](https://www.chessprogramming.org/Transposition_Table) · [Zobrist Hashing](https://www.chessprogramming.org/Zobrist_Hashing)
- [rustic-chess — MVV-LVA](https://rustic-chess.org/search/ordering/mvv_lva.html) — practical move ordering walkthrough

**Tools**
- [egui](https://github.com/emilk/egui)
- [trunk](https://trunkrs.dev/)
- [just](https://github.com/casey/just)
