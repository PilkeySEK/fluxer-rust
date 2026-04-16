use std::fmt::Display;

use bon::Builder;
use neptunium_model::{
    id::{
        Id,
        marker::{ChannelMarker, EmojiMarker, MessageMarker, UserMarker},
    },
    user::PartialUser,
};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RequestReactionType {
    Custom {
        id: Id<EmojiMarker>,
        /// Name of the custom emoji.
        name: Option<String>,
    },
    /// A unicode emoji, such as "🪑".
    Unicode(String),
}

impl Display for RequestReactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom { id, name } => {
                if let Some(name) = name {
                    f.write_str(name)?;
                } else {
                    f.write_str("e")?;
                }

                f.write_str(":")?;

                id.fmt(f)
            }
            Self::Unicode(name) => utf8_percent_encode(name, NON_ALPHANUMERIC).fmt(f),
        }
    }
}

#[derive(Builder, Clone, Debug)]
pub struct ListReactions {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub emoji: RequestReactionType,
}

impl Endpoint for ListReactions {
    type Response = Vec<PartialUser>;
    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!(
                "/channels/{}/messages/{}/reactions/{}",
                self.channel_id, self.message_id, self.emoji
            ))
            .build()
    }
}

#[derive(Builder, Clone, Debug)]
pub struct AddReaction {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub reaction: RequestReactionType,
}

impl Endpoint for AddReaction {
    type Response = ();
    fn into_request(self) -> Request {
        Request::builder()
            .method(Method::PUT)
            .path(format!(
                "/channels/{}/messages/{}/reactions/{}/@me",
                self.channel_id, self.message_id, self.reaction
            ))
            .build()
    }
}

/// Delete the bot's own specified reaction from a message.
#[derive(Builder, Clone, Debug)]
pub struct DeleteOwnReaction {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub reaction: RequestReactionType,
}

impl Endpoint for DeleteOwnReaction {
    type Response = ();

    fn into_request(self) -> Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!(
                "/channels/{}/messages/{}/reactions/{}/@me",
                self.channel_id, self.message_id, self.reaction
            ))
            .build()
    }
}

/// Delete one specified reaction from a user on a message.
#[derive(Builder, Clone, Debug)]
pub struct DeleteReaction {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub reaction: RequestReactionType,
    pub target: Id<UserMarker>,
}

impl Endpoint for DeleteReaction {
    type Response = ();

    fn into_request(self) -> Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!(
                "/channels/{}/messages/{}/reactions/{}/@{}",
                self.channel_id, self.message_id, self.reaction, self.target
            ))
            .build()
    }
}

/// Delete all reactions of a specified emoji from a message.
#[derive(Builder, Clone, Debug)]
pub struct DeleteAllReactionsOfEmoji {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub reaction: RequestReactionType,
}

impl Endpoint for DeleteAllReactionsOfEmoji {
    type Response = ();

    fn into_request(self) -> Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!(
                "/channels/{}/messages/{}/reactions/{}",
                self.channel_id, self.message_id, self.reaction
            ))
            .build()
    }
}

/// Delete all reactions from a message.
#[derive(Builder, Copy, Clone, Debug)]
pub struct DeleteAllReactions {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
}

impl Endpoint for DeleteAllReactions {
    type Response = ();

    fn into_request(self) -> Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!(
                "/channels/{}/messages/{}/reactions",
                self.channel_id, self.message_id
            ))
            .build()
    }
}
