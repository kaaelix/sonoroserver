pub mod bin_data;
pub mod handlers;
pub mod ipc;
pub mod sqlite_store;

use std::collections::HashMap;
use std::sync::OnceLock;

static IN_PROCESS_INIT: OnceLock<Result<(), String>> = OnceLock::new();
static IN_PROCESS_HANDLERS: OnceLock<HashMap<u16, handlers::HandlerFn>> = OnceLock::new();

/// Ensures the SQLite store backing in-process `aki-logic` calls is initialized once.
pub fn ensure_in_process_ready() -> Result<(), String> {
    IN_PROCESS_INIT
        .get_or_init(|| sqlite_store::ensure_initialized().map_err(|err| err.to_string()))
        .clone()
}

/// Dispatches an `aki-logic` message directly in memory without IPC serialization.
pub fn dispatch_in_process(
    player_id: i32,
    msg_id: u16,
    payload: &[u8],
) -> Option<ipc::AkiResult> {
    let _ = ensure_in_process_ready();
    let map = IN_PROCESS_HANDLERS.get_or_init(handlers::build_handlers);
    let handler = map.get(&msg_id)?;
    let result = handler(player_id, payload);
    if result.resp_msg_id == 0 && msg_id != 0 {
        return None;
    }
    Some(result)
}
