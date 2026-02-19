# AuthMfaRequiredResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mfa** | **Mfa** | Indicates MFA is required to complete authentication (enum: true) | 
**ticket** | **String** | MFA ticket to use when completing MFA verification | 
**allowed_methods** | **Vec<String>** | List of allowed MFA methods | 
**sms_phone_hint** | Option<**String**> |  | [optional]
**sms** | **bool** | Whether SMS MFA is available | 
**totp** | **bool** | Whether TOTP authenticator MFA is available | 
**webauthn** | **bool** | Whether WebAuthn security key MFA is available | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


