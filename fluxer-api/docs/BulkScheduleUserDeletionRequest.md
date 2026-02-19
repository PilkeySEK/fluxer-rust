# BulkScheduleUserDeletionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_ids** | **Vec<String>** | List of user IDs to schedule deletion for | 
**reason_code** | **i32** | Code indicating the reason for deletion | 
**public_reason** | Option<**String**> | Public-facing reason for the deletion | [optional]
**days_until_deletion** | Option<**i32**> | Number of days until the accounts are deleted | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


