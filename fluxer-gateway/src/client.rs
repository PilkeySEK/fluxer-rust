use futures_util::{
    StreamExt,
    stream::{SplitSink, SplitStream},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::Message};

use crate::{
    client::client_config::GatewayClientConfiguration,
    model::event::{
        GatewayEvent, GatewayEventPayload, IncomingGatewayEventData, IncomingGatewayOpCode,
    },
};

pub mod client_config;
mod error;

pub use error::*;

pub type GatewayConnectionReadHalf = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
pub type GatewayConnectionWriteHalf =
    SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>;
pub type GatewayConnectionType = (GatewayConnectionWriteHalf, GatewayConnectionReadHalf);

pub struct GatewayClient<'a> {
    config: GatewayClientConfiguration<'a>,
}

impl<'a> GatewayClient<'a> {
    #[must_use]
    pub fn new(config: GatewayClientConfiguration<'a>) -> Self {
        Self { config }
    }

    /// Establishes a connection to the gateway, without sending anything yet. Returns the stream, which can be passed to other functions in this struct.
    /// # Errors
    /// Returns an error when establishing the connection failed.
    pub async fn establish_connection(
        &mut self,
    ) -> Result<GatewayConnectionType, GatewayClientError> {
        Ok(tokio_tungstenite::connect_async(format!(
            "{}/?v={}&encoding=json",
            self.config.gateway_url, self.config.version
        ))
        .await
        .map_err(|e| GatewayClientError {
            kind: GatewayClientErrorType::TungsteniteError(e),
        })?
        .0
        .split())
    }

    async fn next_message_raw(
        stream: &mut GatewayConnectionReadHalf,
    ) -> Result<Message, GatewayClientError> {
        let Some(next_message) = stream.next().await else {
            return Err(GatewayClientError::new(GatewayClientErrorType::Closed));
        };
        next_message
            .map_err(|e| GatewayClientError::new(GatewayClientErrorType::TungsteniteError(e)))
    }

    /// # Errors
    /// Returns an error either when reading the next message from the stream failed,
    /// when the message encoding is unsupported, or when the message could not be parsed
    /// by `serde_json`.
    pub async fn next_payload(
        stream: &mut GatewayConnectionReadHalf,
    ) -> Result<GatewayEventPayload<IncomingGatewayOpCode>, GatewayClientError> {
        let next_message = Self::next_message_raw(stream).await?;
        let Message::Text(next_message) = next_message else {
            return Err(GatewayClientError::new(
                GatewayClientErrorType::UnsupportedMessageEncoding,
            ));
        };

        serde_json::from_str::<GatewayEventPayload<IncomingGatewayOpCode>>(next_message.as_str())
            .map_err(|e| GatewayClientError::new(GatewayClientErrorType::DeserializeError(e)))
    }

    /// # Errors
    /// Has the same errors as [`GatewayClient::next_payload`]. Additionally, returns an
    /// error when invalid data is received.
    pub async fn next_event(
        stream: &mut GatewayConnectionReadHalf,
    ) -> Result<GatewayEvent<IncomingGatewayEventData, IncomingGatewayOpCode>, GatewayClientError>
    {
        let payload = Self::next_payload(stream).await?;
        Ok(match payload.op {
            IncomingGatewayOpCode::Hello => {
                let Some(d) = payload.d.clone() else {
                    return Err(GatewayClientError::new(
                        GatewayClientErrorType::NoDataFieldInPayload,
                    ));
                };
                GatewayEvent {
                    data: IncomingGatewayEventData::Hello(serde_json::from_value(d).map_err(
                        |e| GatewayClientError::new(GatewayClientErrorType::DeserializeError(e)),
                    )?),
                    payload,
                }
            }
            IncomingGatewayOpCode::HeartbeatAck => GatewayEvent {
                data: IncomingGatewayEventData::HeartbeatAck,
                payload,
            },
            IncomingGatewayOpCode::Heartbeat => GatewayEvent {
                data: IncomingGatewayEventData::Heartbeat,
                payload,
            },
            IncomingGatewayOpCode::InvalidSession => GatewayEvent {
                data: IncomingGatewayEventData::InvalidSession,
                payload,
            },
            IncomingGatewayOpCode::Reconnect => GatewayEvent {
                data: IncomingGatewayEventData::Reconnect,
                payload,
            },
            IncomingGatewayOpCode::Dispatch => todo!(),
        })
    }
}
