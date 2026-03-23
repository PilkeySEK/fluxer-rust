use async_trait::async_trait;
use neptunium_http::endpoints::{
    channel::{
        add_user_to_group_dm::AddUserToGroupDm,
        delete_channel::DeleteChannel,
        delete_permission_overwrite::DeletePermissionOverwrite,
        fetch_channel::FetchChannel,
        get_call_eligibility_status::{CallEligibilityStatus, GetCallEligibilityStatus},
        indicate_typing::IndicateTyping,
        list_rtc_regions::{ListRtcRegions, ListRtcRegionsResponseEntry},
        messages::{
            bulk_delete_messages::BulkDeleteMessages,
            create_message::{CreateMessage, CreateMessageBody},
            list_channel_messages::{ListChannelMessages, ListChannelMessagesParams},
        },
        remove_user_from_group_dm::RemoveUserFromGroupDm,
        ring_call_recipients::RingCallRecipients,
        set_permission_overwrite::{PermissionOverwriteUpdate, SetPermissionOverwrite},
        stop_ringing_call_recipients::StopRingingCallRecipients,
        update_call_region::UpdateCallRegion,
        update_channel_settings::{ChannelSettingsUpdates, UpdateChannelSettings},
    },
    invites::{
        create_channel_invite::{CreateChannelInvite, CreateChannelInviteOptions},
        list_channel_invites::ListChannelInvites,
    },
    webhooks::{create_webhook::CreateWebhook, list_channel_webhooks::ListChannelWebhooks},
};
use neptunium_model::{
    channel::{Channel, VoiceRegion, message::Message},
    guild::webhook::Webhook,
    id::{
        Id,
        marker::{GenericMarker, MessageMarker, UserMarker},
    },
    invites::InviteWithMetadata,
};

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
    /// Update the voice region for an ongoing call.
    async fn update_call_region(&self, ctx: &Context, region: VoiceRegion) -> Result<(), Error>;
    /// Sends ringing notifications to specfied users in a call. If the recipients
    /// are set to `None`, rings all channel members.
    async fn ring_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), Error>;
    /// Stops ringing notifications for specified users in a call. This allows callers
    /// to stop notifying users who have declined or not responded. Pass `None` for the
    /// recipients to stop ringing everyone.
    async fn stop_ringing_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), Error>;
    async fn list_messages(
        &self,
        ctx: &Context,
        params: ListChannelMessagesParams,
    ) -> Result<Vec<Message>, Error>;
    async fn bulk_delete_messages(
        &self,
        ctx: &Context,
        messages: Vec<Id<MessageMarker>>,
    ) -> Result<(), Error>;
    /// Same as `create_message`.
    async fn send_message(
        &self,
        ctx: &Context,
        message: CreateMessageBody,
    ) -> Result<Message, Error>;
    async fn create_message(
        &self,
        ctx: &Context,
        message: CreateMessageBody,
    ) -> Result<Message, Error>;
    async fn set_permission_overwrite(
        &self,
        ctx: &Context,
        update: PermissionOverwriteUpdate,
    ) -> Result<(), Error>;
    async fn delete_permission_overwrite(
        &self,
        ctx: &Context,
        overwrite_id: Id<GenericMarker>,
    ) -> Result<(), Error>;
    #[cfg(feature = "user_api")]
    async fn acknowledge_new_pin_notifications(&self, ctx: &Context) -> Result<(), Error>;
    async fn add_user_to_group_dm(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
    ) -> Result<(), Error>;
    /// Remove a user from a group DM or leave a group DM by specifying
    /// your own user ID. Set `silent` to `true` to suppress the system
    /// message when leaving.
    async fn remove_user_from_group_dm(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
        silent: bool,
    ) -> Result<(), Error>;
    async fn list_rtc_regions(
        &self,
        ctx: &Context,
    ) -> Result<Vec<ListRtcRegionsResponseEntry>, Error>;
    async fn indicate_typing(&self, ctx: &Context) -> Result<(), Error>;
    async fn create_invite(
        &self,
        ctx: &Context,
        options: CreateChannelInviteOptions,
    ) -> Result<InviteWithMetadata, Error>;
    async fn list_invites(&self, ctx: &Context) -> Result<Vec<InviteWithMetadata>, Error>;
    async fn list_webhooks(&self, ctx: &Context) -> Result<Vec<Webhook>, Error>;
    /// Create a webhook in this channel, with the given name and optionally the avatar image as a base64-encoded data URI.
    async fn create_webhook(
        &self,
        ctx: &Context,
        name: String,
        avatar: Option<String>,
    ) -> Result<Webhook, Error>;
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

    async fn update_call_region(&self, ctx: &Context, region: VoiceRegion) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                UpdateCallRegion::builder()
                    .channel_id(self.get_channel_id())
                    .region(region)
                    .build(),
            )
            .await?)
    }

    async fn ring_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                RingCallRecipients::builder()
                    .channel_id(self.get_channel_id())
                    .maybe_recipients(recipients)
                    .build(),
            )
            .await?)
    }

    async fn stop_ringing_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                StopRingingCallRecipients::builder()
                    .channel_id(self.get_channel_id())
                    .maybe_recipients(recipients)
                    .build(),
            )
            .await?)
    }

    async fn list_messages(
        &self,
        ctx: &Context,
        params: ListChannelMessagesParams,
    ) -> Result<Vec<Message>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                ListChannelMessages::builder()
                    .channel_id(self.get_channel_id())
                    .params(params)
                    .build(),
            )
            .await?)
    }

    async fn bulk_delete_messages(
        &self,
        ctx: &Context,
        messages: Vec<Id<MessageMarker>>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                BulkDeleteMessages::builder()
                    .channel_id(self.get_channel_id())
                    .messages(messages)
                    .build(),
            )
            .await?)
    }

    async fn send_message(
        &self,
        ctx: &Context,
        message: CreateMessageBody,
    ) -> Result<Message, Error> {
        self.create_message(ctx, message).await
    }

    async fn create_message(
        &self,
        ctx: &Context,
        message: CreateMessageBody,
    ) -> Result<Message, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                CreateMessage::builder()
                    .channel_id(self.get_channel_id())
                    .message(message)
                    .build(),
            )
            .await?)
    }

    async fn set_permission_overwrite(
        &self,
        ctx: &Context,
        update: PermissionOverwriteUpdate,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(SetPermissionOverwrite {
                channel_id: self.get_channel_id(),
                overwrite: update,
            })
            .await?)
    }

    async fn delete_permission_overwrite(
        &self,
        ctx: &Context,
        overwrite_id: Id<GenericMarker>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(DeletePermissionOverwrite {
                channel_id: self.get_channel_id(),
                overwrite_id,
            })
            .await?)
    }

    #[cfg(feature = "user_api")]
    async fn acknowledge_new_pin_notifications(&self, ctx: &Context) -> Result<(), Error> {
        use neptunium_http::endpoints::channel::acknowledge_new_pin_notifications::AcknowledgeNewPinNotifications;

        Ok(ctx
            .get_http_client()
            .execute(AcknowledgeNewPinNotifications {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn add_user_to_group_dm(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(AddUserToGroupDm {
                channel_id: self.get_channel_id(),
                user_id,
            })
            .await?)
    }

    async fn remove_user_from_group_dm(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
        silent: bool,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(RemoveUserFromGroupDm {
                channel_id: self.get_channel_id(),
                user_id,
                silent,
            })
            .await?)
    }

    async fn list_rtc_regions(
        &self,
        ctx: &Context,
    ) -> Result<Vec<ListRtcRegionsResponseEntry>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListRtcRegions {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn indicate_typing(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(IndicateTyping {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn create_invite(
        &self,
        ctx: &Context,
        options: CreateChannelInviteOptions,
    ) -> Result<InviteWithMetadata, Error> {
        Ok(ctx
            .get_http_client()
            .execute(CreateChannelInvite {
                channel_id: self.get_channel_id(),
                options,
            })
            .await?)
    }

    async fn list_invites(&self, ctx: &Context) -> Result<Vec<InviteWithMetadata>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListChannelInvites {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn list_webhooks(&self, ctx: &Context) -> Result<Vec<Webhook>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListChannelWebhooks {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn create_webhook(
        &self,
        ctx: &Context,
        name: String,
        avatar: Option<String>,
    ) -> Result<Webhook, Error> {
        Ok(ctx
            .get_http_client()
            .execute(CreateWebhook {
                channel_id: self.get_channel_id(),
                name,
                avatar,
            })
            .await?)
    }
}
