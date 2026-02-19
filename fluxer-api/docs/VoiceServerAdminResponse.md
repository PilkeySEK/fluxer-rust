# VoiceServerAdminResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region_id** | **String** | ID of the region this server belongs to | 
**server_id** | **String** | Unique identifier for the voice server | 
**endpoint** | **String** | Client signal WebSocket endpoint URL for the voice server | 
**is_active** | **bool** | Whether the server is currently active | 
**vip_only** | **bool** | Whether this server is restricted to VIP users | 
**required_guild_features** | **Vec<String>** | Guild features required to use this server | 
**allowed_guild_ids** | **Vec<String>** | Guild IDs explicitly allowed to use this server | 
**allowed_user_ids** | **Vec<String>** | User IDs explicitly allowed to use this server | 
**created_at** | Option<**String**> |  | 
**updated_at** | Option<**String**> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


