# UpdateVoiceRegionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the voice region | 
**name** | Option<**String**> | Display name of the voice region | [optional]
**emoji** | Option<**String**> | Emoji representing the region | [optional]
**latitude** | Option<**f64**> | Geographic latitude coordinate | [optional]
**longitude** | Option<**f64**> | Geographic longitude coordinate | [optional]
**is_default** | Option<**bool**> | Whether this is the default region | [optional]
**vip_only** | Option<**bool**> | Whether this region is restricted to VIP users | [optional]
**required_guild_features** | Option<**Vec<String>**> | Guild features required to use this region | [optional]
**allowed_guild_ids** | Option<**Vec<String>**> | Guild IDs explicitly allowed to use this region | [optional]
**allowed_user_ids** | Option<**Vec<String>**> | User IDs explicitly allowed to use this region | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


