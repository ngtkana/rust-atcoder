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
	clip.exe <src/main.rs
else
	xclip -selection clipboard <src/main.rs
fi
