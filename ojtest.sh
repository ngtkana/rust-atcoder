set -euC

cargo build --release
oj t -c ./target/release/rust-atcoder
