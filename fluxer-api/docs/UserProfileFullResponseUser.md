# UserProfileFullResponseUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier (snowflake) for this user | 
**username** | **String** | The username of the user, not unique across the platform | 
**discriminator** | **String** | The four-digit discriminator tag of the user | 
**global_name** | Option<**String**> |  | 
**avatar** | Option<**String**> |  | 
**avatar_color** | Option<**i32**> |  | 
**bot** | Option<**bool**> | Whether the user is a bot account | [optional]
**system** | Option<**bool**> | Whether the user is an official system user | [optional]
**flags** | **i32** | The public flags on the user account | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


