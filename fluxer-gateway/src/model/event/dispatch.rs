use crate::{
    client::{GatewayClientError, GatewayClientErrorType},
    model::event::{
        GatewayEventPayload, IncomingGatewayOpCode, dispatch::session::ReadyDispatchData,
    },
};

/// Sent based on channel type (guild channels use guild scope, DMs use presence scope)
pub mod channel;
/// Sent to all users in a guild who have permission to receive it
pub mod guild;
/// Sent to all sessions of the current user
pub mod presence;
/// Sent only to the current session
pub mod session;

#[derive(Debug, Clone)]
pub enum DispatchEvent {
    Ready(ReadyDispatchData),
}

impl TryFrom<GatewayEventPayload<IncomingGatewayOpCode>> for DispatchEvent {
    type Error = GatewayClientError;
    fn try_from(value: GatewayEventPayload<IncomingGatewayOpCode>) -> Result<Self, Self::Error> {
        let Some(event_name) = value.t else {
            return Err(GatewayClientError::new(
                crate::client::GatewayClientErrorType::NoEventNameFieldInPayload,
            ));
        };

        Ok(match event_name.as_str() {
            "READY" => {
                let Some(d) = value.d else {
                    return Err(GatewayClientError::new(
                        crate::client::GatewayClientErrorType::NoDataFieldInPayload,
                    ));
                };
                Self::Ready(serde_json::from_value(d).map_err(|e| {
                    GatewayClientError::new(GatewayClientErrorType::DeserializeError(e))
                })?)
            }
            _ => {
                return Err(GatewayClientError::new(
                    GatewayClientErrorType::UnknownEvent(event_name),
                ));
            }
        })
    }
}
