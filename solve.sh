set -eu

cargo build
tmpfile=$(mktemp)
trap "rm $tmpfile" 0
xclip -o -selection clipboard > $tmpfile
time ./target/debug/rust-atcoder < $tmpfile
