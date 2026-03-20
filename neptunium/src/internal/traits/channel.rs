use neptunium_model::{
    channel::{Channel, ChannelPartial},
    id::{Id, marker::ChannelMarker},
};

/// Things that all channels have in common.
pub trait ChannelTrait: Sync {
    fn get_channel_id(&self) -> Id<ChannelMarker>;
}

impl ChannelTrait for Channel {
    fn get_channel_id(&self) -> Id<ChannelMarker> {
        self.id
    }
}

impl ChannelTrait for ChannelPartial {
    fn get_channel_id(&self) -> Id<ChannelMarker> {
        self.id
    }
}

impl ChannelTrait for Id<ChannelMarker> {
    fn get_channel_id(&self) -> Id<ChannelMarker> {
        *self
    }
}
