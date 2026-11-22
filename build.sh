#!/usr/bin/env bash
set -euo pipefail

SCRIPT_PATH="$(readlink -f "${BASH_SOURCE[0]}")"
ROOT_DIR="$(cd "$(dirname "$SCRIPT_PATH")" && pwd)"

if [ "$#" -eq 0 ]; then
    exec "$ROOT_DIR/build-safe.sh" build gamesv
fi

exec "$ROOT_DIR/build-safe.sh" "$@"
