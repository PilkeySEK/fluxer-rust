# GuildResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for this guild | 
**name** | **String** | The name of the guild | 
**icon** | Option<**String**> |  | [optional]
**banner** | Option<**String**> |  | [optional]
**banner_width** | Option<**i32**> |  | [optional]
**banner_height** | Option<**i32**> |  | [optional]
**splash** | Option<**String**> |  | [optional]
**splash_width** | Option<**i32**> |  | [optional]
**splash_height** | Option<**i32**> |  | [optional]
**splash_card_alignment** | **SplashCardAlignment** | The alignment of the splash card (enum: 0, 1, 2) | 
**embed_splash** | Option<**String**> |  | [optional]
**embed_splash_width** | Option<**i32**> |  | [optional]
**embed_splash_height** | Option<**i32**> |  | [optional]
**vanity_url_code** | Option<**String**> |  | [optional]
**owner_id** | **String** | The ID of the guild owner | 
**system_channel_id** | Option<**String**> |  | [optional]
**system_channel_flags** | **i32** | System channel message flags | 
**rules_channel_id** | Option<**String**> |  | [optional]
**afk_channel_id** | Option<**String**> |  | [optional]
**afk_timeout** | **i32** | AFK timeout in seconds before moving users to the AFK channel | 
**features** | **Vec<String>** | Array of guild feature flags | 
**verification_level** | [**models::GuildVerificationLevel**](GuildVerificationLevel.md) | Required verification level for members to participate | 
**mfa_level** | [**models::GuildMfaLevel**](GuildMFALevel.md) | Required MFA level for moderation actions | 
**nsfw_level** | [**models::NsfwLevel**](NSFWLevel.md) | The NSFW level of the guild | 
**explicit_content_filter** | [**models::GuildExplicitContentFilter**](GuildExplicitContentFilter.md) | Level of content filtering for explicit media | 
**default_message_notifications** | [**models::DefaultMessageNotifications**](DefaultMessageNotifications.md) | Default notification level for new members | 
**disabled_operations** | **i32** | Bitmask of disabled guild operations | 
**message_history_cutoff** | Option<**String**> |  | [optional]
**permissions** | Option<**String**> | The current user permissions in this guild | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


