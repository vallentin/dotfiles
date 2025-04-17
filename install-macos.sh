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

cp -v -a ".val-macos/." "$val"

cargo run --manifest-path "tools/Cargo.toml" -- install

if [ ! -f "$HOME/.cargo/config.toml" ]; then
    echo "Installing \`~/.cargo/config.toml\`"
    cp -v ".cargo/config.toml" "$HOME/.cargo/config.toml"
else
    if ! cmp --silent "$HOME/.cargo/config.toml" ".cargo/config.toml"; then
        echo -e "\033[0;31mCargo configuration mismatch\033[0m"
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
