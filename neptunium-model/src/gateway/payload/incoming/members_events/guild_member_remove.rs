use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::UserMarker};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct GuildMemberRemoveUser {
    pub id: Id<UserMarker>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct GuildMemberRemove {
    pub user: GuildMemberRemoveUser,
}
