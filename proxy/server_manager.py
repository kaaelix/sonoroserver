import argparse
import base64
from http import HTTPStatus
import ipaddress
import json
import os
import random
import select
import socket
import ssl
import struct
import subprocess
import sys
import threading
import time
import urllib.error
import urllib.request
import uuid
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import parse_qs, urlencode, urlparse


class _SafeStream:
    """Prevent closed-PTY EIO/EBADF/BrokenPipeError from crashing server threads."""

    def __init__(self, wrapped):
        self._wrapped = wrapped

    def write(self, data):
        try:
            return self._wrapped.write(data)
        except (OSError, ValueError):
            return len(data)

    def flush(self):
        try:
            self._wrapped.flush()
        except (OSError, ValueError):
            pass

    def __getattr__(self, name):
        return getattr(self._wrapped, name)


if not isinstance(sys.stdout, _SafeStream):
    sys.stdout = _SafeStream(sys.stdout)
if not isinstance(sys.stderr, _SafeStream):
    sys.stderr = _SafeStream(sys.stderr)

BASE_DIR = os.path.dirname(os.path.realpath(__file__))
PAYLOAD_PATH = os.path.join(BASE_DIR, "encrypted_3.8_index.b64")
MODIFIED_INDEX_PATH = os.path.join(BASE_DIR, "modified_3.8_index.json")
CERT_FILE = os.path.join(BASE_DIR, "server.crt")
KEY_FILE = os.path.join(BASE_DIR, "server.key")

AES_KEY_HEX = "b7e004bb9486769cf4eada26a276a32cc6aef402609b24ef561cfd56d19fd7ed"
AES_IV_HEX = "7e9adce65056003401eed8a6d11d89c5"

if os.path.exists(PAYLOAD_PATH):
    with open(PAYLOAD_PATH, "rb") as f:
        DEFAULT_ENCRYPTED_PAYLOAD = f.read().strip()
else:
    DEFAULT_ENCRYPTED_PAYLOAD = b""

if os.path.exists(MODIFIED_INDEX_PATH):
    with open(MODIFIED_INDEX_PATH, "r", encoding="utf-8") as f:
        MODIFIED_INDEX_TEMPLATE = f.read()
else:
    MODIFIED_INDEX_TEMPLATE = ""

OVERRIDE_HOST = os.environ.get("PUBLIC_HOST", "").strip() or None
LOGINSV_UPSTREAM = os.environ.get("LOGINSV_URL", "http://127.0.0.1:10001").rstrip("/")
GAMESV_GM_UPSTREAM = os.environ.get("GAMESV_GM_URL", "http://127.0.0.1:13101").rstrip("/")
LOGIN_PORT = 5500
PROXY_PORT = 8080
SOCKS_PORT = 1080
GATEWAY_PORT = 13100

_PAYLOAD_CACHE: dict[str, bytes] = {}
_PAYLOAD_LOCK = threading.Lock()
_ACCOUNT_SEX_CACHE: dict[str, int] = {"597938510": 1}
_ACCOUNT_CACHE_LOCK = threading.Lock()


def _sync_loginsv_async(target_url: str, user_id: str) -> None:
    try:
        with urllib.request.urlopen(target_url, timeout=2) as sv_resp:
            sv_data = json.loads(sv_resp.read().decode("utf-8"))
            sex = sv_data.get("sex", -1)
            if sex is not None and sex >= 0:
                with _ACCOUNT_CACHE_LOCK:
                    _ACCOUNT_SEX_CACHE[user_id] = sex
    except Exception:
        pass


def get_encrypted_payload(target_host: str) -> bytes:
    if target_host == "127.0.0.1" and LOGIN_PORT == 5500 and DEFAULT_ENCRYPTED_PAYLOAD:
        return DEFAULT_ENCRYPTED_PAYLOAD
    with _PAYLOAD_LOCK:
        cached = _PAYLOAD_CACHE.get(target_host)
        if cached is not None:
            return cached
        if not MODIFIED_INDEX_TEMPLATE:
            return DEFAULT_ENCRYPTED_PAYLOAD
        rendered = MODIFIED_INDEX_TEMPLATE.replace("127.0.0.1", target_host)
        if LOGIN_PORT != 5500:
            rendered = rendered.replace(":5500", f":{LOGIN_PORT}")
        proc = subprocess.run(
            ["openssl", "enc", "-aes-256-cbc", "-e", "-K", AES_KEY_HEX, "-iv", AES_IV_HEX],
            input=rendered.encode("utf-8"),
            capture_output=True,
            check=True,
        )
        encoded = base64.b64encode(proc.stdout)
        _PAYLOAD_CACHE[target_host] = encoded
        return encoded


def resolve_target_host(handler: BaseHTTPRequestHandler) -> str:
    if OVERRIDE_HOST:
        return OVERRIDE_HOST
    if hasattr(handler, "headers") and handler.headers:
        raw_h = handler.headers.get("Host", "")
        if raw_h:
            req_host = raw_h.split(":")[0].strip()
            if req_host and req_host not in ("", "0.0.0.0", "::", "::1", "localhost"):
                return req_host
    host = handler.connection.getsockname()[0]
    if host.startswith("::ffff:"):
        host = host[7:]
    if host in ("", "0.0.0.0", "::", "::1", "127.0.0.1"):
        return "43.167.31.150"
    return host


