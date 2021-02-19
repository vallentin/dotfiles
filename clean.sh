#!/usr/bin/env bash

set -e

dirname=$(cd "$(dirname "${BASH_SOURCE[0]}")"; pwd -P)
cd "$dirname"

files=$(git log --all --pretty=format: --name-only --diff-filter=A | sort -u | grep "bin/" | sed "s/bin\///")

for name in $files; do
    echo '~/bin/'$name
done

read -p "Remove the listed files? [y/n] " -n 1 -r

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
fi

echo

for name in $files; do
    rm -fv ~/bin/$name
done
