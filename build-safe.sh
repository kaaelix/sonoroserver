#!/usr/bin/env bash
set -euo pipefail

SCRIPT_PATH="$(readlink -f "${BASH_SOURCE[0]}")"
ROOT_DIR="$(cd "$(dirname "$SCRIPT_PATH")" && pwd)"
REAL_ZIG="${ZIG_REAL_BIN:-$ROOT_DIR/.direnv/zig-x86_64-linux-0.16.0/zig}"

if [ ! -x "$REAL_ZIG" ]; then
    echo "Error: Zig 0.16.0 not found at $REAL_ZIG" >&2
    exit 1
fi

MAX_MEM_HIGH="${ZIG_MEM_HIGH:-1200M}"
MAX_MEM_HARD="${ZIG_MEM_MAX:-1800M}"
MAX_SWAP="${ZIG_MEM_SWAP:-2048M}"
CPU_QUOTA="${ZIG_CPU_QUOTA:-150%}"
MAX_RSS_MIB="${ZIG_MAX_RSS_MIB:-1400}"
MAX_RSS_BYTES="${ZIG_MAX_RSS_BYTES:-$((MAX_RSS_MIB * 1024 * 1024))}"
AUTO_STOP_GAMESV="${ZIG_AUTO_STOP_GAMESV:-1}"
MIN_AVAIL_MIB="${ZIG_MIN_AVAIL_MIB:-1200}"
MIN_TOTAL_HEADROOM_MIB="${ZIG_MIN_HEADROOM_MIB:-1800}"

args=("$@")
subcmd="${1:-}"

if [ "${ZIG_IN_CGROUP_LIMIT:-0}" != "1" ] && { [ "$subcmd" = "build" ] || [ "$subcmd" = "test" ] || [ "$subcmd" = "build-exe" ]; }; then
    read_mem_kb() {
        awk -v key="$1:" '$1 == key { print $2; exit }' /proc/meminfo
    }
    mem_avail_mb=$(( $(read_mem_kb MemAvailable) / 1024 ))
    swap_free_mb=$(( $(read_mem_kb SwapFree) / 1024 ))
    if [ "$AUTO_STOP_GAMESV" = "1" ] && [ "$mem_avail_mb" -lt 2400 ] && pgrep -x sonoro-gamesv >/dev/null 2>&1; then
        echo "[build-safe] Low available RAM (${mem_avail_mb} MiB); stopping sonoro-gamesv to prevent OOM..." >&2
        pkill -TERM -x sonoro-gamesv || true
        for _ in 1 2 3 4 5; do
            pgrep -x sonoro-gamesv >/dev/null 2>&1 || break
            sleep 1
        done
        if pgrep -x sonoro-gamesv >/dev/null 2>&1; then
            pkill -KILL -x sonoro-gamesv || true
            sleep 1
        fi
        mem_avail_mb=$(( $(read_mem_kb MemAvailable) / 1024 ))
        swap_free_mb=$(( $(read_mem_kb SwapFree) / 1024 ))
    fi
    total_headroom_mb=$(( mem_avail_mb + swap_free_mb ))
    if [ "$mem_avail_mb" -lt "$MIN_AVAIL_MIB" ] && [ "$total_headroom_mb" -lt "$MIN_TOTAL_HEADROOM_MIB" ]; then
        echo "[build-safe] Refusing $subcmd: MemAvailable=${mem_avail_mb} MiB, SwapFree=${swap_free_mb} MiB (< ${MIN_TOTAL_HEADROOM_MIB} MiB headroom)." >&2
        ps -eo pid,rss:10,comm --sort=-rss | head -n 10 >&2
        exit 125
    fi
fi

if [ "$subcmd" = "build" ]; then
    has_j=0
    has_maxrss=0
    has_strip=0
    has_build_maxrss_opt=0
    for a in "${args[@]}"; do
        case "$a" in
            -j|-j*) has_j=1 ;;
            --maxrss|--maxrss=*) has_maxrss=1 ;;
            -Dstrip|-Dstrip=*) has_strip=1 ;;
            -Dmax-rss-mb=*) has_build_maxrss_opt=1 ;;
        esac
    done
    extra=()
    if [ "$has_j" -eq 0 ]; then
        extra+=("-j1")
    fi
    if [ "$has_maxrss" -eq 0 ]; then
        extra+=("--maxrss" "$MAX_RSS_BYTES")
    fi
    if [ "$has_strip" -eq 0 ]; then
        extra+=("-Dstrip=true")
    fi
    if [ "$has_build_maxrss_opt" -eq 0 ]; then
        extra+=("-Dmax-rss-mb=$MAX_RSS_MIB")
    fi
    if [ "${#extra[@]}" -gt 0 ]; then
        args=("$subcmd" "${extra[@]}" "${args[@]:1}")
    fi
fi

# Prevent nested systemd-run if already inside our zig-build scope
if [ "${ZIG_IN_CGROUP_LIMIT:-0}" != "1" ] && command -v systemd-run >/dev/null 2>&1; then
    exec systemd-run --scope --quiet \
        -p "MemoryHigh=$MAX_MEM_HIGH" \
        -p "MemoryMax=$MAX_MEM_HARD" \
        -p "MemorySwapMax=$MAX_SWAP" \
        -p "CPUQuota=$CPU_QUOTA" \
        -p "OOMPolicy=kill" \
        env ZIG_IN_CGROUP_LIMIT=1 MALLOC_ARENA_MAX=2 \
        bash -c 'echo "${ZIG_OOM_SCORE_ADJ:-500}" > /proc/self/oom_score_adj 2>/dev/null || true; exec "$@"' _ "$REAL_ZIG" "${args[@]}"
fi

echo "${ZIG_OOM_SCORE_ADJ:-500}" > /proc/self/oom_score_adj 2>/dev/null || true
exec env MALLOC_ARENA_MAX=2 "$REAL_ZIG" "${args[@]}"
