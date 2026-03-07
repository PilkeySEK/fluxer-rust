use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct PublicUserFlags: u32 {
        const STAFF = 1 << 0;
        const CTP_MEMBER = 1 << 1;
        const PARTNER = 1 << 2;
        const BUG_HUNTER = 1 << 3;
        const FRIENDLY_BOT = 1 << 4;
        const FRIENDLY_BOT_MANUAL_APPROVAL = 1 << 5;
    }
}

impl Serialize for PublicUserFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for PublicUserFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_bits_truncate(u32::deserialize(deserializer)?))
    }
}
