use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::model::event::{
    dispatch::DispatchEvent, hello::HelloEventData, identify::IdentifyEventData,
};

pub mod dispatch;
pub mod hello;
pub mod identify;

/// Opcodes for incoming gateway events
#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum IncomingGatewayOpCode {
    Dispatch = 0,
    Heartbeat = 1,
    Reconnect = 7,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11,
}

/// Opcodes for outgoing gateway events
#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum OutgoingGatewayOpCode {
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    RequestGuildMembers = 8,
    RequestSoundsboardSounds = 31,
}

pub trait GatewayOpCode {}

impl GatewayOpCode for IncomingGatewayOpCode {}
impl GatewayOpCode for OutgoingGatewayOpCode {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayEventPayload<O: GatewayOpCode> {
    /// Gateway opcode, which indicates the payload type
    pub op: O,
    /// Event data
    pub d: Option<Value>,
    /// Sequence number of event used for resuming sessions and heartbeating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<u64>,
    /// Event name. This is always `None` if `op` is not `Dispatch`
    pub t: Option<String>,
}

pub trait GatewayEventData {}

impl GatewayEventData for IncomingGatewayEventData {}

#[derive(Clone, Debug)]
pub struct GatewayEvent<D: GatewayEventData + Clone, O: GatewayOpCode> {
    pub data: D,
    pub payload: GatewayEventPayload<O>,
}

#[derive(Debug, Clone)]
pub enum IncomingGatewayEventData {
    Hello(HelloEventData),
    HeartbeatAck,
    Heartbeat,
    InvalidSession,
    Reconnect,
    /// This variant is boxed to keep the enum size relatively small.
    Dispatch(Box<DispatchEvent>),
}

#[derive(Debug, Clone)]
pub enum OutgoingGatewayEventData {
    Identify(IdentifyEventData),
}