def _ensure_tls_cert() -> None:
    if os.path.exists(CERT_FILE) and os.path.exists(KEY_FILE):
        return
    subprocess.run(
        [
            "openssl", "req", "-x509", "-newkey", "rsa:2048", "-nodes",
            "-keyout", KEY_FILE, "-out", CERT_FILE, "-days", "3650",
            "-subj", "/CN=*.aki-game.net",
            "-addext", "subjectAltName=DNS:*.aki-game.net,DNS:*.aki-game.com,DNS:*.kurogame.com,DNS:localhost",
        ],
        check=True,
        capture_output=True,
    )


_ensure_tls_cert()
ssl_ctx = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
ssl_ctx.load_cert_chain(certfile=CERT_FILE, keyfile=KEY_FILE)


def _configure_tcp_socket(sock: socket.socket) -> None:
    try:
        sock.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
    except Exception:
        pass
    try:
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_RCVBUF, 2 * 1024 * 1024)
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_SNDBUF, 2 * 1024 * 1024)
    except Exception:
        pass


def _configure_udp_socket(sock: socket.socket) -> None:
    try:
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_RCVBUF, 4 * 1024 * 1024)
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_SNDBUF, 4 * 1024 * 1024)
    except Exception:
        pass


class ReusableThreadingHTTPServer(ThreadingHTTPServer):
    allow_reuse_address = True
    daemon_threads = True
    request_queue_size = 256

    def get_request(self):
        conn, addr = super().get_request()
        _configure_tcp_socket(conn)
        return conn, addr

    def handle_error(self, request, client_address):
        _, exc, _ = sys.exc_info()
        if isinstance(exc, (BrokenPipeError, ConnectionResetError, ConnectionAbortedError, TimeoutError)):
            return
        try:
            print(f"http: unhandled error from {client_address}: {exc}")
        except Exception:
            pass


MAX_REQUEST_BODY_BYTES = 16 * 1024 * 1024


def _parse_content_length(headers) -> int:
    raw = headers.get("Content-Length", "0") if headers else "0"
    try:
        val = int(str(raw).strip() or "0")
    except ValueError:
        return 0
    if val <= 0:
        return 0
    return min(val, MAX_REQUEST_BODY_BYTES)


def _proxy_to_gm_upstream(
    method: str,
    path_and_query: str,
    body: bytes | None,
    content_type: str | None,
    extra_headers: dict[str, str] | None = None,
) -> tuple[int, str, bytes]:
    url = f"{GAMESV_GM_UPSTREAM}{path_and_query}"
    headers = {}
    if content_type:
        headers["Content-Type"] = content_type
    if extra_headers:
        for k, v in extra_headers.items():
            if v:
                headers[k] = v
    req = urllib.request.Request(url, data=body, headers=headers, method=method)
    try:
        with urllib.request.urlopen(req, timeout=10) as resp:
            ctype = resp.headers.get("Content-Type", "application/json; charset=utf-8")
            return resp.status, ctype, resp.read()
    except urllib.error.HTTPError as e:
        ctype = e.headers.get("Content-Type", "application/json; charset=utf-8") if e.headers else "application/json"
        return e.code, ctype, e.read()
    except Exception as ex:
        err = json.dumps({"ok": False, "online": False, "message": f"gm upstream unreachable: {ex}"}).encode("utf-8")
        return 200, "application/json; charset=utf-8", err


def _build_notice_payload(path: str, base_origin: str) -> bytes | None:
    if path.endswith("entrypoint.json") or "/gamenotice/" in path:
        return json.dumps({
            "h5AppUrl": [f"{base_origin}/gm"],
            "contentUrl": [f"{base_origin}/gm-notice/notice.json"],
            "apiUrl": f"{base_origin}/gm-notice",
            "apiUrls": [f"{base_origin}/gm-notice"],
        }).encode("utf-8")
    if path.endswith("notice.json"):
        return json.dumps({
            "game": [{
                "id": "cheat_code_notice",
                "red": False,
                "platform": [0, 1, 2, 3, 4, 5, 6],
                "channel": ["0", "1", "2", "3", "18", "19"],
                "whiteList": [],
                "startTimeMs": 0,
                "endTimeMs": 4102444800000,
                "permanent": True,
            }],
            "activity": [],
        }).encode("utf-8")
    if "/notice/red-dot/get" in path:
        return b'{"code":200,"message":"ok","data":false}'
    if "/notice/read-ids/list" in path:
        return b'{"code":200,"message":"ok","data":[]}'
    if "loginNotice" in path or path.endswith("LoginNotice.json"):
        return json.dumps([{
            "Id": 1,
            "WhiteLists": [],
            "ModifyTime": 1,
            "BeginTime": 0,
            "EndTime": 4102444800,
            "Title": "GM Control Panel",
            "content": f"Open the in-game Announcement menu or visit {base_origin}/gm in your browser to use the GM Control Panel (Currencies, Level, Items, Characters, Quest Picker & Skip).",
        }]).encode("utf-8")
    return None


