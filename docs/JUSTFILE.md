# Justfile reference

`just` is an optional task runner that wraps `cargo` for multi-target builds and pipelines multi-step workflows. All commands can be replaced by their raw `cargo`/`trunk` equivalents — see below.

Install: `cargo install just`

---

## Quick reference

| Command | What it does |
|---|---|
| `just wasm` | Dev server with hot-reload (trunk serve) |
| `just wasm-release` | Release WASM build + dev server |
| `just native` | Run native binary (debug) |
| `just n` | Run native binary (release) — alias for `native-release` |
| `just test` | Run all tests |
| `just t` | Alias for `test` |
| `just perft` | Run perft tests only |
| `just clippy` | Clippy for both native and wasm32 targets |
| `just clippy-native` | Clippy for native target only |
| `just clippy-wasm` | Clippy for wasm32 target only |
| `just bench-native [depth]` | Generate `public/native_bench.json` (default depth: 11) |
| `just bench-all` | Generate native bench JSON then start WASM release server |
| `just b` | Alias for `bench-all` |

---

## WASM

```bash
just wasm           # trunk serve — hot-reload at http://127.0.0.1:8080
just wasm-release   # trunk serve --release (full optimizations — use before benchmarking)
```

Equivalent without `just`:
```bash
trunk serve
trunk serve --release
```

---

## Native

```bash
just native         # debug build — fast compile, use during development
just n              # release build — full optimizations, use for actual play
```

Equivalent without `just`:
```bash
cargo run --bin chess_game --features=native
cargo run --release --bin chess_game --features=native
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

Running both targets is important: feature-gated WASM code (`#[cfg(target_arch = "wasm32")]`) is only checked by the wasm32 clippy pass.

---

## Benchmarks

```bash
just bench-native        # depth 11, writes public/native_bench.json
just bench-native 8      # depth 8
just bench-all           # bench-native (if missing) then wasm-release → open bench.html
```

`bench-native` runs the native bench binary (`src/bin/bench.rs`) and writes the result to `public/native_bench.json`. This file is read by `bench.html` for the native vs WASM comparison.

`bench-all` only regenerates `native_bench.json` if the file is absent. Force a refresh by deleting it first:
```bash
rm public/native_bench.json && just bench-all
```

Equivalent without `just`:
```bash
mkdir -p public
cargo run --release --features native --bin bench -- 11 > public/native_bench.json
```
