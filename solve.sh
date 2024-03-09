#! /usr/bin/bash
set -eu

is_wsl() {
	case "$(uname -r)" in
	*microsoft*) true ;; # WSL 2
	*Microsoft*) true ;; # WSL 1
	*) false ;;
	esac
}

paste() {
	if is_wsl; then
		powershell.exe -Command Get-Clipboard
	else
		xclip -o -selection clipboard
	fi
}

cargo build --release --bin solve
tmpfile=$(mktemp)
trap "rm $tmpfile" 0
paste > $tmpfile
time ./target/release/solve < $tmpfile
