# UserProfileFullResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | [**models::UserProfileFullResponseUser**](UserProfileFullResponseUser.md) |  | 
**user_profile** | [**models::UserProfileFullResponseUserProfile**](UserProfileFullResponseUserProfile.md) |  | 
**guild_member** | Option<[**models::GuildMemberResponse**](GuildMemberResponse.md)> |  | [optional]
**guild_member_profile** | Option<[**models::UserProfileFullResponseGuildMemberProfile**](UserProfileFullResponseGuildMemberProfile.md)> |  | [optional]
**premium_type** | Option<[**models::UserPremiumTypes**](UserPremiumTypes.md)> | The type of premium subscription | [optional]
**premium_since** | Option<**String**> | ISO8601 timestamp of when premium was activated | [optional]
**premium_lifetime_sequence** | Option<**i32**> |  | [optional]
**mutual_friends** | Option<[**Vec<models::UserPartialResponse>**](UserPartialResponse.md)> | Array of mutual friends | [optional]
**mutual_guilds** | Option<[**Vec<models::MutualGuildResponse>**](MutualGuildResponse.md)> | Array of mutual guilds | [optional]
**connected_accounts** | Option<[**Vec<models::ConnectionResponse>**](ConnectionResponse.md)> | Array of verified external connections | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


