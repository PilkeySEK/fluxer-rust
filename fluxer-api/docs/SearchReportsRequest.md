# SearchReportsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | Option<**String**> | Search query string | [optional]
**limit** | Option<**i32**> | Maximum number of entries to return | [optional]
**offset** | Option<**i64**> | Number of entries to skip | [optional]
**reporter_id** | Option<**String**> |  | [optional]
**status** | Option<[**models::ReportStatus**](ReportStatus.md)> |  | [optional]
**report_type** | Option<[**models::ReportType**](ReportType.md)> |  | [optional]
**category** | Option<**String**> | Filter by report category | [optional]
**reported_user_id** | Option<**String**> |  | [optional]
**reported_guild_id** | Option<**String**> |  | [optional]
**reported_channel_id** | Option<**String**> |  | [optional]
**guild_context_id** | Option<**String**> |  | [optional]
**resolved_by_admin_id** | Option<**String**> |  | [optional]
**sort_by** | Option<**SortBy**> | Field to sort reports by (enum: createdAt, reportedAt, resolvedAt) | [optional]
**sort_order** | Option<**SortOrder**> | Sort order direction (enum: asc, desc) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


