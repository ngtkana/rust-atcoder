#! /usr/bin/bash
set -euC

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

cargo build --release
paste | ./target/release/gen | ./target/release/solve
