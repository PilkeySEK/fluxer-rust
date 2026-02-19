# SudoVerificationSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**password** | Option<**String**> |  | [optional]
**mfa_method** | Option<**MfaMethod**> | MFA method to use for verification (enum: totp, sms, webauthn) | [optional]
**mfa_code** | Option<**String**> | MFA verification code from authenticator app or SMS | [optional]
**webauthn_response** | Option<**serde_json::Value**> | WebAuthn authentication response | [optional]
**webauthn_challenge** | Option<**String**> | WebAuthn challenge string | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


