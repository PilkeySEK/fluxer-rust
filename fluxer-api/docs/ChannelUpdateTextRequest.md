# ChannelUpdateTextRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**topic** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**parent_id** | Option<**String**> |  | [optional]
**bitrate** | Option<**i32**> |  | [optional]
**user_limit** | Option<**i32**> |  | [optional]
**permission_overwrites** | Option<[**Vec<models::ChannelOverwriteRequest>**](ChannelOverwriteRequest.md)> | Permission overwrites for roles and members | [optional]
**nsfw** | Option<**bool**> |  | [optional]
**rate_limit_per_user** | Option<**i32**> |  | [optional]
**icon** | Option<**String**> | Base64-encoded image data | [optional]
**owner_id** | Option<**String**> |  | [optional]
**nicks** | Option<[**std::collections::HashMap<String, models::ChannelNicknameOverridesValue>**](ChannelNicknameOverridesValue.md)> | User nickname overrides (user ID to nickname mapping) | [optional]
**rtc_region** | Option<**String**> |  | [optional]
**r#type** | **Type** |  (enum: 0) | 
**name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


