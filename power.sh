#! /usr/bin/bash
set -eu

cargo build --bin gen
tmpfile=$(mktemp)
trap "rm $tmpfile" 0
xclip -o -selection clipboard | ./target/debug/gen > $tmpfile
echo "== Start of input =="
cat $tmpfile
echo "== End of input =="
cargo build --bin solve
time ./target/debug/solve < $tmpfile
