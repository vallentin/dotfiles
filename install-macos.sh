#!/usr/bin/env bash

set -e

dirname=$(cd "$(dirname "${BASH_SOURCE[0]}")"; pwd -P)
cd "$dirname"

if [ "$TERM_PROGRAM" == "vscode" ]; then
    clear
fi

rc="$HOME/.zshrc"

line='export PATH="$HOME/bin:$PATH"'
if ! grep --quiet --fixed-strings --line-regexp "$line" "$rc"; then
    echo "Appending to \`$rc\`"
    echo >> "$rc"
    echo "$line" >> "$rc"
fi

echo "Installed"
echo "Restart the terminal or \`source \$HOME/.bashrc\`"
