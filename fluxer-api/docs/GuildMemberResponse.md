# GuildMemberResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | [**models::UserPartialResponse**](UserPartialResponse.md) |  | 
**nick** | Option<**String**> |  | [optional]
**avatar** | Option<**String**> |  | [optional]
**banner** | Option<**String**> |  | [optional]
**accent_color** | Option<**i32**> |  | [optional]
**roles** | **Vec<String>** | Array of role IDs the member has | 
**joined_at** | **String** | ISO8601 timestamp of when the user joined the guild | 
**mute** | **bool** | Whether the member is muted in voice channels | 
**deaf** | **bool** | Whether the member is deafened in voice channels | 
**communication_disabled_until** | Option<**String**> |  | [optional]
**profile_flags** | Option<**i32**> | Member profile flags | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


