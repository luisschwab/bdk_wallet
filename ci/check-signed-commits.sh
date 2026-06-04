#!/bin/bash

set -euo pipefail

BASE_SHA="${1:-}"
HEAD_SHA="${2:-}"

if [ -z "$BASE_SHA" ] || [ -z "$HEAD_SHA" ]; then
    echo "Usage: $0 <base_sha> <head_sha>"
    exit 1
fi

UNSIGNED=0
while IFS=' ' read -r commit status; do
    if [ "$status" = "N" ]; then
        echo "Commit $commit is not GPG signed."
        UNSIGNED=$((UNSIGNED + 1))
    fi
done < <(git log --format="%H %G?" "${BASE_SHA}..${HEAD_SHA}")

if [ "$UNSIGNED" -gt 0 ]; then
    echo "Error: $UNSIGNED commit(s) are not GPG signed. See CONTRIBUTING.md."
    exit 1
fi

echo "All commits are GPG signed."
