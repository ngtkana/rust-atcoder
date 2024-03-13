#! /usr/bin/bash
set -eu

script_dir=$(dirname $0)
work_dir=$script_dir/work

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

release=0
while [[ $# -gt 0 ]]; do
	case $1 in
	'--release')
		release=1
		;;
	*)
		echo -e "Unknown option $1"
		print_help
		exit 1
		;;
	esac
	shift
done

cargo build --release --bin vis
tmpfile=$(mktemp)
trap "rm $tmpfile" 0
paste > $tmpfile
time ./target/release/vis < $tmpfile > $work_dir/graph.dot
dot -Tpng $work_dir/graph.dot > $work_dir/graph.png
open $work_dir/graph.png
