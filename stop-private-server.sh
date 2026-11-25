#!/usr/bin/env bash
set -euo pipefail

for name in sonoro-gamesv sonoro-loginsv sonoro-cfgsv sonoro-aki-logic sonoro-server-manager; do
  pidfile="/tmp/${name}.pid"
  if [[ -f "$pidfile" ]]; then
    pid="$(cat "$pidfile" 2>/dev/null || true)"
    if [[ -n "$pid" ]]; then
      kill "$pid" 2>/dev/null || true
    fi
    rm -f "$pidfile"
  fi
  if [[ "$name" != "sonoro-server-manager" ]]; then
    pkill -f "/target/release/${name}" 2>/dev/null || true
  fi
done
pkill -f "^python3 .*server_manager\.py" 2>/dev/null || true

sleep 0.5
for name in sonoro-gamesv sonoro-loginsv sonoro-cfgsv sonoro-aki-logic; do
  pkill -9 -f "/target/release/${name}" 2>/dev/null || true
done
pkill -9 -f "^python3 .*server_manager\.py" 2>/dev/null || true
rm -f /tmp/aki_logic.sock
echo "Private server stopped."
