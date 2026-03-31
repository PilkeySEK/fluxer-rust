use crate::{client::error::Error, events::context::Context, internal::traits::user::UserTrait};
use async_trait::async_trait;
use neptunium_http::endpoints::users::{
    GetUserById, GetUserProfile, GetUserProfileParams, UserProfileFullResponse,
};
use neptunium_model::user::PartialUser;
#[cfg(feature = "user_api")]
use neptunium_model::user::relationship::Relationship;

#[async_trait]
pub trait UserExt {
    #[cfg(feature = "user_api")]
    async fn send_friend_request(&self, ctx: &Context) -> Result<Relationship, Error>;
    /// Creates or updates a private note on this user.
    /// Pass `None` for the `note` to clear the note.
    #[cfg(feature = "user_api")]
    async fn set_user_note(&self, ctx: &Context, note: Option<String>) -> Result<(), Error>;
    /// Retrieves a specific note the current user has written about this user.
    #[cfg(feature = "user_api")]
    async fn get_user_note(&self, ctx: &Context) -> Result<String, Error>;
    /// Removes a relationship with another user by ID. Removes friends, cancels
    /// friend requests (incoming or outgoing), or unblocks a blocked user
    /// depending on current relationship type.
    #[cfg(feature = "user_api")]
    async fn remove_relationship(&self, ctx: &Context) -> Result<(), Error>;
    #[cfg(feature = "user_api")]
    async fn update_friend_nickname(
        &self,
        ctx: &Context,
        nickname: Option<String>,
    ) -> Result<Relationship, Error>;
    /// May respect privacy settings.
    async fn get_profile(
        &self,
        ctx: &Context,
        params: GetUserProfileParams,
    ) -> Result<UserProfileFullResponse, Error>;
    async fn get_user(&self, ctx: &Context) -> Result<PartialUser, Error>;
}

#[async_trait]
impl<T: UserTrait> UserExt for T {
    #[cfg(feature = "user_api")]
    async fn send_friend_request(&self, ctx: &Context) -> Result<Relationship, Error> {
        use neptunium_http::endpoints::users::SendFriendRequest;

        Ok(ctx
            .get_http_client()
            .execute(SendFriendRequest {
                user_id: self.get_user_id(),
            })
            .await?)
    }

    #[cfg(feature = "user_api")]
    async fn set_user_note(&self, ctx: &Context, note: Option<String>) -> Result<(), Error> {
        use neptunium_http::endpoints::users::SetUserNote;

        Ok(ctx
            .get_http_client()
            .execute(SetUserNote {
                user_id: self.get_user_id(),
                note,
            })
            .await?)
    }

    /// Retrieves a specific note the current user has written about another user.
    #[cfg(feature = "user_api")]
    async fn get_user_note(&self, ctx: &Context) -> Result<String, Error> {
        use neptunium_http::endpoints::users::GetUserNote;

        let response = ctx
            .get_http_client()
            .execute(GetUserNote {
                user_id: self.get_user_id(),
            })
            .await?;
        Ok(response.note)
    }

    #[cfg(feature = "user_api")]
    async fn remove_relationship(&self, ctx: &Context) -> Result<(), Error> {
        use neptunium_http::endpoints::users::RemoveRelationship;

        Ok(ctx
            .get_http_client()
            .execute(RemoveRelationship {
                user_id: self.get_user_id(),
            })
            .await?)
    }

    /// Updates the nickname associated with a relationship.
    /// Nicknames are personal labels that override the user’s display name in the current user’s view.
    #[cfg(feature = "user_api")]
    async fn update_friend_nickname(
        &self,
        ctx: &Context,
        nickname: Option<String>,
    ) -> Result<Relationship, Error> {
        use neptunium_http::endpoints::users::UpdateRelationshipNickname;

        Ok(ctx
            .get_http_client()
            .execute(UpdateRelationshipNickname {
                nickname,
                user_id: self.get_user_id(),
            })
            .await?)
    }

    async fn get_profile(
        &self,
        ctx: &Context,
        params: GetUserProfileParams,
    ) -> Result<UserProfileFullResponse, Error> {
        Ok(ctx
            .get_http_client()
            .execute(GetUserProfile {
                user_id: self.get_user_id(),
                params,
            })
            .await?)
    }

    async fn get_user(&self, ctx: &Context) -> Result<PartialUser, Error> {
        Ok(ctx
            .get_http_client()
            .execute(GetUserById {
                user_id: self.get_user_id(),
            })
            .await?)
    }
}
