use serde::Deserialize;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct HelloEventData {
    pub heartbeat_interval: u64,
}
