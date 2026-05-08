use serde::Deserialize;

use crate::{
    guild::member::GuildMember,
    id::{Id, marker::GuildMarker},
};

#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberUpdate {
    pub guild_id: Id<GuildMarker>,
    #[serde(flatten)]
    pub member: GuildMember,
}
