# /usr/bin/bash
set -euC

function show_help() {
    echo -e "usage: ${0} <contest_id> <problem_id>"
    echo -e "contet_id:\t(ex. abc297)"
    echo -e "problem_id:\t(ex. c)"
}

if [ $# -ne 2 ]; then
    show_help
    exit 1
fi

contest_id="${1}"
problem_id="${2}"

if [ -d ./test ]; then
    rm -r ./test
fi

acc url "${1}" "${1}_${2}" | oj d $(cat)
