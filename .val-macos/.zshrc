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

alias build="cargo build --"
alias buildr="cargo build --release --"

alias doc="cargo doc"

run-example() {
    local example="$1"
    if [[ -z "${example}" ]]; then
        filename=$(get-single-file-in-dir "./examples")
        basename=$(basename "${filename}")
        example="${basename%.*}"
    fi

    cargo run --example "$example" -- "${@:2}"
}

rrun-example() {
    local example="$1"
    if [[ -z "${example}" ]]; then
        filename=$(get-single-file-in-dir "./examples")
        basename=$(basename "${filename}")
        example="${basename%.*}"
    fi

    cargo run --release --example "$example" -- "${@:2}"
}

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

    local entries=()
    if compgen -G "$dir/*" > /dev/null 2>&1; then
        entries=("$dir"/*)
    fi

    local files=()
    for f in "${entries[@]}"; do
        [[ -f "$f" ]] && files+=("$f")
    done

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
