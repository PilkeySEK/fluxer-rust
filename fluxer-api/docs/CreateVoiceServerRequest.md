# CreateVoiceServerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region_id** | **String** | ID of the region this server belongs to | 
**server_id** | **String** | Unique identifier for the voice server | 
**endpoint** | **String** | Client signal WebSocket endpoint URL for the voice server | 
**api_key** | **String** | API key for authenticating with the voice server | 
**api_secret** | **String** | API secret for authenticating with the voice server | 
**is_active** | Option<**bool**> | Whether the server is currently active | [optional]
**vip_only** | Option<**bool**> | Whether this server is restricted to VIP users | [optional]
**required_guild_features** | Option<**Vec<String>**> | Guild features required to use this server | [optional]
**allowed_guild_ids** | Option<**Vec<String>**> | Guild IDs explicitly allowed to use this server | [optional]
**allowed_user_ids** | Option<**Vec<String>**> | User IDs explicitly allowed to use this server | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


