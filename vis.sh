#! /usr/bin/bash
set -eu

script_dir=$(dirname $0)
work_dir=$script_dir/work

cargo build --release --bin vis
tmpfile=$(mktemp)
trap "rm $tmpfile" 0
xclip -o -selection clipboard > $tmpfile
time ./target/release/vis < $tmpfile > $work_dir/graph.dot
dot -Tpng $work_dir/graph.dot > $work_dir/graph.png
open $work_dir/graph.png
