use std::{error::Error, fmt};

use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum GatewayClientErrorType {
    TungsteniteError(tungstenite::Error),
    Closed,
    DeserializeError(serde_json::Error),
    UnsupportedMessageEncoding,
    NoDataFieldInPayload,
    NoEventNameFieldInPayload,
    UnknownEvent(String),
}

#[derive(Debug)]
pub struct GatewayClientError {
    pub(super) kind: GatewayClientErrorType,
}

impl GatewayClientError {
    #[must_use]
    pub fn kind(&self) -> &GatewayClientErrorType {
        &self.kind
    }

    pub(crate) fn new(kind: GatewayClientErrorType) -> Self {
        Self { kind }
    }
}

impl From<GatewayClientError> for GatewayClientErrorType {
    fn from(value: GatewayClientError) -> Self {
        value.kind
    }
}

impl Error for GatewayClientError {}

impl fmt::Display for GatewayClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            GatewayClientErrorType::TungsteniteError(e) => e.fmt(f),
            GatewayClientErrorType::Closed => f.write_str("Connection closed"),
            GatewayClientErrorType::DeserializeError(e) => {
                f.write_fmt(format_args!("Error deserializing: {e}"))
            }
            GatewayClientErrorType::UnsupportedMessageEncoding => {
                f.write_str("Unsupported message encoding")
            }
            GatewayClientErrorType::NoDataFieldInPayload => {
                f.write_str("Expected data \"d\" in payload, but it is not present")
            }
            GatewayClientErrorType::UnknownEvent(event_name) => {
                f.write_fmt(format_args!("Unknown event: {event_name}"))
            }
            GatewayClientErrorType::NoEventNameFieldInPayload => {
                f.write_str("Expected event name \"t\" in payload, but it is not present")
            }
        }
    }
}
