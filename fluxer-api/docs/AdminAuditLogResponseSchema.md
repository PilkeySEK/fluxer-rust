# AdminAuditLogResponseSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**log_id** | **String** |  | 
**admin_user_id** | **String** |  | 
**target_type** | **String** |  | 
**target_id** | **String** | The ID of the affected entity (user, channel, role, invite code, etc.) | 
**action** | **String** |  | 
**audit_log_reason** | Option<**String**> |  | 
**metadata** | **std::collections::HashMap<String, String>** |  | 
**created_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


