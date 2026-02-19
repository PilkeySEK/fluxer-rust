# UpdateGuildSettingsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**guild_id** | **String** |  | 
**verification_level** | Option<[**models::GuildVerificationLevel**](GuildVerificationLevel.md)> | Required verification level for guild members | [optional]
**mfa_level** | Option<[**models::GuildMfaLevel**](GuildMFALevel.md)> | Required MFA level for moderators | [optional]
**nsfw_level** | Option<[**models::NsfwLevel**](NSFWLevel.md)> | NSFW content level for the guild | [optional]
**explicit_content_filter** | Option<[**models::GuildExplicitContentFilter**](GuildExplicitContentFilter.md)> | Explicit content filter level | [optional]
**default_message_notifications** | Option<[**models::DefaultMessageNotifications**](DefaultMessageNotifications.md)> | Default notification setting for new members | [optional]
**disabled_operations** | Option<**i32**> | Bitmask of disabled guild operations | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


