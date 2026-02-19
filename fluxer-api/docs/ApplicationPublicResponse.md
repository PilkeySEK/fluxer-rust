# ApplicationPublicResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the application | 
**name** | **String** | The name of the application | 
**icon** | Option<**String**> |  | 
**description** | Option<**String**> |  | 
**redirect_uris** | **Vec<String>** | The registered redirect URIs for OAuth2 | 
**scopes** | **Vec<String>** | The available OAuth2 scopes | 
**bot_public** | **bool** | Whether the bot can be invited by anyone | 
**bot** | Option<[**models::ApplicationBotResponse**](ApplicationBotResponse.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


