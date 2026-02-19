# ConnectionVerificationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token** | **String** | The verification token to place in DNS or profile | 
**r#type** | **Type** | The type of connection being verified (enum: bsky, domain) | 
**id** | **String** | The connection identifier (handle or domain) | 
**instructions** | **String** | Human-readable instructions for completing verification | 
**initiation_token** | **String** | Signed token the client sends back at verify time | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


