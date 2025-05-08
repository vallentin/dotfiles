# This file is installed in `~/.val` as `.zshrc`

source "$HOME/.val/aliases"
source "$HOME/.val/nas"

export PATH="$HOME/.val/bin:$PATH"

export VAL_DIR="$HOME/.val"
export ICLOUD_DIR="$HOME/Library/Mobile Documents/com~apple~CloudDocs"

alias cdir='cd "${_%/*}"'

alias cd-val="cd \"$VAL_DIR\""
alias cd-icloud-drive="cd \"$ICLOUD_DIR\""

alias edit-dotfiles="code $HOME/.val/dotfiles"

alias clr="clear"
alias cls="clear"

alias c="pbcopy"
alias v="pbpaste"

alias run="cargo run --"
alias rrun="cargo run --release --"

alias build="cargo build --"
alias buildr="cargo build --release --"

alias doc="cargo doc"

run-example() {
    cargo run --example "$1" -- "${@:2}"
}

rrun-example() {
    cargo run --release --example "$1" -- "${@:2}"
}

alias GET="curl --location"

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
