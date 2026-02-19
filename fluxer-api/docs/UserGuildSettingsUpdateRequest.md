# UserGuildSettingsUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message_notifications** | Option<[**models::UserNotificationSettings**](UserNotificationSettings.md)> | Default guild notification level | [optional]
**muted** | Option<**bool**> | Guild muted | [optional]
**mute_config** | Option<[**models::UserGuildSettingsUpdateRequestMuteConfig**](UserGuildSettingsUpdateRequestMuteConfig.md)> |  | [optional]
**mobile_push** | Option<**bool**> | Mobile push notifications enabled | [optional]
**suppress_everyone** | Option<**bool**> | Suppress @everyone mentions | [optional]
**suppress_roles** | Option<**bool**> | Suppress role mentions | [optional]
**hide_muted_channels** | Option<**bool**> | Hide muted channels | [optional]
**channel_overrides** | Option<[**std::collections::HashMap<String, models::UserGuildSettingsUpdateRequestChannelOverridesValue>**](UserGuildSettingsUpdateRequestChannelOverridesValue.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


