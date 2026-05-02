# #!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

default:
    @just --list

DIST_DIR := "dist"
PUBLIC_DIR := "public"
FEATURES := "native"
DEFAULT_DEPTH := "11"

alias t := test
alias b := bench-all

wasm *args:
    trunk serve {{args}}

wasm-release *args:
    just wasm --release {{args}}


native *args:
    cargo run --bin chess_game --features={{FEATURES}} {{args}}


bench-native depth=DEFAULT_DEPTH:
    @mkdir -p {{PUBLIC_DIR}}
    cargo run --release --features {{FEATURES}} --bin bench -- {{depth}} > {{PUBLIC_DIR}}/native_bench.json

bench-all *args:
    @if [ ! -f {{PUBLIC_DIR}}/native_bench.json ]; then just bench-native; fi
    just wasm-release {{args}}


test *args:
    cargo test {{args}}

perft:
    cargo test perft
