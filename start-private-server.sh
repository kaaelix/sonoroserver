#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

BIND_HOST="127.0.0.1"
PUBLIC_HOST=""
STATE_DIR="state"
CFG_PORT="15500"
WITH_MANAGER=1

while [[ $# -gt 0 ]]; do
  case "$1" in
    --host)
      BIND_HOST="${2:-127.0.0.1}"
      shift 2
      ;;
    --public-host)
      PUBLIC_HOST="${2:-}"
      shift 2
      ;;
    --state-dir)
      STATE_DIR="${2:-state}"
      shift 2
      ;;
    --cfg-port)
      CFG_PORT="${2:-15500}"
      shift 2
      ;;
    --no-manager)
      WITH_MANAGER=0
      shift
      ;;
    -h|--help)
      echo "Usage: $0 [--host 127.0.0.1|0.0.0.0] [--public-host <IP_OR_HOST>] [--state-dir state] [--cfg-port 15500] [--no-manager]"
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      exit 1
      ;;
  esac
done

if [[ -z "$PUBLIC_HOST" ]]; then
  if [[ "$BIND_HOST" == "0.0.0.0" ]]; then
    PUBLIC_HOST="127.0.0.1"
  else
    PUBLIC_HOST="$BIND_HOST"
  fi
fi

ensure_assets_db() {
  local db="$ROOT_DIR/assets/assets.db"
  local ver_file="$ROOT_DIR/assets/assets.version.json"
  local manifest_tmp="/tmp/sonoro-assets-manifest.json"
  local manifest=""

  for url in \
    "https://raw.githubusercontent.com/kaaelix/sonoroserver/main/assets.json" \
    "https://raw.githubusercontent.com/kaaelix/wuwaserver/main/assets.json"; do
    if curl -fsSL --connect-timeout 3 --max-time 5 "$url" -o "$manifest_tmp" 2>/dev/null; then
      manifest="$manifest_tmp"
      break
    fi
  done

  if [[ -z "$manifest" && -f "$ROOT_DIR/assets.json" ]]; then
    manifest="$ROOT_DIR/assets.json"
  fi
  if [[ -z "$manifest" ]] || ! command -v python3 >/dev/null 2>&1; then
    rm -f "$manifest_tmp"
    return 0
  fi

  local check_out
  check_out="$(python3 - "$manifest" "$db" "$ver_file" <<'PY'
import hashlib, json, os, sys
m = json.load(open(sys.argv[1]))
db, vf = sys.argv[2], sys.argv[3]
target_ver = str(m.get('version', ''))
target_sha = str(m.get('sha256', '')).lower()
size_mb = round(float(m.get('size', 0)) / (1024 * 1024), 1)
urls = [m.get('url', '')] + list(m.get('mirrors') or [])
urls = [u for u in urls if u]

if not os.path.exists(db) or os.path.getsize(db) <= 0:
    print(f"MISSING|{target_ver}||{size_mb}|" + " ".join(urls))
    sys.exit(0)

st = os.stat(db)
local_ver, local_sha = "", ""
if os.path.exists(vf):
    try:
        v = json.load(open(vf))
        if int(v.get('size', -1)) == st.st_size and int(v.get('mtime_ns', -1)) == st.st_mtime_ns:
            local_ver = str(v.get('version', ''))
            local_sha = str(v.get('sha256', '')).lower()
    except Exception:
        pass
if not local_sha:
    h = hashlib.sha256()
    with open(db, 'rb') as f:
        for chunk in iter(lambda: f.read(4 * 1024 * 1024), b''):
            h.update(chunk)
    local_sha = h.hexdigest().lower()
    if target_sha and local_sha == target_sha:
        local_ver = target_ver
        json.dump({'version': local_ver, 'sha256': local_sha, 'size': st.st_size, 'mtime_ns': st.st_mtime_ns}, open(vf, 'w'))

if (target_sha and local_sha == target_sha) or (local_ver and local_ver == target_ver):
    print(f"OK|{target_ver}")
else:
    print(f"UPDATE|{target_ver}|{local_ver or 'local'}|{size_mb}|" + " ".join(urls))
PY
)"

  local status target_ver cur_ver size_mb urls
  IFS='|' read -r status target_ver cur_ver size_mb urls <<< "$check_out"
  if [[ "$status" == "OK" ]]; then
    echo "assets.db: up to date (v${target_ver})"
    rm -f "$manifest_tmp"
    return 0
  fi

  local ans="n"
  if [[ "$status" == "MISSING" ]]; then
    echo "assets/assets.db not found (v${target_ver}, ${size_mb} MB)."
    if [[ -t 0 ]]; then
      read -r -p "Download assets.db now? [Y/n] " ans
      ans="${ans:-y}"
    else
      ans="y"
    fi
  elif [[ "$status" == "UPDATE" ]]; then
    echo "assets.db update available: ${cur_ver} -> v${target_ver} (${size_mb} MB)."
    if [[ -t 0 ]]; then
      read -r -p "Update assets.db now? [y/N] " ans
      ans="${ans:-n}"
    else
      ans="n"
    fi
  fi

  if [[ ! "$ans" =~ ^[Yy] ]]; then
    rm -f "$manifest_tmp"
    if [[ "$status" == "MISSING" ]]; then
      echo "Error: assets/assets.db missing." >&2
      exit 1
    fi
    echo "Keeping current assets.db."
    return 0
  fi

  mkdir -p "$ROOT_DIR/assets"
  local tmp_db="${db}.tmp"
  for u in $urls; do
    echo "Downloading assets.db from $u..."
    rm -f "$tmp_db"
    if curl -fL --progress-bar "$u" -o "$tmp_db"; then
      if python3 - "$manifest" "$tmp_db" "$db" "$ver_file" <<'PY'
