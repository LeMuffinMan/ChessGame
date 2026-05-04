default:
    @just --list

DIST_DIR := "dist"
PUBLIC_DIR := "public"
FEATURES := "native"
DEFAULT_DEPTH := "11"

alias t := test
alias b := bench-all
alias n := native-release
alias w := wasm-release

# Start trunk dev server with hot-reload
wasm *args:
    trunk serve {{args}}

# Build WASM release and start trunk
wasm-release *args:
    just wasm --release {{args}}

# Run native binary (debug)
native *args:
    cargo run --bin chess_game --features={{FEATURES}} {{args}}

# Run native binary (release)
native-release *args:
    cargo run --release --bin chess_game --features={{FEATURES}} {{args}}

# Generate public/native_bench.json at given depth (default: 11)
bench-native depth=DEFAULT_DEPTH:
    @mkdir -p {{PUBLIC_DIR}}
    cargo run --release --features {{FEATURES}} --bin bench -- {{depth}} > {{PUBLIC_DIR}}/native_bench.json

# Run native bench (if missing) then build WASM release
bench-all *args:
    @if [ ! -f {{PUBLIC_DIR}}/native_bench.json ]; then just bench-native; fi
    just wasm-release {{args}}

# Build uci
uci:
    cargo build --bin uci --features=native

# Run the engine against Stockfish to debug uci
test-uci: uci
    cutechess-cli \
    -engine cmd=./stockfish proto=uci name=SF \
    -engine cmd=./target/debug/uci proto=uci name=CG \
    -each tc=1+0.1 \
    -games 10 \
    -debug all

# Run tests
test *args:
    cargo test {{args}}

# Run perft tests only
perft:
    cargo test perft

# Run clippy for native and WASM target
clippy: clippy-native clippy-wasm

# Run clippy for native target
clippy-native:
    cargo clippy --features native

# Run clippy for WASM target
clippy-wasm:
    cargo clippy --target wasm32-unknown-unknown