def _is_gm_ui_or_api_path(path: str) -> bool:
    return (
        path in ("/gm", "/gm/", "/announcement", "/announcement/", "/aki-logic/status", "/gm/auth")
        or path.startswith("/gm/state")
        or path.startswith("/gm/exec")
        or path.startswith("/gm/catalog")
        or path.startswith("/aki-logic/")
    )


def _extract_gm_headers(headers) -> dict[str, str]:
    out: dict[str, str] = {}
    if not headers:
        return out
    for k in ("X-GM-Sign", "X-GM-Admin"):
        v = headers.get(k)
        if v:
            out[k] = v
    return out


def _is_notice_path(path: str) -> bool:
    return (
        path.startswith("/gm-notice")
        or path.startswith("/gamenotice")
        or path.startswith("/notice/")
        or "/gm/loginNotice/" in path
        or path.endswith("LoginNotice.json")
    )


# Login & config server (default port 5500)
class LoginHandler(BaseHTTPRequestHandler):
    def log_message(self, format, *args):
        print(f"login:{LOGIN_PORT} {self.address_string()} {format % args}")

    def _send_bytes(self, status: int, content_type: str, body: bytes):
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)

    def do_OPTIONS(self):
        self._send_bytes(204, "text/plain", b"")

    def do_GET(self):
        parsed = urlparse(self.path)
        target_host = resolve_target_host(self)
        if parsed.path == "/health":
            self._send_bytes(200, "text/plain; charset=utf-8", b"OK")
            return
        if _is_gm_ui_or_api_path(parsed.path):
            status, ctype, body = _proxy_to_gm_upstream("GET", self.path, None, None, _extract_gm_headers(self.headers))
            self._send_bytes(status, ctype, body)
            return
        if _is_notice_path(parsed.path):
            payload = _build_notice_payload(parsed.path, f"http://{target_host}:{LOGIN_PORT}")
            if payload is not None:
                self._send_bytes(200, "application/json; charset=utf-8", payload)
                return
        if "index.json" in parsed.path:
            payload = get_encrypted_payload(target_host)
            print(f"login:{LOGIN_PORT} index.json (host={target_host}, {len(payload)} bytes) -> {self.address_string()}")
            self._send_bytes(200, "application/json", payload)
        elif parsed.path.startswith("/api/getResourcePackageInfo"):
            qs = parse_qs(parsed.query)
            uid = qs.get("userId", [""])[0]
            if not uid:
                uid = str(random.randint(10000000, 99999999))
            sex = 0
            finished_quests = []
            need_confirm_quests = []
            state_root = os.environ.get("GAMESV_STATE_DIR", "/root/wuwa-server/state")
            pdir = os.path.join(state_root, "player", uid)
            try:
                basic_p = os.path.join(pdir, "basic_info")
                if os.path.exists(basic_p):
                    import re as _re
                    m_sex = _re.search(r"\.sex\s*=\s*(\d+)", open(basic_p, "r", encoding="utf-8").read())
                    if m_sex:
                        sex = int(m_sex.group(1))
                quest_p = os.path.join(pdir, "quest_info")
                if os.path.exists(quest_p):
                    import re as _re
                    qtxt = open(quest_p, "r", encoding="utf-8").read()
                    mf = _re.search(r"\.finished_quests\s*=\s*\.\{(.*?)\}", qtxt, _re.S)
                    if mf:
                        finished_quests = [int(x.strip()) for x in mf.group(1).split(",") if x.strip().isdigit()]
                prog_p = os.path.join(pdir, "progress_info")
                if os.path.exists(prog_p):
                    import re as _re
                    ptxt = open(prog_p, "r", encoding="utf-8").read()
                    mm = _re.search(r'\.key\s*=\s*"quest:missing_res",\s*\.val\s*=\s*"([^"]*)"', ptxt, _re.S)
                    if mm and mm.group(1):
                        need_confirm_quests = [int(x.strip()) for x in mm.group(1).split(",") if x.strip().isdigit()]
            except Exception as exc:
                print(f"login:{LOGIN_PORT} getResourcePackageInfo read error ({uid}): {exc}")
            resp_pkg = {
                "code": 0,
                "userId": uid,
                "needConfirmQuestIdSet": need_confirm_quests,
                "finishQuestIdSet": finished_quests,
                "currentBlockIdSet": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                "sex": sex,
                "curRecallId": 0,
                "mp4FinishQuestFlag": [],
            }
            body = json.dumps(resp_pkg).encode("utf-8")
            print(f"login:{LOGIN_PORT} getResourcePackageInfo uid={uid} sex={sex} finished={len(finished_quests)} missing={len(need_confirm_quests)}")
            self._send_bytes(200, "application/json", body)
        elif parsed.path.startswith("/api/login"):
            try:
                qs = parse_qs(parsed.query)
                try:
                    user_data = int(qs.get("userData", [0])[0])
                except (ValueError, TypeError):
                    user_data = 0
                user_id = qs.get("userId", [""])[0]
                user_name = qs.get("userName", [""])[0]
                if not user_id:
                    user_id = str(random.randint(10000000, 99999999))
                if not user_name:
                    user_name = user_id

                token = qs.get("token", [""])[0]
                if not token:
                    token = f"00-{uuid.uuid4().hex[:32]}-{uuid.uuid4().hex[:16]}-00"

                query_str = urlencode({
                    "loginType": 1,
                    "userId": user_id,
                    "userName": user_name,
                    "token": token,
                    "userData": user_data,
                })
                target_url = f"{LOGINSV_UPSTREAM}/api/login?{query_str}"
                with _ACCOUNT_CACHE_LOCK:
                    cached_sex = _ACCOUNT_SEX_CACHE.get(user_id)
                account_sex = cached_sex if cached_sex is not None else -1
                sv_code = 0
                sv_extra: dict = {}
                if cached_sex is not None:
                    threading.Thread(
                        target=_sync_loginsv_async,
                        args=(target_url, user_id),
                        daemon=True,
                    ).start()
                else:
                    print(f"login:{LOGIN_PORT} sync loginsv: {target_url}")
                    try:
                        with urllib.request.urlopen(target_url, timeout=1.5) as sv_resp:
                            sv_data = json.loads(sv_resp.read().decode("utf-8"))
                            sv_code = sv_data.get("code", 0)
                            account_sex = sv_data.get("sex", -1)
                            if account_sex is None:
                                account_sex = -1
                            if account_sex >= 0:
                                with _ACCOUNT_CACHE_LOCK:
                                    _ACCOUNT_SEX_CACHE[user_id] = account_sex
                            if sv_code != 0:
                                for k in ("errMessage", "banTimeStamp"):
                                    if sv_data.get(k) is not None:
                                        sv_extra[k] = sv_data[k]
                    except Exception as ex:
                        print(f"login:{LOGIN_PORT} loginsv error: {ex}")

                resp_data = {
                    "code": sv_code,
                    "token": token,
                    "udpHosts": [{"host": target_host, "port": GATEWAY_PORT}],
                    "tcpHosts": [{"host": target_host, "port": GATEWAY_PORT}],
                    "tcpRatio": 0,
                    "userData": user_data,
                    "sex": account_sex,
                    "gatewayLatencyConfig": {
                        "enabled": False,
                        "count": 1,
                        "timeoutMs": 3000,
                    },
                    **sv_extra,
                }
                body = json.dumps(resp_data).encode("utf-8")
                print(f"login:{LOGIN_PORT} login uid={user_id} host={target_host}:{GATEWAY_PORT} sex={account_sex}")
                self._send_bytes(200, "application/json", body)
            except Exception as e:
                print(f"login:{LOGIN_PORT} login error: {e}")
                self._send_bytes(500, "application/json", b'{"code":500,"msg":"internal error"}')
        else:
            self._send_bytes(200, "application/json", b'{"code": 0, "msg": "success"}')

    def do_POST(self):
        parsed = urlparse(self.path)
        target_host = resolve_target_host(self)
        if _is_gm_ui_or_api_path(parsed.path):
            content_length = _parse_content_length(self.headers)
            post_body = self.rfile.read(content_length) if content_length > 0 else b""
            status, ctype, body = _proxy_to_gm_upstream("POST", self.path, post_body, self.headers.get("Content-Type", "application/json"), _extract_gm_headers(self.headers))
            self._send_bytes(status, ctype, body)
            return
        if _is_notice_path(parsed.path):
            self._send_bytes(200, "application/json; charset=utf-8", b'{"code":200,"message":"ok","data":true}')
            return
        if parsed.path.startswith("/api/login"):
            try:
                target_url = f"{LOGINSV_UPSTREAM}{self.path}"
                content_length = _parse_content_length(self.headers)
                post_body = self.rfile.read(content_length) if content_length > 0 else None
                req = urllib.request.Request(target_url, data=post_body, method="POST")
                with urllib.request.urlopen(req, timeout=5) as resp:
                    resp_data = resp.read()
                    data = json.loads(resp_data.decode("utf-8"))
                    data["udpHosts"] = [{"host": target_host, "port": GATEWAY_PORT}]
                    data["tcpHosts"] = [{"host": target_host, "port": GATEWAY_PORT}]
                    data["tcpRatio"] = 0
                    if data.get("sex") is None:
                        data["sex"] = -1
                    data["gatewayLatencyConfig"] = {"enabled": False, "count": 1, "timeoutMs": 3000}
                    enriched = json.dumps(data).encode("utf-8")
                    self._send_bytes(resp.status, "application/json", enriched)
            except Exception as e:
                print(f"login:{LOGIN_PORT} post login error: {e}")
                self._send_bytes(200, "application/json", b'{"code": 0}')
        else:
            self._send_bytes(200, "application/json", b'{"code": 0, "msg": "success"}')


