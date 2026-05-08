# Justfile reference

`just` is an optional task runner that wraps `cargo` for multi-target builds and pipelines multi-step workflows. All commands can be replaced by their raw `cargo`/`trunk` equivalents — see below.

Install: `cargo install just`

---

## Quick reference

| Command | What it does |
|---|---|
| `just wasm-run` | WASM release dev server (hot-reload) → http://127.0.0.1:8080 |
| `just wasm-run-debug` | WASM debug dev server (faster compile) |
| `just native-run` | Run native release binary |
| `just native-run-debug` | Run native debug binary (faster compile) |
| `just n` | Alias for `native-run` |
| `just w` | Alias for `wasm-run` |
| `just test [args]` | Run all tests |
| `just t` | Alias for `test` |
| `just perft` | Run perft tests only |
| `just clippy` | Clippy for both native and wasm32 targets |
| `just clippy-native` | Clippy for native target only |
| `just clippy-wasm` | Clippy for wasm32 target only |
| `just bench-native <depth>` | Generate `public/native_bench.json` at given depth |
| `just bench-all <depth>` | Generate native bench JSON then start WASM server |
| `just b` | Alias for `bench-all` |
| `just bench-regression-test` | Run bench at depth 11, compare against baseline (fails if >5% regression) |
| `just update-bench-baseline` | Run bench at depth 11, save as new baseline and commit |
| `just build-uci` | Build the UCI binary |
| `just test-uci` | One game vs Stockfish (skill 0) to debug UCI — requires cutechess-cli |
| `just elo-uci <elo> <games> <concurrency>` | Elo estimate vs Stockfish — requires cutechess-cli |
| `just ci-fast` | Full CI suite: build-uci + tests + bench regression |

---

## WASM

```bash
just wasm-run           # trunk serve --release — hot-reload at http://127.0.0.1:8080
just wasm-run-debug     # trunk serve — debug build, faster compile
```

Equivalent without `just`:
```bash
trunk serve --release
trunk serve
```

---

## Native

```bash
just native-run         # release build — full optimizations, use for actual play
just native-run-debug   # debug build — fast compile, use during development
```

Equivalent without `just`:
```bash
cargo run --release --bin chess_game --features=native
cargo run --bin chess_game --features=native
```

---

## Tests

```bash
just test           # cargo test (all tests)
just perft          # cargo test perft (perft tests only)
```

---

## Clippy

```bash
just clippy         # native + wasm32 (both targets)
just clippy-native  # cargo clippy --features native
just clippy-wasm    # cargo clippy --target wasm32-unknown-unknown
```

Running both targets matters: feature-gated WASM code (`#[cfg(target_arch = "wasm32")]`) is only checked by the wasm32 clippy pass.

---

## Benchmarks

```bash
just bench-native 11     # depth 11, writes public/native_bench.json
just bench-native 8      # depth 8
just bench-all 11        # bench-native then wasm-run → open bench.html
```

`bench-native` runs the native bench binary (`src/bin/bench.rs`) and writes the result to `public/native_bench.json`. This file is picked up by `bench.html` for the native vs WASM comparison.

Force a refresh by deleting the file first:
```bash
rm public/native_bench.json && just bench-all 11
```

Equivalent without `just`:
```bash
mkdir -p public
cargo run --release --features native --bin bench -- measure 11 > public/native_bench.json
```

---

## Bench regression

```bash
just bench-regression-test    # measure depth 11, compare vs baseline, fail if >5% regression
just update-bench-baseline    # measure depth 11, save as new baseline, commit
```

`bench-regression-test` is used in CI (`just ci-fast`). It writes to `bench_results/current.json` and compares against the committed `bench_results/baseline.json`.

Run `just update-bench-baseline` after any intentional search improvement to update the reference.

---

## UCI

These commands require [cutechess-cli](https://github.com/cutechess/cutechess) and a `./stockfish` binary in the project root.

```bash
just build-uci                    # compile target/release/uci
just test-uci                     # one game vs Stockfish skill 0, full debug output
just elo-uci 1500 100 4           # 100 games vs SF@1500, 4 concurrent, writes results_1500.pgn
```

`test-uci` is useful for checking the UCI protocol implementation — the `-debug all` flag logs every command exchanged between cutechess-cli and the engine.

`elo-uci` uses `tc=1+1` (1 min + 1 sec increment). Adjust the time control in the justfile if needed.

---

## CI

```bash
just ci-fast    # build-uci + test + bench-regression-test
```

This is what runs in GitHub Actions on every push to `main`. It checks that the UCI binary compiles, all tests pass, and the bench doesn't regress by more than 5% against the baseline.
