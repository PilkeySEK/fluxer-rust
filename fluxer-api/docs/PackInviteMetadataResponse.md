# PackInviteMetadataResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | The unique invite code | 
**r#type** | **Type** | The type of pack invite (emoji or sticker pack) (enum: 2, 3) | 
**pack** | [**models::PackInviteMetadataResponsePack**](PackInviteMetadataResponsePack.md) |  | 
**inviter** | Option<[**models::UserPartialResponse**](UserPartialResponse.md)> |  | [optional]
**expires_at** | Option<**String**> |  | [optional]
**temporary** | **bool** | Whether the invite grants temporary access | 
**created_at** | **String** | ISO8601 timestamp of when the invite was created | 
**uses** | **i32** | The number of times this invite has been used | 
**max_uses** | **i32** | The maximum number of times this invite can be used | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


