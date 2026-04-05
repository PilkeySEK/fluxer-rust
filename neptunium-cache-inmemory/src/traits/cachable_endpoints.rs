use std::sync::Arc;

#[cfg(feature = "user_api")]
use std::collections::HashMap;

#[cfg(feature = "user_api")]
use neptunium_http::endpoints::{
    channel::PreloadMessagesForChannels,
    users::{
        GetUserSettings, ListCurrentUserMentions, UpdateCurrentUserProfile, UpdateUserSettings,
    },
};

#[cfg(feature = "user_api")]
use neptunium_model::{
    id::{Id, marker::ChannelMarker},
    user::settings::UserSettings,
};

use async_trait::async_trait;
use neptunium_http::{
    client::HttpClient,
    endpoints::{
        Endpoint, ExecuteEndpointRequestError,
        channel::{
            AddUserToGroupDm, BulkDeleteMessages, CreateMessage, CreatePrivateChannel,
            DeleteChannel, DeletePermissionOverwrite, GetChannel, ListChannelMessages,
            ListPrivateChannels, RemoveUserFromGroupDm, SetPermissionOverwrite, UpdateCallRegion,
            UpdateChannelSettings,
        },
        users::{GetCurrentUserProfile, GetUserById, GetUserProfile},
    },
};
use neptunium_model::{
    channel::{PermissionOverwrite, message::Message},
    guild::permissions::Permissions,
};
use tokio::sync::RwLock;

use crate::{CachableEndpoint, Cache, Cached, CachedChannel, traits::CacheValue};

#[async_trait]
impl CachableEndpoint for GetUserById {
    type Response = Cached<<Self as Endpoint>::Response>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        if let Some(cached_user) = cache.users.get(&self.user_id) {
            return Ok(cached_user);
        }
        let res = client.execute(self).await?;
        let user_id = res.id;
        let user = Arc::new(RwLock::new(res));
        cache.users.insert(user_id, Arc::clone(&user));
        Ok(user)
    }
}

#[async_trait]
impl CachableEndpoint for GetUserProfile {
    type Response = Cached<<Self as Endpoint>::Response>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let cache_key = (self.user_id, self.params.guild_id);
        let cached_profile = cache.user_profiles.get(&cache_key);
        let return_cached_profile = 'blk: {
            let Some(cached_profile) = &cached_profile else {
                break 'blk false;
            };
            let guard = cached_profile.read().await;
            if self.params.with_mutual_friends && guard.mutual_friends.is_none() {
                false
            } else {
                !(self.params.with_mutual_guilds && guard.mutual_guilds.is_none())
            }
        };
        if return_cached_profile {
            // Will never panic because the code that determines whether to return the cached profile already checks for Some(...)
            return Ok(cached_profile.unwrap());
        }

        let mut res = client.execute(self).await?;
        if let Some(cached_profile) = cached_profile {
            {
                let guard = cached_profile.read().await;
                if res.mutual_friends.is_none()
                    && let Some(mutual_friends) = &guard.mutual_friends
                {
                    let mutual_friends = mutual_friends.clone();
                    res.mutual_friends = Some(mutual_friends);
                }
                if res.mutual_guilds.is_none()
                    && let Some(mutual_guilds) = &guard.mutual_guilds
                {
                    let mutual_guilds = mutual_guilds.clone();
                    res.mutual_guilds = Some(mutual_guilds);
                }
            }
            {
                let mut guard = cached_profile.write().await;
                *guard = res;
            }
            Ok(cached_profile)
        } else {
            let id = res.user.id;
            let guild_id = self.params.guild_id;
            let arc = Arc::new(RwLock::new(res));
            cache.user_profiles.insert((id, guild_id), Arc::clone(&arc));
            Ok(arc)
        }
    }
}

#[async_trait]
impl CachableEndpoint for DeleteChannel {
    type Response = <Self as Endpoint>::Response;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let channel_id = self.channel_id;
        client.execute(self).await?;
        cache.channels.invalidate(&channel_id);
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for GetChannel {
    type Response = Cached<CachedChannel>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        if let Some(cached_channel) = cache.channels.get(&self.channel_id) {
            return Ok(cached_channel);
        }
        let res = client.execute(self).await?;
        Ok(CachedChannel::from(res).insert_and_return(cache).await)
    }
}

#[async_trait]
impl CachableEndpoint for UpdateChannelSettings {
    type Response = Cached<CachedChannel>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        Ok(CachedChannel::from(res).insert_and_return(cache).await)
    }
}

#[async_trait]
impl CachableEndpoint for UpdateCallRegion {
    type Response = <Self as Endpoint>::Response;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let channel_id = self.channel_id;
        let region_clone = self.region.clone();
        client.execute(self).await?;
        if let Some(cached_channel) = cache.channels.get(&channel_id) {
            let mut guard = cached_channel.write().await;
            guard.rtc_region = Some(region_clone);
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for BulkDeleteMessages {
    type Response = <Self as Endpoint>::Response;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let messages = self.messages.clone();
        client.execute(self).await?;
        for message in messages {
            cache.messages.invalidate(&message);
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for ListChannelMessages {
    type Response = Vec<Cached<Message>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        let mut cached_messages = Vec::with_capacity(res.len());
        for message in res {
            cached_messages.push(message.insert_and_return(cache).await);
        }
        Ok(cached_messages)
    }
}

#[async_trait]
impl CachableEndpoint for GetCurrentUserProfile {
    type Response = Cached<<Self as Endpoint>::Response>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        Ok(res.insert_and_return(cache).await)
    }
}

#[cfg(feature = "user_api")]
#[async_trait]
impl CachableEndpoint for UpdateCurrentUserProfile {
    type Response = Cached<<Self as Endpoint>::Response>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        Ok(res.insert_and_return(cache).await)
    }
}

#[async_trait]
impl CachableEndpoint for ListPrivateChannels {
    type Response = Vec<Cached<CachedChannel>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        let mut cached_channels = Vec::with_capacity(res.len());
        for channel in res {
            cached_channels.push(CachedChannel::from(channel).insert_and_return(cache).await);
        }
        Ok(cached_channels)
    }
}

