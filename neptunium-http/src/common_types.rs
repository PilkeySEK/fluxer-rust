use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct SuccessResponse {
    /// Is always `true`.
    pub success: bool,
}
