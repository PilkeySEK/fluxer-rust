use async_trait::async_trait;
use fluxer_model::{
    channel::{Channel, ChannelPartial},
    id::{Id, marker::ChannelMarker},
};
use neptunium_http::endpoints::channel::{
    ChannelSettingsUpdates, DeleteChannel, FetchChannel, UpdateChannelSettings,
};

use crate::{client::error::Error, events::context::Context};

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
}

#[async_trait]
impl ChannelExt for Id<ChannelMarker> {
    async fn fetch(&self, ctx: &Context) -> Result<Channel, Error> {
        Ok(ctx
            .get_http_client()
            .execute(FetchChannel::builder().channel_id(*self).build())
            .await?)
    }

    async fn delete(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(DeleteChannel::builder().channel_id(*self).build())
            .await?)
    }

    async fn delete_silent(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                DeleteChannel::builder()
                    .channel_id(*self)
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
                    .channel_id(*self)
                    .updates(settings)
                    .build(),
            )
            .await?)
    }
}

#[async_trait]
impl ChannelExt for Channel {
    async fn delete(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(DeleteChannel::builder().channel_id(self.id).build())
            .await?)
    }

    async fn delete_silent(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                DeleteChannel::builder()
                    .channel_id(self.id)
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
                    .channel_id(self.id)
                    .updates(settings)
                    .build(),
            )
            .await?)
    }

    async fn fetch(&self, ctx: &Context) -> Result<Channel, Error> {
        Ok(ctx
            .get_http_client()
            .execute(FetchChannel::builder().channel_id(self.id).build())
            .await?)
    }
}

#[async_trait]
impl ChannelExt for ChannelPartial {
    async fn delete(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(DeleteChannel::builder().channel_id(self.id).build())
            .await?)
    }

    async fn delete_silent(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                DeleteChannel::builder()
                    .channel_id(self.id)
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
                    .channel_id(self.id)
                    .updates(settings)
                    .build(),
            )
            .await?)
    }

    async fn fetch(&self, ctx: &Context) -> Result<Channel, Error> {
        Ok(ctx
            .get_http_client()
            .execute(FetchChannel::builder().channel_id(self.id).build())
            .await?)
    }
}
