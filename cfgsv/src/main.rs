//! Entry point for `sonoro-cfgsv`, the AES-encrypted config server.

mod encryption;
mod serve;

use std::net::SocketAddr;
use std::process::ExitCode;
use std::sync::Arc;

use sonoro_common::FileSystem;
use sonoro_common::args::{FlagSpec, format_usage, parse_flags};
use sonoro_common::signal::shutdown_signal;
use tracing_subscriber::EnvFilter;

use crate::serve::AppState;

const FLAG_SPECS: &[FlagSpec] = &[
    FlagSpec {
        flag: 's',
        name: "state_dir",
    },
    FlagSpec {
        flag: 'l',
        name: "listen_address",
    },
];

struct Args {
    state_dir: String,
    listen_address: String,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            state_dir: String::from("state"),
            listen_address: String::from("127.0.0.1:5500"),
        }
    }
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
        .unwrap_or("sonoro-cfgsv");

    let mut args = Args::default();
    if !parse_flags(&raw_args[1..], FLAG_SPECS, |flag, val| {
        match flag {
            's' => args.state_dir = val.to_string(),
            'l' => args.listen_address = val.to_string(),
            _ => return false,
        }
        true
    }) {
        eprintln!("{}", format_usage(program, FLAG_SPECS));
        return ExitCode::from(1);
    }

    let address: SocketAddr = match args.listen_address.parse() {
        Ok(addr) => addr,
        Err(err) => {
            tracing::error!(listen_address = %args.listen_address, error = %err, "invalid listen address specified");
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

    let listener = match tokio::net::TcpListener::bind(address).await {
        Ok(listener) => listener,
        Err(err) => {
            tracing::error!(%address, error = %err, "failed to listen");
            if err.kind() == std::io::ErrorKind::AddrInUse {
                tracing::error!("another instance of this server might be already running");
            }
            return ExitCode::from(1);
        }
    };

    tracing::info!(%address, "config server is listening");

    let app = serve::router(AppState { fs });
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
