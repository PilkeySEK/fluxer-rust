pub mod event;
pub mod snowflake;

/*use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserPrivateResponse {
    pub accent_color: Option<i32>,
    pub acls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_types: Vec<UserAuthenticatorTypes>,
    pub avatar: Option<String>,
    pub avatar_color: Option<i32>,
    pub banner: Option<String>,
    pub banner_color: Option<i32>,
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    pub discriminator: String,
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_bounced: Option<bool>,
    pub flags: PublicUserFlags,
    pub global_name: Option<String>,
    pub has_dismissed_premium_onboarding: bool,
    pub has_ever_purchased: bool,
    pub has_unread_gift_inventory: bool,
    pub id: Snowflake,
    pub is_staff: bool,
    pub mfa_enabled: bool,
    pub nsfw_allowed: bool,
    pub password_last_changed_at: Option<String>,
    pub pending_bulk_message_deletion: Option<UserPrivateResponsePendingBulkMessageDeletion>,
    pub phone: Option<String>,
    pub premium_badge_hidden: bool,

}*/
