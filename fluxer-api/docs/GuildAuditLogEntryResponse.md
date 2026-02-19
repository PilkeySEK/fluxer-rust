# GuildAuditLogEntryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for this audit log entry | 
**action_type** | [**models::AuditLogActionType**](AuditLogActionType.md) |  | 
**user_id** | Option<**String**> |  | [optional]
**target_id** | Option<**String**> |  | [optional]
**reason** | Option<**String**> | The reason provided for the action | [optional]
**options** | Option<[**models::GuildAuditLogEntryResponseOptions**](GuildAuditLogEntryResponseOptions.md)> |  | [optional]
**changes** | Option<[**Vec<models::AuditLogChangeSchema>**](AuditLogChangeSchema.md)> | Changes made to the target | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


