use crate::time::duration::{Duration, representation::UnixMillis};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Hello {
    pub heartbeat_interval: Duration<UnixMillis>,
}
