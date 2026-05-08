default:
    @just --list

PUBLIC_DIR := "public"
FEATURES := "native"

alias t := test
alias b := bench-all
alias n := native-run
alias w := wasm-run

# Start WASM release dev server (hot-reload)
wasm-run *args:
    trunk serve --release {{args}}

# Start WASM debug dev server (faster compile)
wasm-run-debug *args:
    trunk serve {{args}}

# Run native release binary
native-run:
    cargo run --release --bin chess_game --features={{FEATURES}}

# Run native debug binary (faster compile)
native-run-debug:
    cargo run --bin chess_game --features={{FEATURES}}

# Generate public/native_bench.json at given depth
bench-native depth:
    @mkdir -p {{PUBLIC_DIR}}
    cargo run --release --features {{FEATURES}} --bin bench -- measure {{depth}} > {{PUBLIC_DIR}}/native_bench.json

# Generate native bench JSON then start WASM release server → open bench.html
bench-all depth *args:
    just bench-native {{depth}}
    just wasm-run {{args}}

# Build UCI binary
build-uci:
    cargo build --release --bin uci --features=native

# Run one game against Stockfish (skill 0) to debug UCI — requires cutechess-cli + stockfish
test-uci: build-uci
    cutechess-cli \
        -engine name=Stockfish_Easy cmd=./stockfish option.Skill\ Level=0 \
        -engine name=ChessGame cmd=./target/release/uci \
        -each proto=uci tc=60+1 \
        -games 1 \
        -repeat \
        -debug all \
        -openings file=books/8mvs_big_+80_+109.epd format=epd order=random

# Run Elo estimate: elo-uci <elo> <games> <concurrency> — requires cutechess-cli + stockfish
elo-uci elo games concurrency: build-uci
    cutechess-cli \
        -engine name=SF_{{elo}} cmd=./stockfish option.UCI_LimitStrength=true option.UCI_Elo={{elo}} \
        -engine name=ChessGame cmd=./target/release/uci \
        -each proto=uci tc=1+1 \
        -games {{games}} \
        -concurrency {{concurrency}} \
        -repeat \
        -openings file=books/8mvs_big_+80_+109.epd format=epd order=random \
        -pgnout results_{{elo}}.pgn

# Run bench at depth 11 and compare against baseline (fails if regression > 5%)
bench-regression-test:
    cargo run --release --features=native --bin bench -- measure 11 > bench_results/current.json
    cargo run --release --features=native --bin bench -- compare bench_results/baseline.json bench_results/current.json 5

# Update bench baseline at depth 11 and commit
update-bench-baseline:
    @mkdir -p bench_results
    cargo run --release --features=native --bin bench -- measure 11 > bench_results/baseline.json
    git add bench_results/baseline.json
    git commit -m "chore: update bench baseline"

# Run full CI suite: build UCI + tests + bench regression
ci-fast: build-uci test bench-regression-test

# Run tests
test *args:
    cargo test {{args}}

# Run perft tests only
perft:
    cargo test perft

# Run clippy for native and WASM targets
clippy: clippy-native clippy-wasm

# Run clippy for native target
clippy-native:
    cargo clippy --features native

# Run clippy for WASM target
clippy-wasm:
    cargo clippy --target wasm32-unknown-unknown
