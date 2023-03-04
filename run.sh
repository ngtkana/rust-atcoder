# /usr/bin/bash

set -euC

# # rust-analyzer 対応 （https://github.com/rust-analyzer/rust-analyzer/pull/10457#issuecomment-945069920）
xclip -o -selection clipboard | cargo +1.42 run
