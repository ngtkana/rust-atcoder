# /usr/bin/bash
set -euC

function print_help() {
    echo -e "usage: ${0} [--release]"
}

is_wsl() {
    case "$(uname -r)" in
    *microsoft* ) true ;; # WSL 2
    *Microsoft* ) true ;; # WSL 1
    * ) false;;
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
latest=0
while [[ $# -gt 0 ]]; do
    case $1 in
        '--release')
            release=1
            ;;
        *) echo -e "Unknown option $1";
            print_help
            exit 1;;
    esac
    shift
done

if [ $release -eq 1 ]; then
    args='run --release'
else
    args='run'
fi

echo -e "args=\"${args}\""

paste | cargo ${args}
