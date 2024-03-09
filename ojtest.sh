#! /usr/bin/bash
set -euC

cargo build --release --bin solve
oj t -c ./target/release/solve
