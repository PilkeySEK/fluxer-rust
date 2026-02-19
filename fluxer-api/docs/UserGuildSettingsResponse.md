# UserGuildSettingsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**guild_id** | Option<**String**> |  | 
**message_notifications** | [**models::UserNotificationSettings**](UserNotificationSettings.md) | The default notification level for the guild | 
**muted** | **bool** | Whether the guild is muted | 
**mute_config** | Option<[**models::UserGuildSettingsResponseMuteConfig**](UserGuildSettingsResponseMuteConfig.md)> |  | 
**mobile_push** | **bool** | Whether mobile push notifications are enabled | 
**suppress_everyone** | **bool** | Whether @everyone mentions are suppressed | 
**suppress_roles** | **bool** | Whether role mentions are suppressed | 
**hide_muted_channels** | **bool** | Whether muted channels are hidden in the sidebar | 
**channel_overrides** | Option<[**std::collections::HashMap<String, models::UserGuildSettingsResponseChannelOverridesValue>**](UserGuildSettingsResponseChannelOverridesValue.md)> |  | 
**version** | **i32** | The version number of these settings for sync | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


