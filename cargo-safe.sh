#!/usr/bin/env bash
set -euo pipefail

MAX_MEM_HIGH="${CARGO_MEM_HIGH:-3800M}"
MAX_MEM_HARD="${CARGO_MEM_MAX:-4600M}"
MAX_SWAP="${CARGO_MEM_SWAP:-1500M}"
CPU_QUOTA="${CARGO_CPU_QUOTA:-200%}"

if [ "${CARGO_IN_CGROUP_LIMIT:-0}" != "1" ] && command -v systemd-run >/dev/null 2>&1; then
    exec systemd-run --scope --quiet \
        -p "MemoryHigh=$MAX_MEM_HIGH" \
        -p "MemoryMax=$MAX_MEM_HARD" \
        -p "MemorySwapMax=$MAX_SWAP" \
        -p "CPUQuota=$CPU_QUOTA" \
        -p "OOMPolicy=kill" \
        env CARGO_IN_CGROUP_LIMIT=1 MALLOC_ARENA_MAX=2 CARGO_BUILD_JOBS=1 \
        bash -c 'echo "${CARGO_OOM_SCORE_ADJ:-500}" > /proc/self/oom_score_adj 2>/dev/null || true; exec cargo "$@"' _ "$@"
fi

echo "${CARGO_OOM_SCORE_ADJ:-500}" > /proc/self/oom_score_adj 2>/dev/null || true
exec env MALLOC_ARENA_MAX=2 CARGO_BUILD_JOBS=1 cargo "$@"
