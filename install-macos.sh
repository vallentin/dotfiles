#!/usr/bin/env bash

set -e

dirname=$(cd "$(dirname "${BASH_SOURCE[0]}")"; pwd -P)
cd "$dirname"

if [ "$TERM_PROGRAM" == "vscode" ]; then
    clear
fi

rc="$HOME/.zshrc"

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

line='export PATH="$HOME/bin:$PATH"'
if ! grep --quiet --fixed-strings --line-regexp "$line" "$rc"; then
    echo "Appending to \`$rc\`"
    echo >> "$rc"
    echo "$line" >> "$rc"
fi

echo "Installed"
echo "Restart the terminal or \`source ~/.bashrc\`"
