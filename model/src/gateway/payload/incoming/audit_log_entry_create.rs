use serde::Deserialize;

use crate::{
    guild::audit_log::AuditLogEntry,
    id::{Id, marker::GuildMarker},
};

#[derive(Deserialize, Clone, Debug)]
pub struct GuildAuditLogEntryCreate {
    #[serde(flatten)]
    pub audit_log_entry: AuditLogEntry,
    pub guild_id: Id<GuildMarker>,
}
