# UserUpdateWithVerificationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | Option<**String**> |  | [optional]
**discriminator** | Option<**String**> | The 4-digit discriminator tag | [optional]
**global_name** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**new_password** | Option<**String**> |  | [optional]
**password** | Option<**String**> |  | [optional]
**avatar** | Option<**String**> | Base64-encoded image data | [optional]
**banner** | Option<**String**> | Base64-encoded image data | [optional]
**bio** | Option<**String**> |  | [optional]
**pronouns** | Option<**String**> |  | [optional]
**accent_color** | Option<**i32**> |  | [optional]
**premium_badge_hidden** | Option<**bool**> | Whether to hide the premium badge | [optional]
**premium_badge_masked** | Option<**bool**> | Whether to mask the premium badge | [optional]
**premium_badge_timestamp_hidden** | Option<**bool**> | Whether to hide premium badge timestamp | [optional]
**premium_badge_sequence_hidden** | Option<**bool**> | Whether to hide premium badge sequence | [optional]
**premium_enabled_override** | Option<**bool**> | Override premium enabled state | [optional]
**has_dismissed_premium_onboarding** | Option<**bool**> | Whether user dismissed premium onboarding | [optional]
**has_unread_gift_inventory** | Option<**bool**> | Whether user has unread gifts | [optional]
**used_mobile_client** | Option<**bool**> | Whether user has used mobile client | [optional]
**email_token** | Option<**String**> | Email change token for updating email | [optional]
**mfa_method** | Option<**MfaMethod**> | MFA method to use for verification (enum: totp, sms, webauthn) | [optional]
**mfa_code** | Option<**String**> | MFA verification code from authenticator app or SMS | [optional]
**webauthn_response** | Option<**serde_json::Value**> | WebAuthn authentication response | [optional]
**webauthn_challenge** | Option<**String**> | WebAuthn challenge string | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


