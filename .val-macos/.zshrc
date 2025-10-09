# This file is installed in `~/.val` as `.zshrc`

source "$HOME/.val/aliases"
source "$HOME/.val/nas"

if [ -n "$ZSH_VERSION" ]; then
    setopt no_share_history
    unsetopt share_history
fi

export PATH="$HOME/.val/bin:$PATH"

export VAL_DIR="$HOME/.val"
export ICLOUD_DIR="$HOME/Library/Mobile Documents/com~apple~CloudDocs"

alias cdir='cd "${_%/*}"'

alias cd-val="cd \"$VAL_DIR\""
alias cd-icloud-drive="cd \"$ICLOUD_DIR\""

ln-to-here() {
    local force=""
    if [[ "$1" == "-f" ]]; then
        force="-f"
        shift
    fi

    if [[ -z "$1" ]]; then
        echo "Usage: ln-to-here [-f] <path>"
        return 1
    fi

    # Do not name the var `path`, as `path` is
    # special in Zsh and related to `$PATH`
    local p=$(realpath "${1}")

    ln -v $force -s "${p}" "./"
}

alias edit-dotfiles="code $HOME/.val/dotfiles"

alias clr="clear"
alias cls="clear"

vscode-clear() {
    if [[ "$TERM_PROGRAM" == "vscode" ]]; then
        clear
    fi
}

alias c="pbcopy"
alias v="pbpaste"

alias run="cargo run --"
alias rrun="cargo run --release --"
alias runq="cargo run --quiet --"
alias rrunq="cargo run --quiet --release --"

alias build="cargo build --"
alias buildr="cargo build --release --"

alias rstest="cargo test --"
alias rstest+="cargo test -- --nocapture"

alias doc="cargo doc"

run-bin() {
    cargo run --bin "${1}" -- "${@:2}"
}

rrun-bin() {
    cargo run --release --bin "${1}" -- "${@:2}"
}

runq-bin() {
    cargo run --quiet --bin "${1}" -- "${@:2}"
}

rrunq-bin() {
    cargo run --quiet --release --bin "${1}" -- "${@:2}"
}

rs-get-single-example() {
    local example="$1"
    if [[ -z "${example}" ]]; then
        filename=$(get-single-file-in-dir "./examples") || return 1
        basename=$(basename "${filename}")
        example="${basename%.*}"
    fi
    echo "$example"
}

run-example() {
    local example
    example=$(rs-get-single-example "${1}") || return 1
    cargo run --example "${example}" -- "${@:2}"
}

rrun-example() {
    local example
    example=$(rs-get-single-example "${1}") || return 1
    cargo run --release --example "$example" -- "${@:2}"
}

runq-example() {
    local example
    example=$(rs-get-single-example "${1}") || return 1
    cargo run --quiet --example "${example}" -- "${@:2}"
}

rrunq-example() {
    local example
    example=$(rs-get-single-example "${1}") || return 1
    cargo run --quiet --release --example "$example" -- "${@:2}"
}

alias br="bacon run"
alias bt="bacon test"

path-rs-add() {(
    set -euo pipefail

    cargo_toml=$(cargo locate-project --message-format plain)
    pkg_name=$(cargo metadata --no-deps --format-version 1 \
        | jq -r '.packages[0].name')

    bin="$HOME/.val/bin/$pkg_name"
    _bin="$HOME/.val/bin/_$pkg_name"

    cat << EOF > "${bin}"
set -euo pipefail

if [[ "\$TERM_PROGRAM" == "vscode" ]]; then
    clear
fi

cargo run --quiet --release --manifest-path "${cargo_toml}" -- "\$@"
EOF

    cat << EOF > "${_bin}"
set -euo pipefail

if [[ "\$TERM_PROGRAM" == "vscode" ]]; then
    clear
fi

cargo run --quiet --manifest-path "${cargo_toml}" -- "\$@"
EOF

    chmod +x "${bin}" "${_bin}"

    echo "Installed: $pkg_name"
)}

path-rs-rm() {(
    set -euo pipefail

    pkg_name=$(cargo metadata --no-deps --format-version 1 \
        | jq -r '.packages[0].name')

    bin="$HOME/.val/bin/$pkg_name"
    _bin="$HOME/.val/bin/_$pkg_name"

    rm -f "${bin}"
    rm -f "${_bin}"

    echo "Uninstalled: $pkg_name"
)}

unalias rs-grep
rs-grep() {
    grep -rin \
        --include="*.rs" \
        --exclude-dir=".git" \
        --exclude-dir="target" \
        --exclude-dir="node_modules" \
        . -e \
        "$@"
}

sh-grep() {
    grep -rin \
        --include="*.sh" \
        --exclude-dir=".git" \
        --exclude-dir="target" \
        --exclude-dir="node_modules" \
        . -e \
        "$@"
}

alias sqlite="sqlite3"

alias ssh-vps="ssh vallentin@vallentin.dev"
alias ssh-vps-root="ssh root@vallentin.dev"

# ssh-vps() {(
#     set -e
#
#     set -o allexport
#     source "$HOME/.val/.env"
#     set +o allexport
#
#     ssh -i "${SSH_KEY}" "${SSH_USER}"@"${SSH_ADDR}"
# )}

alias GET="curl --location"

# Used in `git-open-remote` and `git-open-upstream`
alias open-url="open -u"

# Converts GitHub SSH URL into HTTPS URL, on mismatch
# then the input is echoed back
# Examples:
# - `git@github.com:foo/bar`     -> `https://github.com/foo/bar`
# - `git@github.com:foo/bar.git` -> `https://github.com/foo/bar`
gh-ssh-to-https() {
    local url="$1"
    local re='^git@github.com:([^/]+)/([^/.]+)(\.git)?$'
    if [[ "$url" =~ $re ]]; then
        local user="${match[1]}"
        local repo="${match[2]}"
        echo "https://github.com/$user/$repo"
    else
        echo $url
    fi
}

get-single-file-in-dir() {
    local dir="${1}"
    if [[ ! -d "${dir}" ]]; then
        echo "Error: Not a directory" >&2
        return 1
    fi

    local files=($(find "${dir}" -mindepth 1 -maxdepth 1 -type f -print))

    if [[ ${#files[@]} -eq 1 ]]; then
        echo "${files[@]}"
    elif [[ ${#files[@]} -gt 1 ]]; then
        echo "Error: Multiple files" >&2
        for f in "${files[@]}"; do
            echo "  $f" >&2
        done
        return 1
    else
        echo "Error: No files found" >&2
        return 1
    fi
}

alias stay-awake="caffeinate -id"

clean-ds-store() {
    find . -name ".DS_Store" -type f -print -delete
}

dock-add-spacer() {(
    set -e
    defaults write com.apple.dock persistent-apps -array-add '{tile-data={}; tile-type="spacer-tile";}'
    killall Dock
)}

dock-add-spacer-small() {(
    set -e
    defaults write com.apple.dock persistent-apps -array-add '{"tile-type"="small-spacer-tile";}'
    killall Dock
)}

reload-val() {
    # source "$HOME/.zshrc"
    source "$HOME/.val/.zshrc"
}

reinstall-tools() {
    manifest_path="$HOME/.val/dotfiles/tools/Cargo.toml"
    FORCE_TOOL="tools" cargo run --release --manifest-path "${manifest_path}" -- install
}