# HTTP/HTTPS proxy & TLS MITM (default port 8080)
def _pump_half_duplex(src: socket.socket, dst: socket.socket) -> None:
    buf = bytearray(262144)
    view = memoryview(buf)
    try:
        while True:
            n = src.recv_into(view)
            if not n:
                break
            dst.sendall(view[:n])
    except Exception:
        pass
    finally:
        try:
            dst.shutdown(socket.SHUT_WR)
        except Exception:
            pass


def _tunnel_stream(client: socket.socket, target: socket.socket) -> None:
    try:
        _configure_tcp_socket(client)
        _configure_tcp_socket(target)
        client.settimeout(120)
        target.settimeout(120)
        up = threading.Thread(target=_pump_half_duplex, args=(client, target), daemon=True)
        up.start()
        _pump_half_duplex(target, client)
        up.join(timeout=5)
    except Exception:
        pass


def serve_aki_config_mitm(client_sock: socket.socket, host: str, target_host: str, send_http_200: bool) -> None:
    print(f"proxy: tls mitm {host} (target={target_host})")
    if send_http_200:
        try:
            client_sock.sendall(b"HTTP/1.1 200 Connection Established\r\n\r\n")
        except Exception as e:
            print(f"proxy: connect reply failed: {e}")
            return

    try:
        client_sock.settimeout(10)
        with ssl_ctx.wrap_socket(client_sock, server_side=True) as ssl_conn:
            buf = b""
            while b"\r\n\r\n" not in buf and len(buf) < 65536:
                chunk = ssl_conn.recv(4096)
                if not chunk:
                    break
                buf += chunk
            req = buf.decode("utf-8", errors="ignore")
            first_line = req.splitlines()[0] if req else "empty"
            print(f"proxy: tls req {first_line}")

            payload = get_encrypted_payload(target_host)
            response_headers = (
                "HTTP/1.1 200 OK\r\n"
                "Content-Type: application/json\r\n"
                f"Content-Length: {len(payload)}\r\n"
                "Connection: close\r\n"
                "\r\n"
            ).encode("utf-8")
            ssl_conn.sendall(response_headers + payload)
            print(f"proxy: sent index.json ({target_host})")
    except ssl.SSLError as e:
        print(f"proxy: tls handshake error: {e}")
    except Exception as e:
        print(f"proxy: tls error: {e}")


