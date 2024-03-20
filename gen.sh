#! /usr/bin/bash
set -eu

cargo build --release --bin gen
tmpfile=$(mktemp)
trap "rm $tmpfile" 0
xclip -o -selection clipboard > $tmpfile
time ./target/release/gen < $tmpfile
