# SonoroServer

Wuthering Waves 3.8 server emulator in Rust.

## Client (Windows)

Run `launch.bat`, choose `1. Connect Server`, and enter the server IP.
No Rust, compiler, or `assets.db` required.

## Client (Android)

Coming soon.

## Host (Windows)

Put `launch.bat` in an empty folder or the repo folder, run it, and choose `2. Local Server` (`Solo` or `Co-op`).
Downloads server files, `assets.db`, MSVC Build Tools, Rust, and Python if missing.

## Host (Linux)

```sh
./start-private-server.sh --host 0.0.0.0 --public-host <IP>
./stop-private-server.sh
```

## Ports

- `5500/tcp` login / config
- `8080/tcp` proxy
- `13100/udp` game (KCP)
- `1080/tcp+udp` SOCKS5 (optional)
