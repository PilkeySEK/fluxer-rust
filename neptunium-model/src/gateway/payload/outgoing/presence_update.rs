use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    gateway::presence::CustomStatus,
    time::timestamp::{Timestamp, representations::UnixMillis},
};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Online,
    #[serde(rename = "dnd")]
    DoNotDisturb,
    Idle,
    Invisible,
    Offline,
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug)]
pub struct PresenceUpdateOutgoing {
    pub since: Option<Timestamp<UnixMillis>>,
    // #[builder(default = vec![])]
    // pub activities: Vec<Activity>,
    pub status: Status,
    pub custom_status: CustomStatus,
    #[builder(default = false)]
    pub afk: bool,
}
