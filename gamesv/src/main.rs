mod network;
#[cfg(test)]
mod audit_tests;

use std::env;
use std::net::SocketAddr;
use std::process::ExitCode;
use std::sync::Arc;

use sonoro_common::args::{FlagSpec, format_usage, parse_flags};
use sonoro_common::fs::FileSystem;
use sonoro_common::signal::shutdown_signal;
use sonoro_gamesv_data::{Assets, SqliteStore};
use tracing_subscriber::EnvFilter;

use network::SessionManager;

const FLAG_SPECS: &[FlagSpec] = &[
    FlagSpec {
        flag: 's',
        name: "state_dir",
    },
    FlagSpec {
        flag: 'b',
        name: "bind_address",
    },
    FlagSpec {
        flag: 'g',
        name: "gm_bind_address",
    },
];

struct Args {
    state_dir: String,
    bind_address: String,
    gm_bind_address: String,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            state_dir: "state".to_string(),
            bind_address: "127.0.0.1:13100".to_string(),
            gm_bind_address: "127.0.0.1:13101".to_string(),
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

    let raw_args: Vec<String> = env::args().collect();
    let prog = raw_args.first().map_or("sonoro-gamesv", String::as_str);
    let mut args = Args::default();
    if !parse_flags(&raw_args[1..], FLAG_SPECS, |flag, val| {
        match flag {
            's' => args.state_dir = val.to_string(),
            'b' => args.bind_address = val.to_string(),
            'g' => args.gm_bind_address = val.to_string(),
            _ => return false,
        }
        true
    }) {
        eprintln!("{}", format_usage(prog, FLAG_SPECS));
        return ExitCode::from(1);
    }

    network::aki_logic_bridge::configure_from_environment();
    network::aki_logic_bridge::warm_up_in_process();
    let _ = network::aki_logic_bridge::ping();

    let assets = match Assets::load_default() {
        Ok(a) => Arc::new(a),
        Err(err) => {
            tracing::error!("failed to load assets: {}", err);
            return ExitCode::from(1);
        }
    };
    // Warm world-controller cache for Map 8 so the very first login never stalls 1-2s.
    let _ = assets.world_controller_entities(8);

    let bind_address: SocketAddr = match args.bind_address.parse() {
        Ok(addr) => addr,
        Err(err) => {
            tracing::error!("invalid bind address '{}': {}", args.bind_address, err);
            return ExitCode::from(1);
        }
    };
    let gm_bind_address: SocketAddr = match args.gm_bind_address.parse() {
        Ok(addr) => addr,
        Err(err) => {
            tracing::error!("invalid gm bind address '{}': {}", args.gm_bind_address, err);
            return ExitCode::from(1);
        }
    };

    let fs = match FileSystem::open(&args.state_dir) {
        Ok(fs) => Arc::new(fs),
        Err(err) => {
            tracing::error!("failed to open filesystem at '{}': {}", args.state_dir, err);
            return ExitCode::from(1);
        }
    };

    let db_path = std::path::Path::new(&args.state_dir).join("gamesv.db");
    let sqlite_store = match SqliteStore::open(&db_path) {
        Ok(store) => {
            if let Err(err) = store.attach_to_fs(&fs) {
                tracing::warn!("SQLite attach/backfill warning: {}", err);
            }
            Some(store)
        }
        Err(err) => {
            tracing::warn!(
                "failed to initialize shared SQLite store ({}), continuing with file-only state",
                err
            );
            None
        }
    };
    let sqlite_keepalive = sqlite_store;

    let session_manager = Arc::new(SessionManager::new());
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);

    let net_sm = Arc::clone(&session_manager);
    let net_fs = Arc::clone(&fs);
    let net_assets = Arc::clone(&assets);
    let net_shutdown = shutdown_rx.clone();
    let net_task = tokio::spawn(async move {
        if let Err(err) =
            network::bind_and_serve(bind_address, net_fs, net_assets, net_sm, net_shutdown).await
        {
            tracing::error!("game server UDP bind failed: {}", err);
        }
    });

    let gm_sm = Arc::clone(&session_manager);
    let gm_assets = Arc::clone(&assets);
    let gm_shutdown = shutdown_rx;
    let gm_task = tokio::spawn(async move {
        network::gm_http::serve(gm_bind_address, gm_assets, gm_sm, gm_shutdown).await;
    });

    tracing::info!("game server is listening at {}", bind_address);
    shutdown_signal().await;
    tracing::info!("shutting down...");

    let _ = shutdown_tx.send(true);
    let _ = net_task.await;
    let _ = gm_task.await;
    session_manager.clear_handoffs();
    fs.flush();
    if let Some(store) = sqlite_keepalive {
        store.flush();
    }
    ExitCode::SUCCESS
}
