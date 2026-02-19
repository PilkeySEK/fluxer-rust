# ApplicationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the application | 
**name** | **String** | The name of the application | 
**redirect_uris** | **Vec<String>** | The registered redirect URIs for OAuth2 | 
**bot_public** | **bool** | Whether the bot can be invited by anyone | 
**bot_require_code_grant** | **bool** | Whether the bot requires OAuth2 code grant | 
**client_secret** | Option<**String**> | The client secret for OAuth2 authentication | [optional]
**bot** | Option<[**models::ApplicationBotResponse**](ApplicationBotResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


