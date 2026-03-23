use serde::{Deserialize, Serialize};

use crate::guild::permissions::GuildRole;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildRoleUpdateBulk {
    pub roles: Vec<GuildRole>,
}
