# SearchAuditLogsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | Option<**String**> | Search query string | [optional]
**admin_user_id** | Option<**String**> |  | [optional]
**target_id** | Option<**String**> | Filter by target entity ID (user, channel, role, invite code, etc.) | [optional]
**sort_by** | Option<**SortBy**> | Field to sort audit logs by (enum: createdAt, relevance) | [optional]
**sort_order** | Option<**SortOrder**> | Sort order direction (enum: asc, desc) | [optional]
**limit** | Option<**i32**> | Maximum number of entries to return | [optional]
**offset** | Option<**i64**> | Number of entries to skip | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


