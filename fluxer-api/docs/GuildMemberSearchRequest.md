# GuildMemberSearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | Option<**String**> | Text to search for in usernames, global names, and nicknames | [optional]
**limit** | Option<**i32**> | Maximum number of results to return | [optional]
**offset** | Option<**i64**> | Number of results to skip for pagination | [optional]
**role_ids** | Option<**Vec<String>**> | Filter by role IDs (member must have all specified roles) | [optional]
**joined_at_gte** | Option<**i32**> | Filter members who joined at or after this unix timestamp | [optional]
**joined_at_lte** | Option<**i32**> | Filter members who joined at or before this unix timestamp | [optional]
**join_source_type** | Option<**Vec<i32>**> | Filter by join source types | [optional]
**source_invite_code** | Option<**Vec<String>**> | Filter by invite codes used to join | [optional]
**is_bot** | Option<**bool**> | Filter by bot status | [optional]
**user_created_at_gte** | Option<**i32**> | Filter members whose account was created at or after this unix timestamp | [optional]
**user_created_at_lte** | Option<**i32**> | Filter members whose account was created at or before this unix timestamp | [optional]
**sort_by** | Option<**SortBy**> | Sort results by field (enum: joinedAt, relevance) | [optional]
**sort_order** | Option<**SortOrder**> | Sort order (enum: asc, desc) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


