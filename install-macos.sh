#!/usr/bin/env bash

set -e

dirname=$(cd "$(dirname "${BASH_SOURCE[0]}")"; pwd -P)
cd "$dirname"

if [ "$TERM_PROGRAM" == "vscode" ]; then
    clear
fi

rc="$HOME/.zshrc"

val="$HOME/.val"

mkdir -v -p "$val"

if [ -e ".val" ]; then
    find ".val" -mindepth 1 -maxdepth 1 -exec ln -v -f -s "$dirname/{}" "$val/" \;
fi

find ".val-macos" -mindepth 1 -maxdepth 1 -exec ln -v -f -s "$dirname/{}" "$val/" \;

# ln -v -f -s "$dirname/.val-macos/.zshrc" "$val/.zshrc"
ln -v -f -s "$dirname/aliases" "$val/aliases"

# Symlink the `dotfiles` dir into `~/.val`
ln -v -f -s "$dirname" "$val/"

cargo run --release --manifest-path "tools/Cargo.toml" -- install

if [ ! -f "$HOME/.cargo/config.toml" ]; then
    echo "Installing \`~/.cargo/config.toml\`"
    ln -v -f -s "$dirname/.cargo/config.toml" "$HOME/.cargo/config.toml"
else
    if ! cmp --silent "$HOME/.cargo/config.toml" ".cargo/config.toml"; then
        echo -e "\033[0;31mCargo configuration mismatch\033[0m"
        exit 1
    else
        echo "Cargo configuration already installed"
    fi
fi

echo "Marking as executable..."
find "$val/bin" -type f -print -exec chmod +x {} \;

line='source "$HOME/.val/.zshrc"'
if ! grep --quiet --fixed-strings --line-regexp "$line" "$rc"; then
    echo "Appending to \`$rc\`"
    echo >> "$rc"
    echo "$line" >> "$rc"
fi

echo "Installed"
echo "Restart the terminal or \`source ~/.zshrc\`"

# source "$HOME/.zshrc"
source "$HOME/.val/.zshrc"
