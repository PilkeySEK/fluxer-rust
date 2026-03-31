use serde::Deserialize;

use crate::{
    channel::message::Message,
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker, SavedMessageMarker},
    },
};

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SavedMessageAvailability {
    Available,
    MissingPermissions,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SavedMessage {
    pub id: Id<SavedMessageMarker>,
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub status: SavedMessageAvailability,
    pub message: Message,
}
