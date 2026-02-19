# RelationshipResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for the relationship | 
**r#type** | [**models::RelationshipTypes**](RelationshipTypes.md) | The type of relationship (friend, blocked, pending, etc.) | 
**user** | [**models::UserPartialResponse**](UserPartialResponse.md) |  | 
**since** | Option<**String**> | ISO8601 timestamp of when the relationship was established | [optional]
**nickname** | Option<**String**> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