def serve_aki_notice_mitm(client_sock: socket.socket, host: str, send_http_200: bool) -> None:
    if send_http_200:
        try:
            client_sock.sendall(b"HTTP/1.1 200 Connection Established\r\n\r\n")
        except Exception:
            return
    try:
        with ssl_ctx.wrap_socket(client_sock, server_side=True) as ssl_conn:
            buf = b""
            while b"\r\n\r\n" not in buf and len(buf) < 65536:
                chunk = ssl_conn.recv(4096)
                if not chunk:
                    break
                buf += chunk
            if b"\r\n\r\n" not in buf:
                return
            hdr_raw, rest = buf.split(b"\r\n\r\n", 1)
            lines = hdr_raw.decode("utf-8", errors="ignore").split("\r\n")
            req_parts = lines[0].split()
            method = req_parts[0] if len(req_parts) >= 1 else "GET"
            raw_target = req_parts[1] if len(req_parts) >= 2 else "/"
            content_len = 0
            ctype_in = "application/json"
            gm_hdrs: dict[str, str] = {}
            for ln in lines[1:]:
                if ":" in ln:
                    k, v = ln.split(":", 1)
                    kl = k.strip().lower()
                    if kl == "content-length":
                        try:
                            content_len = max(0, min(int(v.strip() or "0"), MAX_REQUEST_BODY_BYTES))
                        except ValueError:
                            content_len = 0
                    elif kl == "content-type":
                        ctype_in = v.strip()
                    elif kl == "x-gm-sign":
                        gm_hdrs["X-GM-Sign"] = v.strip()
                    elif kl == "x-gm-admin":
                        gm_hdrs["X-GM-Admin"] = v.strip()
            body = rest
            while len(body) < content_len:
                more = ssl_conn.recv(min(65536, content_len - len(body)))
                if not more:
                    break
                body += more
            body = body[:content_len] if content_len > 0 else b""

            parsed = urlparse(raw_target)
            base_origin = f"https://{host}"
            if method == "OPTIONS":
                status, ctype_out, resp_body = 204, "text/plain", b""
            elif _is_gm_ui_or_api_path(parsed.path):
                status, ctype_out, resp_body = _proxy_to_gm_upstream(method, raw_target, body if method == "POST" else None, ctype_in, gm_hdrs)
            else:
                notice_body = _build_notice_payload(parsed.path, base_origin)
                status = 200
                ctype_out = "application/json; charset=utf-8"
                resp_body = notice_body if notice_body is not None else b'{"code":200,"message":"ok"}'

            phrase = HTTPStatus(status).phrase if status in HTTPStatus._value2member_map_ else "OK"
            headers = (
                f"HTTP/1.1 {status} {phrase}\r\n"
                f"Content-Type: {ctype_out}\r\n"
                f"Content-Length: {len(resp_body)}\r\n"
                "Access-Control-Allow-Origin: *\r\n"
                "Access-Control-Allow-Headers: Content-Type, X-GM-Sign, X-GM-Admin\r\n"
                "Cache-Control: no-store\r\n"
                "Connection: close\r\n\r\n"
            ).encode("utf-8")
            ssl_conn.sendall(headers + resp_body)
    except Exception as e:
        print(f"proxy: notice mitm error ({host}): {e}")


