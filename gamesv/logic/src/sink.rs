use sonoro_proto::wire::PbMessage;

#[derive(Debug, Clone)]
pub struct OutboundPacket {
    pub msg_id: u16,
    pub payload: Vec<u8>,
}

/// Abstract push/response transport sink so `sonoro-gamesv-logic` remains
/// independent of networking crates.
pub trait PushSink {
    fn push_raw(&mut self, msg_id: u16, payload: Vec<u8>);

    fn push<M: PbMessage>(&mut self, msg_id: u16, message: &M) {
        self.push_raw(msg_id, message.encode_to_vec());
    }
}

/// In-memory recorder sink useful for unit tests and buffered packet builders.
#[derive(Debug, Default, Clone)]
pub struct VecPushSink {
    pub packets: Vec<OutboundPacket>,
}

impl PushSink for VecPushSink {
    fn push_raw(&mut self, msg_id: u16, payload: Vec<u8>) {
        self.packets.push(OutboundPacket { msg_id, payload });
    }
}

/// Discards pushed packets when only in-memory scene state synchronization is needed.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopSink;

impl PushSink for NoopSink {
    fn push_raw(&mut self, _msg_id: u16, _payload: Vec<u8>) {}
}
