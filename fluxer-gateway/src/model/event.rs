use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::model::event::hello::HelloEventData;

pub mod hello;

#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum GatewayOpCode {
    Dispatch = 0,
    Heartbeat,
    Identify,
    PresenceUpdate,
    VoiceStateUpdate,
    Resume = 6,
    Reconnect,
    RequestGuildMembers,
    InvalidSession,
    Hello,
    HeartbeatAck,
    RequestSoundboardSounds = 31,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayEventPayload {
    /// Gateway opcode, which indicates the payload type
    pub op: GatewayOpCode,
    /// Event data
    pub d: Option<Value>,
    /// Sequence number of event used for resuming sessions and heartbeating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<u64>,
    /// Event name
    pub t: Option<String>,
}

#[derive(Clone, Debug)]
pub struct GatewayEvent {
    pub data: GatewayEventData,
    pub payload: GatewayEventPayload,
}

#[derive(Debug, Copy, Clone)]
pub enum GatewayEventData {
    Hello(HelloEventData),
    HeartbeatAck,
    Heartbeat,
}