import hashlib, json, os, shutil, sys
m = json.load(open(sys.argv[1]))
tmp_db, db, vf = sys.argv[2], sys.argv[3], sys.argv[4]
expected = str(m.get('sha256', '')).lower()
h = hashlib.sha256()
with open(tmp_db, 'rb') as f:
    for chunk in iter(lambda: f.read(4 * 1024 * 1024), b''):
        h.update(chunk)
actual = h.hexdigest().lower()
if expected and actual != expected:
    raise SystemExit(f"sha256 mismatch: {actual} != {expected}")
shutil.move(tmp_db, db)
st = os.stat(db)
json.dump({'version': str(m.get('version', '')), 'sha256': actual, 'size': st.st_size, 'mtime_ns': st.st_mtime_ns}, open(vf, 'w'))
PY
      then
        echo "assets.db ready (v${target_ver})"
        rm -f "$manifest_tmp"
        return 0
      fi
    fi
    rm -f "$tmp_db"
  done
  rm -f "$manifest_tmp"
  echo "Error: failed to download assets.db." >&2
  exit 1
}

ensure_assets_db
ensure_rust_and_build() {
  local bins=(
    "$ROOT_DIR/target/release/sonoro-aki-logic"
    "$ROOT_DIR/target/release/sonoro-cfgsv"
    "$ROOT_DIR/target/release/sonoro-loginsv"
    "$ROOT_DIR/target/release/sonoro-gamesv"
  )
  local missing=0
  for b in "${bins[@]}"; do
    if [[ ! -x "$b" ]]; then
      missing=1
      break
    fi
  done
  if [[ "$missing" -eq 0 ]]; then
    return 0
  fi

  if ! command -v cc >/dev/null 2>&1 && ! command -v gcc >/dev/null 2>&1; then
    echo "C compiler not found; installing build tools..."
    if command -v apt-get >/dev/null 2>&1; then
      apt-get update -qq && apt-get install -y build-essential curl pkg-config libssl-dev python3 openssl
    elif command -v dnf >/dev/null 2>&1; then
      dnf install -y gcc gcc-c++ make curl pkgconfig openssl-devel python3 openssl
    elif command -v yum >/dev/null 2>&1; then
      yum install -y gcc gcc-c++ make curl pkgconfig openssl-devel python3 openssl
    fi
  fi

  if [[ -f "$HOME/.cargo/env" ]]; then
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env"
  fi
  if ! command -v cargo >/dev/null 2>&1 || ! command -v rustc >/dev/null 2>&1; then
    echo "Rust toolchain not found; installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env"
  fi

  echo "Building Rust release binaries..."
  if [[ -x "$ROOT_DIR/cargo-safe.sh" ]]; then
    "$ROOT_DIR/cargo-safe.sh" build --release --workspace
  else
    cargo build --release --workspace
  fi
}

