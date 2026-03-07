use serde::{Serialize, ser::SerializeStruct};

use crate::gateway::{
    event::op_code::OpCode,
    payload::outgoing::{heartbeat::Heartbeat, identify::Identify},
};

pub mod heartbeat;
pub mod identify;

#[derive(Clone, Debug)]
pub enum OutgoingGatewayMessage {
    Identify(Identify),
    Heartbeat(Heartbeat),
}

impl Serialize for OutgoingGatewayMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("OutgoingGatewayMessage", 2)?;

        let op = match self {
            Self::Identify(_) => OpCode::Identify,
            Self::Heartbeat(_) => OpCode::Heartbeat,
        } as u8;
        s.serialize_field("op", &op)?;
        match self {
            Self::Heartbeat(d) => s.serialize_field("d", d),
            Self::Identify(d) => s.serialize_field("d", d),
        }?;

        s.end()
    }
}
