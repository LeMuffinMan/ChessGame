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
pgn="results_sanity_$(date +%Y%m%d-%H%M%S).pgn"

cutechess-cli \
    -engine name=a cmd=./target/release/uci \
    -engine name=b cmd=./target/release/uci \
    -each proto=uci tc="$tc" \
    -rounds "$rounds" \
    -games 2 \
    -concurrency "$concurrency" \
    -repeat \
    -openings file=books/8mvs_big_+80_+109.epd format=epd order=random \
    -pgnout "$pgn" || true

count() { grep -c "$1" "$pgn" 2>/dev/null || true; }

played=$(count '^\[Result ')
forfeits=$(count '^\[Termination "time forfeit"\]')
illegal=$(count '^\[Termination "illegal move"\]')
stalled=$(count '^\[Termination "unterminated"\]')
abandoned=$(count '^\[Termination "abandoned"\]')
unfinished=$(count '^\[Result "\*"\]')
missing=$((expected - played))
[ "$missing" -lt 0 ] && missing=0

printf '\n%-22s %s\n' "pgn" "$pgn"
printf '%-22s %s vs %s at %s\n\n' "match" "a" "b" "$tc"
printf '%-22s %6d\n' "games played" "$played"
printf '%-22s %6d\n' "games missing" "$missing"
printf '%-22s %6d\n' "time forfeits" "$forfeits"
printf '%-22s %6d\n' "illegal moves" "$illegal"
printf '%-22s %6d\n' "abandoned games" "$abandoned"
printf '%-22s %6d\n' "unfinished games" "$unfinished"
printf '%-22s %6d\n' "  of which stalled" "$stalled"

failures=$((missing + forfeits + illegal + abandoned + unfinished))

printf '\nno elo is reported here on purpose: two identical binaries decide nothing,\n'
printf 'and a self play elo of a few dozen points is pure noise.\n\n'

if [ "$failures" -ne 0 ]; then
    echo "SANITY FAILED: $failures binary criteria tripped"
    exit 1
fi

echo "SANITY OK: $played games, none lost on time, none illegal, none stalled"
