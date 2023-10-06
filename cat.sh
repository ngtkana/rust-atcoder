# /usr/bin/bash
set -euC

is_wsl() {
    case "$(uname -r)" in
    *microsoft* ) true ;; # WSL 2
    *Microsoft* ) true ;; # WSL 1
    * ) false;;
    esac
}

if is_wsl; then
    cat src/main.rs | clip.exe
else
    cat src/main.rs | xsel -bi
fi
