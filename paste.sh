#! /usr/bin/sh
set -eux

xclip -o -selection clipboard | cargo run
