use std::fmt::Display;

use bon::Builder;
use fluxer_model::{
    id::{
        Id,
        marker::{ChannelMarker, EmojiMarker, MessageMarker},
    },
    user::UserPartial,
};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RequestReactionType<'a> {
    Custom {
        id: Id<EmojiMarker>,
        /// Name of the custom emoji.
        name: Option<&'a str>,
    },
    /// A unicode emoji, such as "🪑".
    Unicode(&'a str),
}

impl Display for RequestReactionType<'_> {
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

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListReactions<'a> {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub emoji: &'a RequestReactionType<'a>,
}

impl Endpoint for ListReactions<'_> {
    type Response = Vec<UserPartial>;
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
pub struct AddReaction<'a> {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub reaction: &'a RequestReactionType<'a>,
}

impl Endpoint for AddReaction<'_> {
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