#[async_trait]
impl CachableEndpoint for CreatePrivateChannel {
    type Response = Cached<CachedChannel>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        Ok(CachedChannel::from(client.execute(self).await?)
            .insert_and_return(cache)
            .await)
    }
}

#[cfg(feature = "user_api")]
#[async_trait]
impl CachableEndpoint for ListCurrentUserMentions {
    type Response = Vec<Cached<Message>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        let mut cached_messages = Vec::with_capacity(res.len());
        for message in res {
            cached_messages.push(message.insert_and_return(cache).await);
        }
        Ok(cached_messages)
    }
}

#[cfg(feature = "user_api")]
#[async_trait]
impl CachableEndpoint for PreloadMessagesForChannels {
    type Response = HashMap<Id<ChannelMarker>, Cached<Message>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        let mut cached_messages = HashMap::with_capacity(res.len());
        for (id, message) in res {
            cached_messages.insert(id, message.insert_and_return(cache).await);
        }
        Ok(cached_messages)
    }
}

#[async_trait]
impl CachableEndpoint for CreateMessage {
    type Response = Cached<<Self as Endpoint>::Response>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        Ok(client.execute(self).await?.insert_and_return(cache).await)
    }
}

#[async_trait]
impl CachableEndpoint for SetPermissionOverwrite {
    type Response = <Self as Endpoint>::Response;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        client.execute(self).await?;
        if let Some(existing_channel) = cache.channels.get(&self.channel_id) {
            let mut guard = existing_channel.write().await;
            if let Some(existing_overwrites) = &mut guard.permission_overwrites {
                for existing_overwrite in existing_overwrites {
                    if existing_overwrite.id == self.overwrite.id {
                        // https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/packages/api/src/channel/controllers/ChannelController.tsx#L272
                        // Permission overwrites are set to 0 (empty) if they were not provided in the request.
                        existing_overwrite.allow =
                            self.overwrite.allow.unwrap_or(Permissions::empty());
                        existing_overwrite.deny =
                            self.overwrite.deny.unwrap_or(Permissions::empty());
                    }
                }
            } else {
                guard.permission_overwrites = Some(vec![PermissionOverwrite {
                    allow: self.overwrite.allow.unwrap_or(Permissions::empty()),
                    deny: self.overwrite.deny.unwrap_or(Permissions::empty()),
                    id: self.overwrite.id,
                    r#type: self.overwrite.r#type,
                }]);
            }
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for DeletePermissionOverwrite {
    type Response = <Self as Endpoint>::Response;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        client.execute(self).await?;
        if let Some(existing_channel) = cache.channels.get(&self.channel_id) {
            let mut guard = existing_channel.write().await;
            if let Some(existing_overwrites) = &mut guard.permission_overwrites {
                existing_overwrites.retain(|overwrite| overwrite.id != self.overwrite_id);
            }
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for AddUserToGroupDm {
    type Response = ();
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        client.execute(self).await?;
        let Some(cached_user) = cache.users.get(&self.user_id) else {
            // TODO: Maybe spawn a new task to fetch the user, but this might not be a good idea
            return Ok(());
        };
        if let Some(existing_channel) = cache.channels.get(&self.channel_id) {
            let mut guard = existing_channel.write().await;
            let Some(recipients) = &mut guard.recipients else {
                drop(guard);
                tracing::warn!(%self.channel_id, "Cached group DM channel does not have recipients.");
                return Ok(());
            };
            recipients.push(cached_user);
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for RemoveUserFromGroupDm {
    type Response = ();
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        client.execute(self).await?;
        if let Some(existing_channel) = cache.channels.get(&self.channel_id) {
            let mut guard = existing_channel.write().await;
            let Some(recipients) = &mut guard.recipients else {
                drop(guard);
                tracing::warn!(%self.channel_id, "Cached group DM channel does not have recipients.");
                return Ok(());
            };
            let mut index = None;
            for (i, recipient) in recipients.iter().enumerate() {
                let guard = recipient.read().await;
                if guard.id == self.user_id {
                    index = Some(i);
                    break;
                }
            }
            let Some(index) = index else {
                drop(guard);
                tracing::trace!("Group DM recipient was not cached.");
                return Ok(());
            };
            recipients.remove(index);
        }
        Ok(())
    }
}

#[cfg(feature = "user_api")]
#[async_trait]
impl CachableEndpoint for GetUserSettings {
    type Response = Cached<UserSettings>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        if let Some(cached_settings) = cache.current_user_settings.get() {
            return Ok(Arc::clone(cached_settings));
        }
        Ok(Arc::clone(
            cache
                .current_user_settings
                .get_or_try_init(async || {
                    client
                        .execute(self)
                        .await
                        .map(|settings| Arc::new(RwLock::new(settings)))
                })
                .await?,
        ))
    }
}

#[cfg(feature = "user_api")]
#[async_trait]
impl CachableEndpoint for UpdateUserSettings {
    type Response = Cached<UserSettings>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let settings = client.execute(self).await?;
        if let Some(cached_settings) = cache.current_user_settings.get() {
            let cached_settings = Arc::clone(cached_settings);
            {
                let mut guard = cached_settings.write().await;
                *guard = settings;
            }
            return Ok(cached_settings);
        }
        Ok(Arc::clone(
            cache
                .current_user_settings
                .get_or_init(async || Arc::new(RwLock::new(settings)))
                .await,
        ))
    }
}
