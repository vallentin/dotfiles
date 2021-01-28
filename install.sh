#!/usr/bin/env bash

set -e

dirname=$(cd "$(dirname "${BASH_SOURCE[0]}")"; pwd -P)
cd "$dirname"

cp -r bin ~

files=$(ls bin)
cd ~/bin
chmod +x $files
