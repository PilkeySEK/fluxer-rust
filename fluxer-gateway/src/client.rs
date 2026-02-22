use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{Message, Utf8Bytes},
};
use tracing::Level;

use crate::{
    client::client_config::GatewayClientConfiguration,
    model::event::{
        GatewayEvent, GatewayEventPayload, IncomingGatewayEventData, IncomingGatewayOpCode,
        OutgoingGatewayEventData, OutgoingGatewayOpCode,
        dispatch::DispatchEvent,
        identify::{ConnectionProperties, IdentifyEventData},
        invalid_session::InvalidSessionEventData,
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
        let url = format!(
            "{}/?v={}&encoding=json",
            self.config.gateway_url, self.config.version
        );
        tracing::event!(Level::TRACE, %url, "Establishing connection");
        Ok(tokio_tungstenite::connect_async(url)
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
        tracing::event!(Level::TRACE, "Received message from gateway: {:?}", payload);
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
            IncomingGatewayOpCode::InvalidSession => {
                let Some(d) = payload.d.clone() else {
                    return Err(GatewayClientError::new(
                        GatewayClientErrorType::NoDataFieldInPayload,
                    ));
                };
                let Value::Bool(resumable) = d else {
                    return Err(GatewayClientError::new(
                        GatewayClientErrorType::UnexpectedData,
                    ));
                };
                GatewayEvent {
                    data: IncomingGatewayEventData::InvalidSession(InvalidSessionEventData {
                        resumable,
                    }),
                    payload,
                }
            }
            IncomingGatewayOpCode::Reconnect => GatewayEvent {
                data: IncomingGatewayEventData::Reconnect,
                payload,
            },
            IncomingGatewayOpCode::Dispatch => {
                let dispatch_event: DispatchEvent = payload.clone().try_into()?;
                GatewayEvent {
                    data: IncomingGatewayEventData::Dispatch(Box::new(dispatch_event)),
                    payload,
                }
            }
        })
    }

    /// Send a raw message.
    /// # Errors
    /// If sending failed, will return a tungstenite error.
    pub async fn send_raw(
        sink: &mut GatewayConnectionWriteHalf,
        message: Message,
    ) -> Result<(), GatewayClientError> {
        sink.send(message)
            .await
            .map_err(|e| GatewayClientError::new(GatewayClientErrorType::TungsteniteError(e)))
    }

    /// # Errors
    /// If sending failed, will return a tungstenite error.
    /// # Panics
    /// Panics if `serde_json::to_value()` or `serde_json::to_string()` fails (which shouldn't happen in this case).
    pub async fn send(
        sink: &mut GatewayConnectionWriteHalf,
        event: OutgoingGatewayEventData,
    ) -> Result<(), GatewayClientError> {
        tracing::event!(Level::TRACE, "Sending: {:?}", event);
        match event {
            OutgoingGatewayEventData::Identify(data) => {
                let payload = GatewayEventPayload {
                    op: OutgoingGatewayOpCode::Identify,
                    d: Some(serde_json::to_value(data).unwrap()),
                    s: None,
                    t: None,
                };

                Self::send_raw(
                    sink,
                    Message::Text(Utf8Bytes::from(serde_json::to_string(&payload).unwrap())),
                )
                .await
            }
            OutgoingGatewayEventData::Heartbeat(data) => {
                let payload = GatewayEventPayload {
                    op: OutgoingGatewayOpCode::Heartbeat,
                    d: Some(serde_json::to_value(data.last_sequence_number).unwrap()),
                    s: None,
                    t: None,
                };

                Self::send_raw(
                    sink,
                    Message::Text(Utf8Bytes::from(serde_json::to_string(&payload).unwrap())),
                )
                .await
            }
        }
    }

    /// # Errors
    /// If sending failed, will return a tungstenite error.
    pub async fn identify(
        &self,
        sink: &mut GatewayConnectionWriteHalf,
    ) -> Result<(), GatewayClientError> {
        Self::send(
            sink,
            OutgoingGatewayEventData::Identify(IdentifyEventData {
                token: self.config.token.to_string(),
                properties: ConnectionProperties {
                    os: String::from("linux"), // TODO: properly detect OS
                    browser: String::from("fluxer-gateway"),
                    device: String::from("fluxer-gateway"),
                },
                compress: None,
                large_threshold: None,
                shard: None,
                presence: None,
                intents: self.config.intents.into(),
            }),
        )
        .await
    }
}
