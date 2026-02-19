# HarvestStatusResponseSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**harvest_id** | **String** | Unique identifier for the harvest request | 
**status** | [**models::HarvestStatusEnum**](HarvestStatusEnum.md) |  | 
**created_at** | **String** | ISO 8601 timestamp when the harvest request was created | 
**started_at** | Option<**String**> |  | 
**completed_at** | Option<**String**> |  | 
**failed_at** | Option<**String**> |  | 
**file_size** | Option<**String**> |  | 
**progress_percent** | **f64** | Harvest progress as a percentage value between 0 and 100 | 
**progress_step** | Option<**String**> |  | 
**error_message** | Option<**String**> |  | 
**download_url_expires_at** | Option<**String**> |  | 
**expires_at** | Option<**String**> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


