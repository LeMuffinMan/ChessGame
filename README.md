# ChessGame ♟️

[![CI](https://github.com/LeMuffinMan/ChessGame/actions/workflows/deploy.yml/badge.svg)](https://github.com/LeMuffinMan/ChessGame/actions/workflows/deploy.yml)
[![Live Demo](https://img.shields.io/badge/demo-live-brightgreen)](https://lemuffinman.github.io/ChessGame/)

<p align="center"><strong><a href="https://lemuffinman.github.io/ChessGame/">▶ Play</a></strong></p>

---

I built this project to learn Rust on something real and not exercises or tutorials. The rules are complex enough to punish bad design (and they did), the algorithms are well-documented,  the Chess Programming Wiki, which I discovered exists and is enormous, became my bible over the two intense sprints I spent on this project. Also, seeing how simple evaluation criteria (material value and position) can lead to natural openings and an already decent bot, before improving it until you get mated by the algorithm you built… that's quite exciting. I now aim to integrate UCI to measure its Elo against other engines.

**Speed buys intelligence.**
Since I haven’t yet implemented parallelism on WASM or threads on native, the goal was to push depth as far as possible within a 300ms budget — enough to keep the UI responsive. What I found satisfying was that each bottleneck was measurable: the bench infrastructure made every optimization visible, and clearing one bottleneck felt like unlocking resources to invest elsewhere. A deeper, faster search means better move ordering pays off more, a richer evaluation stays affordable. That’s the trade-off behind every decision: the ladder mate evaluator is probably where I’m spending too many cycles now, but at least the bot doesn’t miss an easy rook-and-king mate.

**A well-placed hint.**
A friend who convinced me to start this project and learn Rust along the way gave me one early nudge: model the board around `enum Cell { Occupied(Piece, Color), Free }`. That was enough. Following that thread, I found myself reaching naturally for exhaustive pattern matching, `Option<Coord>` for en passant and check state where null is impossible by construction, zero-size types, traits for abstraction without overhead. I didn’t study Rust — I developed a taste for it.

**From 3 seconds to 300ms.**
The performance story starts at depth 5 taking 3 seconds — purest minimax, no pruning, full board evaluation through a generic `Evaluator` trait at every node. Alpha-beta alone cut that by an order of magnitude. Move ordering (MVV-LVA, killers, history) pushed the branching factor down further. Replacing the per-leaf trait evaluation with an incremental score maintained directly inside `apply` and `undo` removed the evaluation overhead entirely. Then a Transposition Table — 1M entries in a fixed `Vec`, indexed by Zobrist hash, with generation counters to isolate games — collapsed the tree on repeated positions. Depth 5 now runs in under 30ms, depth 11 around 300ms.

---

## What it does

- **Play chess** — human vs human, human vs bot, or bot vs bot with all common chess features
- **Standard Timers** —  with standard time controls (blitz, rapid…) in human vs human games
- **AI opponent** — alpha-beta engine at configurable depth, three difficulty levels with iterative deepening
- **Hint system** — ask for the engine's best move suggestion at any point
- **Replay & PGN** — game replay and PGN export
- **Bench page** — standalone `bench.html` comparing performance across positions and depths (native vs WASM if running localy)
- **Portable** — WASM build runs in any browser on any device; native binary available for desktop and benchmarking
- **Responsive** — dedicated UI for desktop and mobile

<p align="center">
  <img src="assets/screenshot_demo1.png" width="750" alt="Desktop demo" />
  <img src="assets/mobile_demo.gif" width="225" alt="Mobile demo" />
</p>

---

## Benchmarks

The project includes a standalone benchmark page (`bench.html`) that runs the engine against standard positions at increasing depths and reports nodes per second, effective branching factor, and quiescence node ratio.

The native vs WASM comparison requires a local setup — run `just bench-all` after cloning to generate `public/native_bench.json` and open `bench.html` in your browser. This comparison is not available on the live demo.

> **Note on reliability:** comparing native and WASM numbers is only meaningful on the same machine. Even then, WASM performance is harder to measure accurately: the browser introduces scheduling noise, lacks SIMD optimizations, and runs single-threaded in a sandboxed environment. Treat WASM NPS as a relative indicator across depths or positions — comparing it to native figures from a different machine has limited value.

<p align="center">
  <img src="assets/260502_20h39m56s_screenshot.png" width="750" alt="Bench depth 10 WASM - Native" />
</p>

---

## Algorithms

See [docs/ALGORITHMS.md](docs/ALGORITHMS.md) for context and implementation notes on each technique.

<table>
<tr>
<td valign="top">

**Search**

| Technique | Ref |
|---|---|
| Alpha-Beta Pruning | [CPW](https://www.chessprogramming.org/Alpha-Beta) |
| Principal Variation Search | [CPW](https://www.chessprogramming.org/Principal_Variation_Search) |
| Iterative Deepening | [CPW](https://www.chessprogramming.org/Iterative_Deepening) |
| Aspiration Windows | [CPW](https://www.chessprogramming.org/Aspiration_Windows) |
| Null Move Pruning | [CPW](https://www.chessprogramming.org/Null_Move_Pruning) |
| Late Move Reductions (LMR) | [CPW](https://www.chessprogramming.org/Late_Move_Reductions) |
| Futility Pruning | [CPW](https://www.chessprogramming.org/Futility_Pruning) |
| Check Extensions | [CPW](https://www.chessprogramming.org/Check_Extensions) |
| Quiescence Search | [CPW](https://www.chessprogramming.org/Quiescence_Search) |
| Delta Pruning (in quiescence) | [CPW](https://www.chessprogramming.org/Delta_Pruning) |

**Hashing & memory**

| Technique | Ref |
|---|---|
| Zobrist Hashing (incremental) | [CPW](https://www.chessprogramming.org/Zobrist_Hashing) |
| Transposition Table (fixed Vec 1M) | [CPW](https://www.chessprogramming.org/Transposition_Table) |

</td>
<td valign="top">

**Move ordering**

| Technique | Ref |
|---|---|
| MVV-LVA | [CPW](https://www.chessprogramming.org/MVV-LVA) |
| Killer Move Heuristic | [CPW](https://www.chessprogramming.org/Killer_Move) |
| History Heuristic | [CPW](https://www.chessprogramming.org/History_Heuristic) |
| TT move ordering | [CPW](https://www.chessprogramming.org/Transposition_Table) |

**Evaluation**

| Technique | Ref |
|---|---|
| Piece-Square Tables (opening/endgame) | [CPW](https://www.chessprogramming.org/Piece-Square_Tables) |
| King Safety (pawn shield + attackers) | [CPW](https://www.chessprogramming.org/King_Safety) |
| Pin Detection | [CPW](https://www.chessprogramming.org/Pin) |
| Mop-up Evaluation | [CPW](https://www.chessprogramming.org/Mop-up_Evaluation) |
| King Corner Pressure | [CPW](https://www.chessprogramming.org/King_Centralization) |
| Rook & Bishop Cut Bonus | [CPW](https://www.chessprogramming.org/Rook_Endgames) |

</td>
</tr>
</table>

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

The `engine/` module has zero dependency on `egui`. It allows us to run our separate benchmark binary with it, and open the path for UCI implementation.


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

`just` wraps `cargo` to handle compilation targets (WASM vs native) and debug/release profiles, and pipelines multi-step workflows. You can always use raw `cargo` commands instead.

```bash
# WASM — runs in the browser via trunk
just wasm          # dev server with hot-reload → http://127.0.0.1:8080
just wasm-release  # release build (optimized, use before benchmarking WASM)

# Native — runs as a desktop binary
just native        # debug build (fast compile, use during development)
just n             # release build (full optimizations, use for actual play)

# Bench — native vs WASM comparison
just bench-all     # generate native_bench.json then start WASM → bench.html

# Quality
just test          # cargo test
just clippy        # clippy for both native and wasm32 targets
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
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Other Rust Chess Engines](https://www.chessprogramming.org/Category:Rust)

**WebAssembly**
- [Rust and WebAssembly book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen docs](https://rustwasm.github.io/docs/wasm-bindgen/)

**Chess programming**
- [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)
- [Minimax](https://www.chessprogramming.org/Minimax) · [Alpha-Beta](https://www.chessprogramming.org/Alpha-Beta) · [Quiescence Search](https://www.chessprogramming.org/Quiescence_Search)
- [Transposition Table](https://www.chessprogramming.org/Transposition_Table) · [Zobrist Hashing](https://www.chessprogramming.org/Zobrist_Hashing)
- [rustic-chess — MVV-LVA](https://rustic-chess.org/search/ordering/mvv_lva.html)

**Tools**
- [egui](https://github.com/emilk/egui)
- [trunk](https://trunkrs.dev/)
- [just](https://github.com/casey/just)
