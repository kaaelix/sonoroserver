//! Entry point for `sonoro-loginsv`, the HTTP login & gateway discovery server.

mod account;
mod serve;

use std::net::SocketAddr;
use std::process::ExitCode;
use std::sync::Arc;

use sonoro_common::FileSystem;
use sonoro_common::args::{FlagSpec, format_usage, parse_flags};
use sonoro_common::signal::shutdown_signal;
use tracing_subscriber::EnvFilter;

use crate::serve::{AppState, ClientGatewayConfig};

const FLAG_SPECS: &[FlagSpec] = &[
    FlagSpec {
        flag: 's',
        name: "state_dir",
    },
    FlagSpec {
        flag: 'l',
        name: "listen_address",
    },
    FlagSpec {
        flag: 'g',
        name: "gateway_address",
    },
];

struct Args {
    state_dir: String,
    listen_address: String,
    gateway_address: String,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            state_dir: String::from("state"),
            listen_address: String::from("127.0.0.1:10001"),
            gateway_address: String::from("127.0.0.1:13100"),
        }
    }
}

fn parse_gateway_address(raw: &str) -> Result<ClientGatewayConfig, &'static str> {
    let (host, port_str) = raw.rsplit_once(':').ok_or("missing port")?;
    if host.is_empty() {
        return Err("missing host");
    }
    let port: u16 = port_str.parse().map_err(|_| "invalid port")?;
    Ok(ClientGatewayConfig {
        host: host.to_string(),
        port,
    })
}

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let raw_args: Vec<String> = std::env::args().collect();
    let program = raw_args
        .first()
        .map(String::as_str)
        .unwrap_or("sonoro-loginsv");

    let mut args = Args::default();
    if !parse_flags(&raw_args[1..], FLAG_SPECS, |flag, val| {
        match flag {
            's' => args.state_dir = val.to_string(),
            'l' => args.listen_address = val.to_string(),
            'g' => args.gateway_address = val.to_string(),
            _ => return false,
        }
        true
    }) {
        eprintln!("{}", format_usage(program, FLAG_SPECS));
        return ExitCode::from(1);
    }

    let listen_address: SocketAddr = match args.listen_address.parse() {
        Ok(addr) => addr,
        Err(err) => {
            tracing::error!(listen_address = %args.listen_address, error = %err, "invalid listen address specified");
            return ExitCode::from(1);
        }
    };

    let gateway = match parse_gateway_address(&args.gateway_address) {
        Ok(gw) => gw,
        Err(err) => {
            tracing::error!(gateway_address = %args.gateway_address, error = err, "invalid gateway address specified");
            return ExitCode::from(1);
        }
    };

    let fs = match FileSystem::open(&args.state_dir) {
        Ok(fs) => Arc::new(fs),
        Err(err) => {
            tracing::error!(state_dir = %args.state_dir, error = %err, "failed to open filesystem");
            return ExitCode::from(1);
        }
    };

    let listener = match tokio::net::TcpListener::bind(listen_address).await {
        Ok(listener) => listener,
        Err(err) => {
            tracing::error!(%listen_address, error = %err, "failed to listen");
            if err.kind() == std::io::ErrorKind::AddrInUse {
                tracing::error!("another instance of this server might be already running");
            }
            return ExitCode::from(1);
        }
    };

    tracing::info!(%listen_address, "login server is listening");

    let app = serve::router(AppState { fs, gateway });
    if let Err(err) = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        tracing::error!(error = %err, "server error");
        return ExitCode::from(1);
    }

    tracing::info!("shutting down...");
    ExitCode::SUCCESS
}
