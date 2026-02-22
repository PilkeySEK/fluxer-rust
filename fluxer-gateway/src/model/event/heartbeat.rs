#[derive(Debug, Copy, Clone)]
pub struct OutgoingHeartbeatEventData {
    pub last_sequence_number: Option<u64>,
}
