use std::{sync::Arc, time::Duration};

use fluxer_gateway::{
    client::{
        GatewayClient, GatewayConnectionWriteHalf, client_config::GatewayClientConfiguration,
    },
    model::event::{
        GatewayEvent, IncomingGatewayEventData, IncomingGatewayOpCode, OutgoingGatewayEventData,
        dispatch::DispatchEvent, heartbeat::OutgoingHeartbeatEventData,
    },
};
use tokio::sync::{
    Mutex,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

pub use async_trait::async_trait;
pub use error::Error;
pub use fluxer_gateway::client::client_config::GatewayIntents;
use tracing::Level;

mod error;
pub mod events;

use crate::{
    error::NeptuniumErrorKind,
    events::{
        Event, EventBus, EventListener,
        data::{GuildDeleteEventData, ReadyEventData},
    },
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EventType {
    Ready,
}

enum ClientMessage {
    SendMessage(OutgoingGatewayEventData),
    SendHeartbeat,
    Received(GatewayEvent<IncomingGatewayEventData, IncomingGatewayOpCode>),
}

pub struct Client<'a> {
    #[expect(clippy::struct_field_names)]
    gateway_client: GatewayClient<'a>,
    // // tx and rx should be moved outside of the Client struct to avoid
    // // having to Mutex::lock() the client each time we want to
    // // clone tx or access rx
    // tx: UnboundedSender<ClientMessage>,
    // rx: UnboundedReceiver<ClientMessage>,
    last_sequence_number: Option<u64>,
    event_bus: Option<EventBus>,
}

impl<'a> From<GatewayClient<'a>> for Client<'a> {
    fn from(value: GatewayClient<'a>) -> Self {
        Self {
            gateway_client: value,
            last_sequence_number: None,
            event_bus: None,
        }
    }
}

impl<'a> Client<'a> {
    /// Construct a new client given a `token` and the `GatewayIntents`
    #[must_use]
    pub fn new(token: &'a str, intents: GatewayIntents) -> Self {
        Self {
            gateway_client: GatewayClient::new(GatewayClientConfiguration::new(token, intents)),
            last_sequence_number: None,
            event_bus: None,
        }
    }

    pub async fn start(mut self) -> Result<(), crate::error::Error> {
        tracing::event!(Level::DEBUG, "Starting client");
        let (tx, mut rx) = mpsc::unbounded_channel::<ClientMessage>();
        let (mut write, mut read) = self
            .gateway_client
            .establish_connection()
            .await
            .map_err(Into::<Error>::into)?;
        tracing::event!(Level::TRACE, "Successfully established connection");
        let next_event = match GatewayClient::next_event(&mut read).await {
            Ok(event) => event,
            Err(e) => return Err(e.into()),
        };
        let IncomingGatewayEventData::Hello(hello_data) = next_event.data else {
            return Err(Error::new(NeptuniumErrorKind::UnexpectedEvent(next_event)));
        };

        let heartbeat_interval = hello_data.heartbeat_interval;

        let mut event_bus = self.event_bus.take().unwrap_or(EventBus::new());

        let client = Arc::new(tokio::sync::Mutex::new(self));

        let cloned_tx = tx.clone();
        tokio::spawn(async move {
            let wait_time = rand::random_range(0..heartbeat_interval);
            tokio::time::sleep(Duration::from_millis(wait_time)).await;
            let _ = cloned_tx.send(ClientMessage::SendHeartbeat);
        });
        let cloned_tx = tx.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(heartbeat_interval)).await;
                if cloned_tx.send(ClientMessage::SendHeartbeat).is_err() {
                    // If tx.send() returns an error it means that the channel has been closed
                    // which indicates that this task should end too
                    break;
                }
            }
        });
        let cloned_tx = tx.clone();
        tokio::spawn(async move {
            loop {
                let Ok(next_event) = GatewayClient::next_event(&mut read).await else {
                    // TODO: Maybe check for which error happened
                    break;
                };
                if cloned_tx.send(ClientMessage::Received(next_event)).is_err() {
                    break;
                }
            }
        });

        if let Err(e) = client
            .lock()
            .await
            .gateway_client
            .identify(&mut write)
            .await
        {
            return Err(e.into());
        }

        Self::handle_messages(client, tx, rx, event_bus, write).await
    }

    async fn handle_messages(
        client: Arc<Mutex<Self>>,
        tx: UnboundedSender<ClientMessage>,
        mut rx: UnboundedReceiver<ClientMessage>,
        mut event_bus: EventBus,
        mut write: GatewayConnectionWriteHalf,
    ) -> Result<(), crate::error::Error> {
        while let Some(message) = rx.recv().await {
            match message {
                ClientMessage::SendMessage(message) => {
                    if let Err(e) = GatewayClient::send(&mut write, message).await {
                        return Err(e.into());
                    }
                }
                ClientMessage::SendHeartbeat => {
                    let message = OutgoingGatewayEventData::Heartbeat(OutgoingHeartbeatEventData {
                        last_sequence_number: client.lock().await.last_sequence_number,
                    });
                    if let Err(e) = GatewayClient::send(&mut write, message).await {
                        return Err(e.into());
                    }
                }
                ClientMessage::Received(event) => match event.data {
                    IncomingGatewayEventData::HeartbeatAck => {}
                    IncomingGatewayEventData::Hello(_) => {
                        return Err(Error::new(NeptuniumErrorKind::UnexpectedEvent(event)));
                    }
                    IncomingGatewayEventData::Heartbeat => {
                        let _ = tx.send(ClientMessage::SendHeartbeat);
                    }
                    IncomingGatewayEventData::InvalidSession(data) => {
                        if !data.resumable {
                            return Err(Error::new(NeptuniumErrorKind::SessionInvalidated));
                        }

                        todo!("resuming/reconnecting");
                    }
                    IncomingGatewayEventData::Reconnect => todo!("reconnecting"),
                    IncomingGatewayEventData::Dispatch(event) => match *event {
                        DispatchEvent::Ready(data) => {
                            event_bus
                                .emit(
                                    Event::Ready(Box::new(ReadyEventData {
                                        dispatch_data: *data,
                                    })),
                                    Arc::clone(&client),
                                )
                                .await;
                        }
                        DispatchEvent::GuildDelete(data) => {
                            event_bus
                                .emit(
                                    Event::GuildDelete(GuildDeleteEventData {
                                        id: data.id,
                                        unavailable: data.unavailable.unwrap_or(false),
                                    }),
                                    Arc::clone(&client),
                                )
                                .await;
                        }
                        DispatchEvent::GuildCreate(data) => todo!(),
                    },
                },
            }
        }

        Ok(())
    }

    pub fn register_event_listener(&mut self, listener: impl EventListener + Send + 'static) {
        self.event_bus
            .get_or_insert(EventBus::new())
            .register(Box::new(listener) as Box<dyn EventListener + Send>);
    }
}
