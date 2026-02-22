use fluxer_api::models::{ChannelResponse, GuildMemberResponse};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::OffsetDateTime;

use crate::{
    __fluxer_gateway_bitflags_as_number,
    model::{event::dispatch::session::PresenceResponse, snowflake::Snowflake},
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildDeleteDispatchData {
    /// **NOTE:** This was specified as `string` in the documentation,
    /// but a guild ID should be a `Snowflake`
    pub id: Snowflake,
    /// True if the guild is unavailable due to an outage (not a
    /// leave/kick)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable: Option<bool>,
}

#[expect(clippy::struct_excessive_bools)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VoiceStateResponse {
    guild_id: Option<Snowflake>,
    channel_id: Option<Snowflake>,
    user_id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<GuildMemberResponse>,
    mute: bool,
    deaf: bool,
    self_mute: bool,
    self_deaf: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_video: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_mobile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    viewer_stream_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<u64>,
}

#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum DefaultMessageNotifications {
    AllMessages = 0,
    MentionsOnly = 1,
}

#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum GuildExplicitContentFilter {
    None = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

__fluxer_gateway_bitflags_as_number! {
    GuildOperationsDef =>
    #[derive(Copy, Clone, Debug)]
    pub struct GuildOperations: u32 {
        const PUSH_NOTIFICATIONS = 1;
        const EVERYONE_MENTIONS = 2;
        const TYPING_EVENTS = 4;
        const INSTANT_INVITES = 8;
        const SEND_MESSAGE = 16;
        const REACTIONS = 32;
        const MEMBER_LIST_UPDATES = 64;
    }
}

/// A guild feature flag
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum GuildFeatureSchema {
    #[serde(rename = "ANIMATED_ICON")]
    AnimatedIcon,
    /// Guild can have an animated banner
    #[serde(rename = "ANIMATED_BANNER")]
    AnimatedBanner,
    /// Guild can have a banner
    #[serde(rename = "BANNER")]
    Banner,
    /// Guild banner is detached from splash
    #[serde(rename = "DETACHED_BANNER")]
    DetachedBanner,
    /// Guild can have an invite splash
    #[serde(rename = "INVITE_SPLASH")]
    InviteSplash,
    /// Guild has invites disabled
    #[serde(rename = "INVITES_DISABLED")]
    InvitesDisabled,
    /// Guild allows flexible text channel names
    #[serde(rename = "TEXT_CHANNEL_FLEXIBLE_NAMES")]
    TextChannelFlexibleNames,
    /// Guild has increased emoji slots
    #[serde(rename = "MORE_EMOJI")]
    MoreEmoji,
    /// Guild has increased sticker slots
    #[serde(rename = "MORE_STICKERS")]
    MoreStickers,
    /// Guild has unlimited emoji slots
    #[serde(rename = "UNLIMITED_EMOJI")]
    UnlimitedEmoji,
    /// Guild has unlimited sticker slots
    #[serde(rename = "UNLIMITED_STICKERS")]
    UnlimitedStickers,
    /// Guild allows purging expressions
    #[serde(rename = "EXPRESSION_PURGE_ALLOWED")]
    ExpressionPurgeAllowed,
    /// Guild can have a vanity URL
    #[serde(rename = "VANITY_URL")]
    VanityUrl,
    /// Guild is verified
    #[serde(rename = "VERIFIED")]
    Verified,
    /// Guild has VIP voice features
    #[serde(rename = "VIP_VOICE")]
    VipVoice,
    /// Guild is unavailable for everyone
    #[serde(rename = "UNAVAILABLE_FOR_EVERYONE")]
    UnavailableForEveryone,
    /// Guild is unavailable except for staff
    #[serde(rename = "UNAVAILABLE_FOR_EVERYONE_BUT_STAFF")]
    UnavailableForEveryoneButStaff,
    /// Guild is a visionary guild
    #[serde(rename = "VISIONARY")]
    Visionary,
    /// Guild is an operator guild
    #[serde(rename = "OPERATOR")]
    Operator,
    /// Guild has large guild overrides enabled
    #[serde(rename = "LARGE_GUILD_OVERRIDE")]
    LargeGuildOverride,
    /// Guild has increased member capacity enabled
    #[serde(rename = "VERY_LARGE_GUILD")]
    VeryLargeGuild,
    /// Guild has managed message scheduling
    #[serde(rename = "MT_MESSAGE_SCHEDULING")]
    MtMessageScheduling,
    /// Guild has managed expression packs
    #[serde(rename = "MT_EXPRESSION_PACKS")]
    MtExpressionPacks,
}

