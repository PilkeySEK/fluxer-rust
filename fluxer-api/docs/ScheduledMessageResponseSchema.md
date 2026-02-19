# ScheduledMessageResponseSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for this scheduled message | 
**channel_id** | **String** | The ID of the channel this message will be sent to | 
**scheduled_at** | **String** | The ISO 8601 UTC timestamp when the message is scheduled to be sent | 
**scheduled_local_at** | **String** | The ISO 8601 timestamp in the user local timezone | 
**timezone** | **String** | The IANA timezone identifier used for scheduling | 
**status** | **Status** | The current status of the scheduled message (enum: pending, invalid, scheduled, sent, failed, cancelled) | 
**status_reason** | Option<**String**> |  | 
**payload** | [**models::ScheduledMessageResponseSchemaPayload**](ScheduledMessageResponseSchemaPayload.md) |  | 
**created_at** | **String** | The ISO 8601 timestamp when this scheduled message was created | 
**invalidated_at** | Option<**String**> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


