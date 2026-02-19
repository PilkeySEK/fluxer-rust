# GuildMemberSearchResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Composite ID (guildId:userId) | 
**guild_id** | **String** | Guild ID | 
**user_id** | **String** | User ID | 
**username** | **String** | Username | 
**discriminator** | **String** | Zero-padded 4-digit discriminator | 
**global_name** | Option<**String**> |  | 
**nickname** | Option<**String**> |  | 
**role_ids** | **Vec<String>** | Role IDs | 
**joined_at** | **f64** | Unix timestamp of when the member joined | 
**supplemental** | [**models::GuildMemberSearchResultSupplemental**](GuildMemberSearchResultSupplemental.md) |  | 
**is_bot** | **bool** | Whether the user is a bot | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


