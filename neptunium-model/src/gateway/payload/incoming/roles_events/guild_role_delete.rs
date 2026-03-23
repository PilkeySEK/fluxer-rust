use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::RoleMarker};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct GuildRoleDelete {
    pub role_id: Id<RoleMarker>,
}
