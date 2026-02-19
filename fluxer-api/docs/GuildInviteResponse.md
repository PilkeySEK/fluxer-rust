# GuildInviteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | The unique invite code | 
**r#type** | **Type** | The type of invite (guild) (enum: 0) | 
**guild** | [**models::GuildInviteMetadataResponseGuild**](GuildInviteMetadataResponseGuild.md) |  | 
**channel** | [**models::ChannelPartialResponse**](ChannelPartialResponse.md) |  | 
**inviter** | Option<[**models::UserPartialResponse**](UserPartialResponse.md)> |  | [optional]
**member_count** | **i32** | The approximate total member count of the guild | 
**presence_count** | **i32** | The approximate online member count of the guild | 
**expires_at** | Option<**String**> |  | [optional]
**temporary** | **bool** | Whether the invite grants temporary membership | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


