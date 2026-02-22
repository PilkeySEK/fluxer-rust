use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct InvalidSessionEventData {
    pub resumable: bool,
}
