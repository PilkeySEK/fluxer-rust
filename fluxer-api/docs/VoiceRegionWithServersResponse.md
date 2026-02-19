# VoiceRegionWithServersResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the voice region | 
**name** | **String** | Display name of the voice region | 
**emoji** | **String** | Emoji representing the region | 
**latitude** | **f64** | Geographic latitude coordinate | 
**longitude** | **f64** | Geographic longitude coordinate | 
**is_default** | **bool** | Whether this is the default region | 
**vip_only** | **bool** | Whether this region is restricted to VIP users | 
**required_guild_features** | **Vec<String>** | Guild features required to use this region | 
**allowed_guild_ids** | **Vec<String>** | Guild IDs explicitly allowed to use this region | 
**allowed_user_ids** | **Vec<String>** | User IDs explicitly allowed to use this region | 
**created_at** | Option<**String**> |  | 
**updated_at** | Option<**String**> |  | 
**servers** | Option<[**Vec<models::VoiceServerAdminResponse>**](VoiceServerAdminResponse.md)> | Voice servers in this region | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


