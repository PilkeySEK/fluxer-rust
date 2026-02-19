# SystemDmJobResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | **String** | Unique identifier for the job | 
**status** | **Status** | Current status of the system DM job (enum: pending, approved, running, completed, failed) | 
**content** | **String** | Message content being sent | 
**target_count** | **i32** | Total number of users targeted | 
**sent_count** | **i32** | Number of messages successfully sent | 
**failed_count** | **i32** | Number of messages that failed to send | 
**created_at** | **String** | ISO 8601 timestamp when the job was created | 
**approved_at** | Option<**String**> |  | [optional]
**registration_start** | Option<**String**> |  | [optional]
**registration_end** | Option<**String**> |  | [optional]
**excluded_guild_ids** | **Vec<String>** | List of excluded guild IDs | 
**last_error** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


