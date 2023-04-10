# /usr/bin/bash
set -euC

function print_help() {
    echo -e "usage: ${0} [--release] [--latest]"
}

release=0
latest=0
while [[ $# -gt 0 ]]; do
    case $1 in
        '--release')
            release=1
            ;;
        '--latest')
            latest=1
            ;;
        *) echo "Unknown option $1";
            print_help
            exit 1;;
    esac
    shift
done

if [ $release -eq 1 ]; then
    bin='./target/release/main'
    if [ $latest -eq 1 ]; then
        args='build --release'
    else
        args='+1.42 build --release'
    fi
else
    bin='./target/debug/main'
    if [ $latest -eq 1 ]; then
        args='build'
    else
        args='+1.42 build'
    fi
fi

echo -e "args=\"${args}\""
echo -e "bin=\"${bin}\""
echo ""

cargo ${args} && oj t -c ${bin}
