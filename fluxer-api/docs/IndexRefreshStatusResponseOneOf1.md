# IndexRefreshStatusResponseOneOf1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **Status** | Current status of the index refresh job (enum: in_progress, completed, failed) | 
**index_type** | **String** | Type of index being refreshed | 
**total** | Option<**f64**> | Total number of items to index | [optional]
**indexed** | Option<**f64**> | Number of items indexed so far | [optional]
**started_at** | Option<**String**> | ISO 8601 timestamp when the job started | [optional]
**completed_at** | Option<**String**> | ISO 8601 timestamp when the job completed | [optional]
**failed_at** | Option<**String**> | ISO 8601 timestamp when the job failed | [optional]
**error** | Option<**String**> | Error message if the job failed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


