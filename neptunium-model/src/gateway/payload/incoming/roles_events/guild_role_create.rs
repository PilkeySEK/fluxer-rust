use serde::{Deserialize, Serialize};

use crate::guild::permissions::GuildRole;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildRoleCreate {
    pub role: GuildRole,
}
