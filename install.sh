#!/usr/bin/env bash

set -e

dirname=$(cd "$(dirname "${BASH_SOURCE[0]}")"; pwd -P)
cd "$dirname"

cp -r bin ~
cp aliases $HOME/.vallentin-aliases

files=$(ls bin)
cd ~/bin
chmod +x $files

source='source $HOME/.vallentin-aliases'
if ! grep -q "$source" $HOME/.bashrc; then
    echo >> $HOME/.bashrc
    echo "$source" >> $HOME/.bashrc
fi

# Then execute `source $HOME/.bashrc`
