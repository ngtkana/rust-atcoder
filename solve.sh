#! /usr/bin/bash
set -eu

cargo build --bin solve
tmpfile=$(mktemp)
trap "rm $tmpfile" 0
xclip -o -selection clipboard > $tmpfile
time ./target/debug/solve < $tmpfile
