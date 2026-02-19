# ChannelResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier (snowflake) for this channel | 
**guild_id** | Option<**String**> |  | [optional]
**name** | Option<**String**> | The name of the channel | [optional]
**topic** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**icon** | Option<**String**> |  | [optional]
**owner_id** | Option<**String**> |  | [optional]
**r#type** | **i32** | The type of the channel | 
**position** | Option<**i32**> |  | [optional]
**parent_id** | Option<**String**> |  | [optional]
**bitrate** | Option<**i32**> |  | [optional]
**user_limit** | Option<**i32**> |  | [optional]
**rtc_region** | Option<**String**> |  | [optional]
**last_message_id** | Option<**String**> |  | [optional]
**last_pin_timestamp** | Option<**String**> |  | [optional]
**permission_overwrites** | Option<[**Vec<models::ChannelOverwriteResponse>**](ChannelOverwriteResponse.md)> | The permission overwrites for this channel | [optional]
**recipients** | Option<[**Vec<models::UserPartialResponse>**](UserPartialResponse.md)> | The recipients of the DM channel | [optional]
**nsfw** | Option<**bool**> | Whether the channel is marked as NSFW | [optional]
**rate_limit_per_user** | Option<**i32**> |  | [optional]
**nicks** | Option<**std::collections::HashMap<String, String>**> | Custom nicknames for users in this channel (for group DMs) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