class ProxyHandler(BaseHTTPRequestHandler):
    def log_message(self, format, *args):
        pass

    def do_GET(self):
        self._forward_http()

    def do_POST(self):
        self._forward_http()

    def _forward_http(self):
        parsed = urlparse(self.path)
        target_host = resolve_target_host(self)
        if "index.json" in parsed.path and "aki-config" in self.path:
            payload = get_encrypted_payload(target_host)
            print(f"proxy:{PROXY_PORT} index.json (host={target_host}) {self.path}")
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)
            return

        if "aki-gm-resources-back" in self.path or _is_notice_path(parsed.path) or _is_gm_ui_or_api_path(parsed.path):
            req_path = parsed.path + (f"?{parsed.query}" if parsed.query else "")
            if _is_gm_ui_or_api_path(parsed.path):
                cl = _parse_content_length(self.headers)
                req_body = self.rfile.read(cl) if cl > 0 else None
                status, ctype, resp_body = _proxy_to_gm_upstream(self.command, req_path, req_body if self.command == "POST" else None, self.headers.get("Content-Type"), _extract_gm_headers(self.headers))
            else:
                status = 200
                ctype = "application/json; charset=utf-8"
                resp_body = _build_notice_payload(parsed.path, f"http://{target_host}:{LOGIN_PORT}") or b'{"code":200,"message":"ok"}'
            self.send_response(status)
            self.send_header("Content-Type", ctype)
            self.send_header("Content-Length", str(len(resp_body)))
            self.end_headers()
            self.wfile.write(resp_body)
            return

        url = self.path
        if not url.startswith("http://") and not url.startswith("https://"):
            host_hdr = self.headers.get("Host", "127.0.0.1")
            url = f"http://{host_hdr}{self.path}"

        try:
            content_length = _parse_content_length(self.headers)
            body = self.rfile.read(content_length) if content_length > 0 else None

            headers = {
                k: v
                for k, v in self.headers.items()
                if k.lower() not in ("host", "content-length", "proxy-connection")
            }
            req = urllib.request.Request(url, data=body, headers=headers, method=self.command)

            with urllib.request.urlopen(req, timeout=30) as resp:
                self.send_response(resp.status)
                has_cl = False
                for k, v in resp.getheaders():
                    kl = k.lower()
                    if kl == "content-length":
                        has_cl = True
                        self.send_header(k, v)
                    elif kl not in ("transfer-encoding", "connection"):
                        self.send_header(k, v)
                if not has_cl:
                    resp_body = resp.read()
                    self.send_header("Content-Length", str(len(resp_body)))
                    self.end_headers()
                    self.wfile.write(resp_body)
                else:
                    self.end_headers()
                    while True:
                        chunk = resp.read(262144)
                        if not chunk:
                            break
                        self.wfile.write(chunk)
        except urllib.error.HTTPError as e:
            try:
                resp_body = e.read()
                self.send_response(e.code)
                for k, v in e.headers.items():
                    if k.lower() not in ("transfer-encoding", "content-length", "connection"):
                        self.send_header(k, v)
                self.send_header("Content-Length", str(len(resp_body)))
                self.end_headers()
                self.wfile.write(resp_body)
            except Exception:
                self.send_response(e.code)
                self.end_headers()
        except Exception:
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.end_headers()
            self.wfile.write(b'{"code":0}')

    def do_CONNECT(self):
        try:
            host, port_str = self.path.rsplit(":", 1)
            host = host.strip("[]")
            port = int(port_str)
        except Exception:
            host = self.path.strip("[]")
            port = 443

        target_host = resolve_target_host(self)

        # TLS MITM only for Kuro config/notice domains; tunnel CDN directly.
        if "aki-config" in host:
            serve_aki_config_mitm(self.connection, host, target_host, send_http_200=True)
            return
        if "aki-gm-resources-back" in host:
            serve_aki_notice_mitm(self.connection, host, send_http_200=True)
            return

        target = None
        try:
            target = socket.create_connection((host, port), timeout=10)
            self.connection.sendall(b"HTTP/1.1 200 Connection Established\r\n\r\n")
            _tunnel_stream(self.connection, target)
        except Exception:
            pass
        finally:
            if target:
                try:
                    target.close()
                except Exception:
                    pass


# SOCKS5 proxy (TCP CONNECT + UDP ASSOCIATE, default port 1080)
def _recv_exact(sock: socket.socket, n: int) -> bytes:
    buf = bytearray()
    while len(buf) < n:
        chunk = sock.recv(n - len(buf))
        if not chunk:
            raise ConnectionError("EOF")
        buf.extend(chunk)
    return bytes(buf)


def _parse_socks_addr(buf: bytes, offset: int) -> tuple[str, int, int]:
    atyp = buf[offset]
    offset += 1
    if atyp == 0x01:
        host = socket.inet_ntoa(buf[offset : offset + 4])
        offset += 4
    elif atyp == 0x03:
        dlen = buf[offset]
        offset += 1
        host = buf[offset : offset + dlen].decode("utf-8", errors="ignore")
        offset += dlen
    elif atyp == 0x04:
        host = socket.inet_ntop(socket.AF_INET6, buf[offset : offset + 16])
        offset += 16
    else:
        raise ValueError(f"Unsupported ATYP {atyp}")
    port = struct.unpack("!H", buf[offset : offset + 2])[0]
    offset += 2
    return host, port, offset


