use async_trait::async_trait;
use neptunium_http::endpoints::channel::{
    CallEligibilityStatus, ChannelSettingsUpdates, DeleteChannel, FetchChannel,
    GetCallEligibilityStatus, UpdateChannelSettings,
};
use neptunium_model::channel::Channel;

use crate::{
    client::error::Error, events::context::Context, internal::traits::channel::ChannelTrait,
};

#[async_trait]
pub trait ChannelExt {
    async fn delete(&self, ctx: &Context) -> Result<(), Error>;
    async fn delete_silent(&self, ctx: &Context) -> Result<(), Error>;
    // TODO: Maybe make a builder or something around the ChannelSettingsUpdates
    // because it's annoying to create ig
    async fn update_settings(
        &self,
        ctx: &Context,
        settings: ChannelSettingsUpdates,
    ) -> Result<Channel, Error>;
    async fn fetch(&self, ctx: &Context) -> Result<Channel, Error>;
    async fn get_call_eligibility_status(
        &self,
        ctx: &Context,
    ) -> Result<CallEligibilityStatus, Error>;
}

#[async_trait]
impl<T: ChannelTrait> ChannelExt for T {
    async fn delete(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                DeleteChannel::builder()
                    .channel_id(self.get_channel_id())
                    .build(),
            )
            .await?)
    }
    async fn delete_silent(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                DeleteChannel::builder()
                    .channel_id(self.get_channel_id())
                    .silent(true)
                    .build(),
            )
            .await?)
    }
    async fn update_settings(
        &self,
        ctx: &Context,
        settings: ChannelSettingsUpdates,
    ) -> Result<Channel, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                UpdateChannelSettings::builder()
                    .channel_id(self.get_channel_id())
                    .updates(settings)
                    .build(),
            )
            .await?)
    }
    async fn fetch(&self, ctx: &Context) -> Result<Channel, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                FetchChannel::builder()
                    .channel_id(self.get_channel_id())
                    .build(),
            )
            .await?)
    }
    async fn get_call_eligibility_status(
        &self,
        ctx: &Context,
    ) -> Result<CallEligibilityStatus, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                GetCallEligibilityStatus::builder()
                    .channel_id(self.get_channel_id())
                    .build(),
            )
            .await?)
    }
}
