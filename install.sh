#!/usr/bin/env bash

set -e

dirname=$(cd "$(dirname "${BASH_SOURCE[0]}")"; pwd -P)
cd "$dirname"

echo "Copying files..."

cp -r bin ~
cp aliases $HOME/.vallentin-aliases

echo "Marking as executable..."

files=$(ls bin)
cd ~/bin
chmod +x $files

source='source $HOME/.vallentin-aliases'
if ! grep -q "$source" $HOME/.bashrc; then
    echo "Appending to `$HOME/.bashrc`"

    echo >> $HOME/.bashrc
    echo "$source" >> $HOME/.bashrc
fi

echo "Installed"
echo "Restart the terminal or \`source \$HOME/.bashrc\`"
