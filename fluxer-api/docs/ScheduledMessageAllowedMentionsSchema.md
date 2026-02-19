# ScheduledMessageAllowedMentionsSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parse** | Option<**Vec<Parse>**> | Types of mentions to parse from content (enum: users, roles, everyone) | [optional]
**users** | Option<**Vec<String>**> | Array of user IDs to mention | [optional]
**roles** | Option<**Vec<String>**> | Array of role IDs to mention | [optional]
**replied_user** | Option<**bool**> | Whether to mention the author of the replied message | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


