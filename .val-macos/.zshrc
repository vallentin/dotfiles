# This file is located in `~/.val` as `.zshrc`

export PATH="$HOME/.val/bin:$PATH"

export VAL_DIR="$HOME/.val"
export ICLOUD_DIR="$HOME/Library/Mobile Documents/com~apple~CloudDocs"

alias cd-val="cd \"$VAL_DIR\""
alias cd-icloud-drive="cd \"$ICLOUD_DIR\""

alias c="pbcopy"
alias v="pbpaste"

ssh-vps() {(
    set -e

    set -o allexport
    source "$HOME/.val/.env"
    set +o allexport

    ssh -i "${SSH_KEY}" "${SSH_USER}"@"${SSH_ADDR}"
)}

clean-ds-store() {
    find . -name ".DS_Store" -type f -print -delete
}

source "$HOME/.val/aliases"

reload-val() {
    # source "$HOME/.zshrc"
    source "$HOME/.val/.zshrc"
}
