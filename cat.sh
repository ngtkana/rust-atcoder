#! /usr/bin/bash
set -euC

is_wsl() {
	case "$(uname -r)" in
	*microsoft*) true ;; # WSL 2
	*Microsoft*) true ;; # WSL 1
	*) false ;;
	esac
}

if is_wsl; then
	clip.exe < src/solve.rs
else
	xclip -selection clipboard < src/solve.rs
fi
