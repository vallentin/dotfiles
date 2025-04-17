# This file is located in `~/.val` as `.zshrc`

export PATH="$HOME/.val/bin:$PATH"

alias cd-val="cd ~/.val"

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
