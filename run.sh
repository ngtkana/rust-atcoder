# /usr/bin/bash
set -euC

function print_help() {
    echo -e "usage: ${0} [--release]"
}

release=0
latest=0
while [[ $# -gt 0 ]]; do
    case $1 in
        '--release')
            release=1
            ;;
        *) echo "Unknown option $1";
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

echo "args=\"${args}\""

xclip -o -selection clipboard | cargo ${args}
