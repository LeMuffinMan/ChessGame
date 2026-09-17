#!/usr/bin/env bash

set -euo pipefail

rounds=${1:-10}
tc=${2:-10+0.1}
concurrency=${3:-3}

cd "$(dirname "$0")/.."

if [ ! -x ./target/release/uci ]; then
    echo "sanity: ./target/release/uci is missing, run just build-uci" >&2
    exit 2
fi

if pgrep -x cutechess-cli >/dev/null; then
    echo "sanity: another cutechess-cli is running, refusing to share the machine" >&2
    exit 2
fi

expected=$((rounds * 2))
stamp=$(date +%Y%m%d-%H%M%S)
pgn="results_sanity_$stamp.pgn"
log="results_sanity_$stamp.log"

cutechess-cli \
    -engine name=a cmd=./target/release/uci \
    -engine name=b cmd=./target/release/uci \
    -each proto=uci tc="$tc" \
    -rounds "$rounds" \
    -games 2 \
    -concurrency "$concurrency" \
    -repeat \
    -openings file=books/8mvs_big_+80_+109.epd format=epd order=random \
    -pgnout "$pgn" > "$log" 2>&1 || true

count() { grep -c "$1" "$pgn" 2>/dev/null || true; }

played=$(count '^\[Result ')
forfeits=$(count '^\[Termination "time forfeit"\]')
illegal=$(count '^\[Termination "illegal move"\]')
stalled=$(count '^\[Termination "unterminated"\]')
abandoned=$(count '^\[Termination "abandoned"\]')
unfinished=$(count '^\[Result "\*"\]')
missing=$((expected - played))
[ "$missing" -lt 0 ] && missing=0

printf '%-12s %s\n'   "pgn"        "$pgn"
printf '%-12s %s\n'   "log"        "$log"
printf '%-12s %s\n\n' "tc"         "$tc"
printf '%-12s %5d\n'  "played"     "$played"
printf '%-12s %5d\n'  "missing"    "$missing"
printf '%-12s %5d\n'  "forfeits"   "$forfeits"
printf '%-12s %5d\n'  "illegal"    "$illegal"
printf '%-12s %5d\n'  "abandoned"  "$abandoned"
printf '%-12s %5d\n'  "unfinished" "$unfinished"
printf '%-12s %5d\n'  "stalled"    "$stalled"

failures=$((missing + forfeits + illegal + abandoned + unfinished))

if [ "$failures" -ne 0 ]; then
    printf '\nSANITY KO: %d\n' "$failures"
    exit 1
fi

printf '\nSANITY OK\n'
