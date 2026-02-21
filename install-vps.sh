#!/usr/bin/env bash

set -euo pipefail

dirname=$(cd "$(dirname "${BASH_SOURCE[0]}")"; pwd -P)
cd "$dirname"

if [[ "${TERM_PROGRAM:-}" == "vscode" ]]; then
    clear
fi

# chmod 755 for dirs and 644 for files
rsync -v -rz -p --chmod=Du=rwx,Dgo=rx,Fu=rw,Fog=r --progress \
    vps/ root@vallentin.dev:/etc/val

for user in root vallentin; do
    echo "Installing .bashrc for $user..."
    ssh $user@vallentin.dev "bash -ls" <<'EOF'
set -euo pipefail

rc="$HOME/.bashrc"
line="source '/etc/val/.bashrc'"

if ! grep --quiet --fixed-strings --line-regexp "$line" "$rc"; then
    echo "Appending to \`$rc\`"
    echo >> "$rc"
    echo "$line" >> "$rc"
fi
EOF
done

echo "Installed"
echo "Restart terminal or \`source ~/.bashrc\`"
