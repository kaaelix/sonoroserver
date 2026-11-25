//! `AkiLogicService` IPC server.

use std::process::ExitCode;

use sonoro_aki_logic::{handlers, ipc::IpcServer, sqlite_store};
use sonoro_common::signal::shutdown_signal;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    println!("==================================================");
    println!("  AkiLogicService                                 ");
    println!("==================================================");

    if let Err(err) = sqlite_store::ensure_initialized() {
        tracing::error!(error = %err, "failed to initialize SQLite store");
        return ExitCode::from(1);
    }

    let (player_count, file_count) = sqlite_store::get_database_stats().unwrap_or((0, 0));
    let handler_count = handlers::rich_handler_count();
    println!("[AkiLogicService] SQLite stats: players={player_count}, state_files={file_count}");
    println!("[AkiLogicService] Explicit Rust Handlers: {handler_count}");

    let socket_path = std::env::var("AKI_IPC_SOCKET_PATH")
        .unwrap_or_else(|_| String::from("/tmp/aki_logic.sock"));
    let tcp_port: u16 = std::env::var("AKI_IPC_TCP_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(19999);

    let server = IpcServer::new(socket_path, tcp_port);
    if let Err(err) = server.run(shutdown_signal()).await {
        tracing::error!(error = %err, "IPC server terminated with error");
        return ExitCode::from(1);
    }

    println!("[AkiLogicService] Shutting down gracefully...");
    ExitCode::SUCCESS
}
