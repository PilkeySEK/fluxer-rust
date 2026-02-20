use std::num::ParseIntError;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct Snowflake(String);

impl From<Snowflake> for String {
    fn from(value: Snowflake) -> Self {
        value.0
    }
}

impl TryFrom<Snowflake> for u64 {
    type Error = ParseIntError;
    fn try_from(value: Snowflake) -> Result<Self, Self::Error> {
        value.0.parse()
    }
}

impl From<String> for Snowflake {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<u64> for Snowflake {
    fn from(value: u64) -> Self {
        Self(value.to_string())
    }
}