_UDP_HDR_CACHE: dict[tuple[str, int], bytes] = {}


def _encode_socks_udp_header(host: str, port: int) -> bytes:
    key = (host, port)
    cached = _UDP_HDR_CACHE.get(key)
    if cached is not None:
        return cached
    try:
        hdr = b"\x00\x00\x00\x01" + socket.inet_aton(host) + struct.pack("!H", port)
    except OSError:
        try:
            ip = ipaddress.ip_address(host)
            if ip.version == 4:
                hdr = b"\x00\x00\x00\x01" + ip.packed + struct.pack("!H", port)
            else:
                hdr = b"\x00\x00\x00\x04" + ip.packed + struct.pack("!H", port)
        except ValueError:
            enc = host.encode("utf-8")[:255]
            hdr = b"\x00\x00\x00\x03" + bytes([len(enc)]) + enc + struct.pack("!H", port)
    if len(_UDP_HDR_CACHE) < 1024:
        _UDP_HDR_CACHE[key] = hdr
    return hdr


def _socks_reply(rep: int, bind_ip: str, bind_port: int) -> bytes:
    try:
        ip = ipaddress.ip_address(bind_ip)
        if ip.version == 4:
            return struct.pack("!BBBB", 5, rep, 0, 1) + ip.packed + struct.pack("!H", bind_port)
        return struct.pack("!BBBB", 5, rep, 0, 4) + ip.packed + struct.pack("!H", bind_port)
    except ValueError:
        return struct.pack("!BBBB", 5, rep, 0, 1) + b"\x00\x00\x00\x00" + struct.pack("!H", bind_port)


class Socks5UdpRelay:
    def __init__(self, port: int):
        self.port = port
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        _configure_udp_socket(self.sock)
        self.sock.bind(("0.0.0.0", port))
        self._lock = threading.Lock()
        self._sessions: dict[tuple[str, int], list] = {}

    def start(self):
        t = threading.Thread(target=self._run, daemon=True)
        t.start()
        t_clean = threading.Thread(target=self._cleanup_loop, daemon=True)
        t_clean.start()

    def _get_or_create_outbound(self, client_addr: tuple[str, int]) -> socket.socket:
        now = time.monotonic()
        entry = self._sessions.get(client_addr)
        if entry is not None:
            entry[1] = now
            return entry[0]
        with self._lock:
            entry = self._sessions.get(client_addr)
            if entry is not None:
                entry[1] = now
                return entry[0]
            out_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
            _configure_udp_socket(out_sock)
            entry = [out_sock, now]
            self._sessions[client_addr] = entry
            threading.Thread(
                target=self._pump_outbound,
                args=(client_addr, out_sock, entry),
                daemon=True,
            ).start()
            return out_sock

    def _pump_outbound(self, client_addr: tuple[str, int], out_sock: socket.socket, entry: list):
        out_sock.settimeout(180)
        try:
            while True:
                try:
                    data, remote_addr = out_sock.recvfrom(65535)
                except Exception:
                    break
                try:
                    header = _encode_socks_udp_header(remote_addr[0], remote_addr[1])
                    self.sock.sendto(header + data, client_addr)
                    entry[1] = time.monotonic()
                except Exception:
                    pass
        finally:
            with self._lock:
                cur = self._sessions.get(client_addr)
                if cur is not None and cur[0] is out_sock:
                    self._sessions.pop(client_addr, None)
            try:
                out_sock.close()
            except Exception:
                pass

    def _cleanup_loop(self):
        while True:
            time.sleep(30)
            cutoff = time.monotonic() - 180
            with self._lock:
                stale = [k for k, entry in self._sessions.items() if entry[1] < cutoff]
                for k in stale:
                    entry = self._sessions.pop(k)
                    try:
                        entry[0].close()
                    except Exception:
                        pass

    def _run(self):
        while True:
            try:
                packet, client_addr = self.sock.recvfrom(65535)
                if len(packet) < 10 or packet[2] != 0:
                    continue
                dst_host, dst_port, data_off = _parse_socks_addr(packet, 3)
                payload = packet[data_off:]
                if dst_port == GATEWAY_PORT:
                    dst_host = "127.0.0.1"
                out_sock = self._get_or_create_outbound(client_addr)
                out_sock.sendto(payload, (dst_host, dst_port))
            except Exception:
                pass


