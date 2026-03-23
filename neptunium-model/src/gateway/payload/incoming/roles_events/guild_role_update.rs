use serde::{Deserialize, Serialize};

use crate::guild::permissions::GuildRole;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildRoleUpdate {
    pub role: GuildRole,
}