/// Required MFA level for moderation actions
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum GuildMfaLevel {
    NoMfaRequirement = 0,
    RequiresMfa = 1,
}

#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum NsfwLevel {
    Default = 0,
    HasExplicitContent = 1,
    IsSafe = 2,
    IsAgeRestricted = 3,
}

#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum SplashCardAlignment {
    Center = 0,
    Left = 1,
    Right = 2,
}

#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum GuildVerificationLevel {
    Unrestricted = 0,
    MustHaveVerifiedEmail,
    RegisteredForMoreThan5Minutes,
    MemberOfServerForMoreThan10Minutes,
    MustHaveVerifiedPhoneNumber,
}

__fluxer_gateway_bitflags_as_number! {
    SystemChannelFlagsDef =>
    #[derive(Copy, Clone, Debug)]
    pub struct SystemChannelFlags: u32 {
        const SUPPRESS_JOIN_NOTIFICATIONS = 1;
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildCreateProperties {
    afk_channel_id: Option<Snowflake>,
    afk_timeout: i32,
    banner: Option<String>,
    banner_height: Option<i32>,
    banner_width: Option<i32>,
    default_message_notifications: DefaultMessageNotifications,
    disabled_operations: GuildOperationsDef,
    embed_splash: Option<String>,
    embed_splash_height: Option<i32>,
    embed_splash_width: Option<i32>,
    explicit_content_filter: GuildExplicitContentFilter,
    features: Vec<GuildFeatureSchema>,
    /// Hash of the guild icon
    icon: Option<String>,
    id: Snowflake,
    #[serde(with = "time::serde::iso8601::option")]
    message_history_cutoff: Option<OffsetDateTime>,
    mfa_level: GuildMfaLevel,
    name: String,
    nsfw_level: NsfwLevel,
    owner_id: Snowflake,
    permissions: Option<String>,
    rules_channel_id: Option<Snowflake>,
    /// Hash of the guild splash screen
    splash: Option<String>,
    splash_card_alignment: SplashCardAlignment,
    splash_height: Option<i32>,
    splash_width: Option<i32>,
    system_channel_flags: SystemChannelFlagsDef,
    system_channel_id: Option<Snowflake>,
    vanity_url_code: Option<String>,
    verification_level: GuildVerificationLevel,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildEmoji {
    guild_id: Snowflake,
    id: Snowflake,
    name: String,
    creator_id: Snowflake,
    animated: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildSticker {
    guild_id: Snowflake,
    id: Snowflake,
    name: String,
    description: Option<String>,
    format_type: i32,
    tags: Option<Vec<String>>,
    creator_id: Snowflake,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildRole {
    id: Snowflake,
    name: String,
    permissions: String, // TODO: proper type
    position: i32,
    color: i64,
    /// Hash of the icon
    icon: Option<String>,
    unicode_emoji: Option<String>,
    hoist: bool,
    hoist_position: Option<i32>,
    mentionable: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildCreateDispatchData {
    properties: GuildCreateProperties,
    channels: Vec<ChannelResponse>,
    emojis: Vec<GuildEmoji>,
    members: Vec<GuildMemberResponse>,
    presences: Vec<PresenceResponse>,
    voice_states: Vec<VoiceStateResponse>,
    #[serde(with = "time::serde::iso8601")]
    joined_at: OffsetDateTime,
    stickers: Vec<GuildSticker>,
    roles: Vec<GuildRole>,
    online_count: i32,
    id: Snowflake,
    member_count: i32,
}
