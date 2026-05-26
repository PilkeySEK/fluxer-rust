use serde::{Deserialize, Serialize};

use crate::id::{
    Id,
    marker::{GuildMarker, UserMarker},
};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct GuildMemberRemoveMember {
    pub id: Id<UserMarker>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct GuildMemberRemove {
    pub guild_id: Id<GuildMarker>,
    pub user: GuildMemberRemoveMember,
}
