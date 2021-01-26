# /usr/bin/bash

set -euC

xclip -o -selection clipboard | cargo run
