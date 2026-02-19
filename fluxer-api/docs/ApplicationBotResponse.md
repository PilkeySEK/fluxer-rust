# ApplicationBotResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the bot user | 
**username** | **String** | The username of the bot | 
**discriminator** | **String** | The discriminator of the bot | 
**avatar** | Option<**String**> |  | [optional]
**banner** | Option<**String**> |  | [optional]
**bio** | Option<**String**> |  | 
**token** | Option<**String**> | The bot token for authentication | [optional]
**mfa_enabled** | Option<**bool**> | Whether the bot has MFA enabled | [optional]
**authenticator_types** | Option<[**Vec<models::AuthenticatorType>**](AuthenticatorType.md)> | The types of authenticators enabled | [optional]
**flags** | **i32** | The bot user flags | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


