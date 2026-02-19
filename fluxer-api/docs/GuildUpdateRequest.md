# GuildUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the guild (1-100 characters) | [optional]
**icon** | Option<**String**> | Base64-encoded image data | [optional]
**system_channel_id** | Option<**String**> |  | [optional]
**system_channel_flags** | Option<**i32**> | System channel message flags | [optional]
**afk_channel_id** | Option<**String**> |  | [optional]
**afk_timeout** | Option<**i32**> | AFK timeout in seconds (60-3600) before moving users to the AFK channel | [optional]
**default_message_notifications** | Option<[**models::DefaultMessageNotifications**](DefaultMessageNotifications.md)> | Default notification level for new members | [optional]
**verification_level** | Option<[**models::GuildVerificationLevel**](GuildVerificationLevel.md)> | Required verification level for members to participate | [optional]
**mfa_level** | Option<[**models::GuildMfaLevel**](GuildMFALevel.md)> | Required MFA level for moderation actions | [optional]
**nsfw_level** | Option<[**models::NsfwLevel**](NSFWLevel.md)> | The NSFW level of the guild | [optional]
**explicit_content_filter** | Option<[**models::GuildExplicitContentFilter**](GuildExplicitContentFilter.md)> | Level of content filtering for explicit media | [optional]
**banner** | Option<**String**> | Base64-encoded image data | [optional]
**splash** | Option<**String**> | Base64-encoded image data | [optional]
**embed_splash** | Option<**String**> | Base64-encoded image data | [optional]
**splash_card_alignment** | Option<**SplashCardAlignment**> | Alignment of the splash card (center, left, or right) (enum: 0, 1, 2) | [optional]
**features** | Option<**Vec<String>**> | Array of guild feature strings | [optional]
**message_history_cutoff** | Option<**String**> |  | [optional]
**password** | Option<**String**> |  | [optional]
**mfa_method** | Option<**MfaMethod**> | MFA method to use for verification (enum: totp, sms, webauthn) | [optional]
**mfa_code** | Option<**String**> | MFA verification code from authenticator app or SMS | [optional]
**webauthn_response** | Option<**serde_json::Value**> | WebAuthn authentication response | [optional]
**webauthn_challenge** | Option<**String**> | WebAuthn challenge string | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


