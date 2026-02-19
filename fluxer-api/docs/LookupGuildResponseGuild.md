# LookupGuildResponseGuild

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**owner_id** | **String** |  | 
**name** | **String** |  | 
**vanity_url_code** | Option<**String**> |  | 
**icon** | Option<**String**> |  | 
**banner** | Option<**String**> |  | 
**splash** | Option<**String**> |  | 
**embed_splash** | Option<**String**> |  | 
**features** | **Vec<String>** |  | 
**verification_level** | [**models::GuildVerificationLevel**](GuildVerificationLevel.md) |  | 
**mfa_level** | [**models::GuildMfaLevel**](GuildMFALevel.md) |  | 
**nsfw_level** | [**models::NsfwLevel**](NSFWLevel.md) |  | 
**explicit_content_filter** | [**models::GuildExplicitContentFilter**](GuildExplicitContentFilter.md) |  | 
**default_message_notifications** | [**models::DefaultMessageNotifications**](DefaultMessageNotifications.md) |  | 
**afk_channel_id** | Option<**String**> |  | 
**afk_timeout** | **i32** |  | 
**system_channel_id** | Option<**String**> |  | 
**system_channel_flags** | **i32** | System channel message flags | 
**rules_channel_id** | Option<**String**> |  | 
**disabled_operations** | **i32** |  | 
**member_count** | **i32** |  | 
**channels** | [**Vec<models::LookupGuildResponseGuildChannelsInner>**](LookupGuildResponseGuildChannelsInner.md) |  | 
**roles** | [**Vec<models::LookupGuildResponseGuildRolesInner>**](LookupGuildResponseGuildRolesInner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


