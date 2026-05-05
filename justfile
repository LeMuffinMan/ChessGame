default:
    @just --list

DIST_DIR := "dist"
PUBLIC_DIR := "public"
FEATURES := "native"

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

# Generate public/native_bench.json at given depth
bench-native depth:
    @mkdir -p {{PUBLIC_DIR}}
    cargo run --release --features {{FEATURES}} --bin bench -- {{depth}} > {{PUBLIC_DIR}}/native_bench.json

# Run native bench (if missing) then build WASM release
bench-all depth *args:
    just bench-native {{depth}}
    just wasm-release {{args}}

# Build uci
build-uci:
    cargo build --release --bin uci --features=native

# Run the engine against Stockfish to debug uci
test-uci: build-uci
    cutechess-cli \
        -engine name=Stockfish_Easy cmd=./stockfish option.Skill\ Level=0 \
        -engine name=ChessGame cmd=./target/release/uci \
        -each proto=uci tc=60+1 \
        -games 1 \
        -repeat \
        -debug all \
        -openings file=books/8mvs_big_+80_+109.epd format=epd order=random

# Run an elo test : elo-uci <bot_elo> <nb_games>
elo-uci elo games concurrency: build-uci
    cutechess-cli \
        -engine name=SF_{{elo}} cmd=./stockfish option.UCI_LimitStrength=true option.UCI_Elo={{elo}} \
        -engine name=ChessGame cmd=./target/release/uci \
        -each proto=uci tc=60+1 \
        -games {{games}} \
        -concurrency {{concurrency}} \
        -repeat \
        -openings file=books/8mvs_big_+80_+109.epd format=epd order=random \
        -pgnout results_{{elo}}.pgn

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