def _handle_socks5_client(conn: socket.socket, udp_port: int):
    try:
        _configure_tcp_socket(conn)
        ver_n = _recv_exact(conn, 2)
        if ver_n[0] != 5:
            return
        methods = _recv_exact(conn, ver_n[1])
        if 0 not in methods:
            conn.sendall(b"\x05\xff")
            return
        conn.sendall(b"\x05\x00")

        hdr = _recv_exact(conn, 4)
        ver, cmd, _, atyp = hdr
        if ver != 5:
            return
        if atyp == 0x01:
            addr_bytes = _recv_exact(conn, 4)
            dst_host = socket.inet_ntoa(addr_bytes)
        elif atyp == 0x03:
            dlen = _recv_exact(conn, 1)[0]
            dst_host = _recv_exact(conn, dlen).decode("utf-8", errors="ignore")
        elif atyp == 0x04:
            addr_bytes = _recv_exact(conn, 16)
            dst_host = socket.inet_ntop(socket.AF_INET6, addr_bytes)
        else:
            conn.sendall(_socks_reply(8, "0.0.0.0", 0))
            return
        dst_port = struct.unpack("!H", _recv_exact(conn, 2))[0]

        local_ip = conn.getsockname()[0]
        if local_ip.startswith("::ffff:"):
            local_ip = local_ip[7:]
        target_host = OVERRIDE_HOST or ("43.167.31.150" if local_ip in ("", "0.0.0.0", "::", "::1", "127.0.0.1") else local_ip)

        if cmd == 0x01:  # CONNECT (TCP)
            if "aki-config" in dst_host and dst_port == 443:
                conn.sendall(_socks_reply(0, local_ip, SOCKS_PORT))
                serve_aki_config_mitm(conn, dst_host, target_host, send_http_200=False)
                return
            if "aki-gm-resources-back" in dst_host and dst_port == 443:
                conn.sendall(_socks_reply(0, local_ip, SOCKS_PORT))
                serve_aki_notice_mitm(conn, dst_host, send_http_200=False)
                return

            remote = None
            try:
                remote = socket.create_connection((dst_host, dst_port), timeout=10)
                conn.sendall(_socks_reply(0, local_ip, SOCKS_PORT))
                _tunnel_stream(conn, remote)
            except Exception:
                try:
                    conn.sendall(_socks_reply(4, "0.0.0.0", 0))
                except Exception:
                    pass
            finally:
                if remote:
                    try:
                        remote.close()
                    except Exception:
                        pass
        elif cmd == 0x03:  # UDP ASSOCIATE
            conn.sendall(_socks_reply(0, local_ip, udp_port))
            while True:
                if not conn.recv(1024):
                    break
        else:
            conn.sendall(_socks_reply(7, "0.0.0.0", 0))
    except Exception:
        pass
    finally:
        try:
            conn.close()
        except Exception:
            pass


def _serve_socks5(port: int):
    udp_relay = Socks5UdpRelay(port)
    udp_relay.start()

    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(("0.0.0.0", port))
    srv.listen(256)
    while True:
        client, _ = srv.accept()
        threading.Thread(target=_handle_socks5_client, args=(client, port), daemon=True).start()


def run_servers():
    global OVERRIDE_HOST, LOGINSV_UPSTREAM, GAMESV_GM_UPSTREAM, LOGIN_PORT, PROXY_PORT, SOCKS_PORT, GATEWAY_PORT
    parser = argparse.ArgumentParser(description="Wuthering Waves 3.8 Proxy & Login Manager")
    parser.add_argument("--host", default=OVERRIDE_HOST, help="Override host IP returned to game client (auto-detects socket local IP if omitted)")
    parser.add_argument("--login-port", type=int, default=LOGIN_PORT)
    parser.add_argument("--proxy-port", type=int, default=PROXY_PORT)
    parser.add_argument("--socks-port", type=int, default=SOCKS_PORT)
    parser.add_argument("--gateway-port", type=int, default=GATEWAY_PORT)
    parser.add_argument("--loginsv-url", default=LOGINSV_UPSTREAM)
    parser.add_argument("--gm-url", default=GAMESV_GM_UPSTREAM)
    args = parser.parse_args()

    OVERRIDE_HOST = args.host.strip() if args.host else None
    LOGIN_PORT = args.login_port
    PROXY_PORT = args.proxy_port
    SOCKS_PORT = args.socks_port
    GATEWAY_PORT = args.gateway_port
    LOGINSV_UPSTREAM = args.loginsv_url.rstrip("/")
    GAMESV_GM_UPSTREAM = args.gm_url.rstrip("/")

    # Warm default payload + verify template
    for warm_host in ("127.0.0.1", "43.167.31.150"):
        try:
            _ = get_encrypted_payload(warm_host)
        except Exception:
            pass
    if OVERRIDE_HOST:
        _ = get_encrypted_payload(OVERRIDE_HOST)

    print(f"login/cfg : 0.0.0.0:{LOGIN_PORT}")
    print(f"proxy     : 0.0.0.0:{PROXY_PORT}")
    print(f"socks5    : 0.0.0.0:{SOCKS_PORT}")
    print(f"gateway   : {OVERRIDE_HOST or 'auto'}:{GATEWAY_PORT}")
    print(f"loginsv   : {LOGINSV_UPSTREAM}")
    print(f"tls cert  : {CERT_FILE}")

    login_srv = ReusableThreadingHTTPServer(("0.0.0.0", LOGIN_PORT), LoginHandler)
    t1 = threading.Thread(target=login_srv.serve_forever, daemon=True)
    t1.start()

    proxy_srv = ReusableThreadingHTTPServer(("0.0.0.0", PROXY_PORT), ProxyHandler)
    t2 = threading.Thread(target=proxy_srv.serve_forever, daemon=True)
    t2.start()

    t3 = threading.Thread(target=_serve_socks5, args=(SOCKS_PORT,), daemon=True)
    t3.start()

    try:
        t1.join()
    except KeyboardInterrupt:
        print("\nstopped.")


if __name__ == "__main__":
    run_servers()
