use fluxer_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};

use crate::channel::reactions::RequestReactionType;

macro_rules! define_routes {
    (
        $(#[$enum_meta:meta])*
        $enum_vis:vis enum $enum_name:ident<$lt:lifetime> {
            $(
                $route_name:ident $({ $($route_field_name:ident : $route_field_type:ty),+ $(,)? })?,
                $route_path:expr,
                $route_method:ident,
                // TODO: Rate limiting and stuff
            )+
        }
    ) => {
        $(#[$enum_meta])*
        $enum_vis enum $enum_name<$lt> {
            $(
                $route_name $({ $( $route_field_name : $route_field_type ),+ })?,
            )+
        }

        impl<$lt> $enum_name<$lt> {
            #[must_use]
            pub fn path(&self) -> ::std::borrow::Cow<'static, str> {
                match self {
                    $(
                        Self::$route_name $( { $( $route_field_name),+ } )? => $route_path.into(),
                    )+
                }
            }

            #[allow(unused)]
            #[must_use]
            pub fn method(&self) -> ::reqwest::Method {
                match self {
                    $(
                        Self::$route_name $( { $($route_field_name),+ } )? => ::reqwest::Method::$route_method,
                    )+
                }
            }
        }
    };
}

// /// Route information.
// #[derive(Clone, Debug, Eq, PartialEq, Hash)]
// pub enum Route<'a> {
//     /// Create a message in a channel.
//     CreateMessage {
//         channel_id: Id<ChannelMarker>,
//     },
//     /// Create a reaction on a message.
//     CreateReaction {
//         channel_id: Id<ChannelMarker>,
//         message_id: Id<MessageMarker>,
//         emoji: &'a RequestReactionType<'a>,
//     }
// }

define_routes! {
    #[derive(Clone, Debug, Eq, PartialEq, Hash)]
    pub enum Route<'a> {
        CreateMessage { channel_id: Id<ChannelMarker> },
        format!("/channels/{}/messages", channel_id),
        POST,

        CreateReaction { channel_id: Id<ChannelMarker>, emoji: &'a RequestReactionType<'a>, message_id: Id<MessageMarker> },
        format!("/channels/{}/messages/{}/reactions/{}/@me", channel_id, message_id, emoji),
        PUT,
    }
}