find_server_manager() {
  for p in \
    "$ROOT_DIR/proxy/server_manager.py" \
    "$ROOT_DIR/server_manager.py" \
    "/root/proxy/server_manager.py" \
    "/root/server_manager.py"; do
    if [[ -f "$p" ]]; then
      echo "$p"
      return 0
    fi
  done
  return 1
}

ensure_assets_db
ensure_rust_and_build

"$ROOT_DIR/stop-private-server.sh" >/dev/null 2>&1 || true

rm -f /tmp/aki_logic.sock

nohup "$ROOT_DIR/target/release/sonoro-aki-logic" \
  >/tmp/sonoro-aki-logic.log 2>&1 &
echo $! >/tmp/sonoro-aki-logic.pid

for _ in $(seq 1 40); do
  if [[ -S /tmp/aki_logic.sock ]]; then
    break
  fi
  sleep 0.25
done

nohup "$ROOT_DIR/target/release/sonoro-cfgsv" \
  -s "$STATE_DIR" \
  -l "${BIND_HOST}:${CFG_PORT}" \
  >/tmp/sonoro-cfgsv.log 2>&1 &
echo $! >/tmp/sonoro-cfgsv.pid

nohup "$ROOT_DIR/target/release/sonoro-loginsv" \
  -s "$STATE_DIR" \
  -l "${BIND_HOST}:10001" \
  -g "${PUBLIC_HOST}:13100" \
  >/tmp/sonoro-loginsv.log 2>&1 &
echo $! >/tmp/sonoro-loginsv.pid

nohup "$ROOT_DIR/target/release/sonoro-gamesv" \
  -s "$STATE_DIR" \
  -b "${BIND_HOST}:13100" \
  -g "127.0.0.1:13101" \
  >/tmp/sonoro-gamesv.log 2>&1 &
echo $! >/tmp/sonoro-gamesv.pid

if [[ "$WITH_MANAGER" -eq 1 ]]; then
  MGR_PATH="$(find_server_manager || true)"
  if [[ -n "$MGR_PATH" ]] && command -v python3 >/dev/null 2>&1; then
    GAMESV_STATE_DIR="$ROOT_DIR/$STATE_DIR" nohup python3 "$MGR_PATH" \
      --host "$PUBLIC_HOST" \
      --login-port 5500 \
      --proxy-port 8080 \
      --socks-port 1080 \
      --gateway-port 13100 \
      --loginsv-url "http://127.0.0.1:10001" \
      --gm-url "http://127.0.0.1:13101" \
      >/tmp/sonoro-server-manager.log 2>&1 &
    echo $! >/tmp/sonoro-server-manager.pid
  else
    echo "Warning: server_manager.py or python3 not found; skipping proxy manager." >&2
  fi
fi

for _ in $(seq 1 60); do
  if curl -fsS "http://127.0.0.1:13101/health" >/dev/null 2>&1 && curl -fsS "http://127.0.0.1:10001/health" >/dev/null 2>&1; then
    break
  fi
  sleep 0.5
done

echo "Private server started:"
echo "  aki-logic : /tmp/aki_logic.sock + 127.0.0.1:19999 (pid $(cat /tmp/sonoro-aki-logic.pid))"
echo "  cfgsv     : http://${PUBLIC_HOST}:${CFG_PORT} (pid $(cat /tmp/sonoro-cfgsv.pid))"
echo "  loginsv   : http://${PUBLIC_HOST}:10001 (pid $(cat /tmp/sonoro-loginsv.pid))"
echo "  gamesv    : ${PUBLIC_HOST}:13100 (UDP KCP) / http://127.0.0.1:13101/gm (pid $(cat /tmp/sonoro-gamesv.pid))"
if [[ -f /tmp/sonoro-server-manager.pid ]]; then
  echo "  manager   : ${PUBLIC_HOST}:5500 (login/cfg) / :8080 (http/https proxy) / :1080 (socks5) (pid $(cat /tmp/sonoro-server-manager.pid))"
fi
