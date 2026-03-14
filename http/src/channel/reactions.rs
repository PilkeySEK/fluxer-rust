use std::fmt::Display;

use fluxer_model::id::{Id, marker::EmojiMarker};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

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
