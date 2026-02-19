# \UsersApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_or_update_friend_request**](UsersApi.md#accept_or_update_friend_request) | **PUT** /users/@me/relationships/{user_id} | Accept or update friend request
[**add_phone_to_account**](UsersApi.md#add_phone_to_account) | **POST** /users/@me/phone | Add phone number to account
[**cancel_bulk_message_deletion2**](UsersApi.md#cancel_bulk_message_deletion2) | **DELETE** /users/@me/messages/delete | Cancel bulk message deletion
[**cancel_scheduled_message**](UsersApi.md#cancel_scheduled_message) | **DELETE** /users/@me/scheduled-messages/{scheduled_message_id} | Cancel scheduled message
[**check_username_tag_availability**](UsersApi.md#check_username_tag_availability) | **GET** /users/check-tag | Check username tag availability
[**complete_password_change**](UsersApi.md#complete_password_change) | **POST** /users/@me/password-change/complete | Complete password change
[**create_private_channel**](UsersApi.md#create_private_channel) | **POST** /users/@me/channels | Create private channel
[**delete_current_user_account**](UsersApi.md#delete_current_user_account) | **POST** /users/@me/delete | Delete current user account
[**delete_mention**](UsersApi.md#delete_mention) | **DELETE** /users/@me/mentions/{message_id} | Delete mention
[**delete_webauthn_credential**](UsersApi.md#delete_webauthn_credential) | **DELETE** /users/@me/mfa/webauthn/credentials/{credential_id} | Delete WebAuthn credential
[**disable_current_user_account**](UsersApi.md#disable_current_user_account) | **POST** /users/@me/disable | Disable current user account
[**disable_sms_mfa**](UsersApi.md#disable_sms_mfa) | **POST** /users/@me/mfa/sms/disable | Disable SMS multi-factor authentication
[**disable_totp_mfa**](UsersApi.md#disable_totp_mfa) | **POST** /users/@me/mfa/totp/disable | Disable TOTP multi-factor authentication
[**enable_sms_mfa**](UsersApi.md#enable_sms_mfa) | **POST** /users/@me/mfa/sms/enable | Enable SMS multi-factor authentication
[**enable_totp_mfa**](UsersApi.md#enable_totp_mfa) | **POST** /users/@me/mfa/totp/enable | Enable TOTP multi-factor authentication
[**forget_authorized_ips**](UsersApi.md#forget_authorized_ips) | **DELETE** /users/@me/authorized-ips | Forget authorized IPs for current user
[**get_backup_codes_mfa**](UsersApi.md#get_backup_codes_mfa) | **POST** /users/@me/mfa/backup-codes | Get backup codes for multi-factor authentication
[**get_current_user**](UsersApi.md#get_current_user) | **GET** /users/@me | Get current user profile
[**get_current_user_settings**](UsersApi.md#get_current_user_settings) | **GET** /users/@me/settings | Get current user settings
[**get_data_harvest_download_url**](UsersApi.md#get_data_harvest_download_url) | **GET** /users/@me/harvest/{harvestId}/download | Get data harvest download URL
[**get_data_harvest_status**](UsersApi.md#get_data_harvest_status) | **GET** /users/@me/harvest/{harvestId} | Get data harvest status
[**get_latest_data_harvest**](UsersApi.md#get_latest_data_harvest) | **GET** /users/@me/harvest/latest | Get latest data harvest
[**get_note_on_user**](UsersApi.md#get_note_on_user) | **GET** /users/@me/notes/{target_id} | Get note on user
[**get_scheduled_message**](UsersApi.md#get_scheduled_message) | **GET** /users/@me/scheduled-messages/{scheduled_message_id} | Get scheduled message
[**get_sudo_webauthn_authentication_options**](UsersApi.md#get_sudo_webauthn_authentication_options) | **POST** /users/@me/sudo/webauthn/authentication-options | Get sudo WebAuthn authentication options
[**get_user_by_id**](UsersApi.md#get_user_by_id) | **GET** /users/{user_id} | Get user by ID
[**get_user_profile**](UsersApi.md#get_user_profile) | **GET** /users/{target_id}/profile | Get user profile
[**get_webauthn_registration_options**](UsersApi.md#get_webauthn_registration_options) | **POST** /users/@me/mfa/webauthn/credentials/registration-options | Get WebAuthn registration options
[**list_current_user_notes**](UsersApi.md#list_current_user_notes) | **GET** /users/@me/notes | List current user notes
[**list_mentions_for_current_user**](UsersApi.md#list_mentions_for_current_user) | **GET** /users/@me/mentions | List mentions for current user
[**list_private_channels**](UsersApi.md#list_private_channels) | **GET** /users/@me/channels | List private channels
[**list_push_subscriptions**](UsersApi.md#list_push_subscriptions) | **GET** /users/@me/push/subscriptions | List push subscriptions
[**list_saved_messages**](UsersApi.md#list_saved_messages) | **GET** /users/@me/saved-messages | List saved messages
[**list_scheduled_messages**](UsersApi.md#list_scheduled_messages) | **GET** /users/@me/scheduled-messages | List scheduled messages
[**list_sudo_mfa_methods**](UsersApi.md#list_sudo_mfa_methods) | **GET** /users/@me/sudo/mfa-methods | List sudo multi-factor authentication methods
[**list_user_gifts**](UsersApi.md#list_user_gifts) | **GET** /users/@me/gifts | List user gifts
[**list_user_relationships**](UsersApi.md#list_user_relationships) | **GET** /users/@me/relationships | List user relationships
[**list_webauthn_credentials**](UsersApi.md#list_webauthn_credentials) | **GET** /users/@me/mfa/webauthn/credentials | List WebAuthn credentials
[**pin_direct_message_channel**](UsersApi.md#pin_direct_message_channel) | **PUT** /users/@me/channels/{channel_id}/pin | Pin direct message channel
[**preload_messages_for_channels**](UsersApi.md#preload_messages_for_channels) | **POST** /users/@me/preload-messages | Preload messages for channels
[**preload_messages_for_channels_alt**](UsersApi.md#preload_messages_for_channels_alt) | **POST** /users/@me/channels/messages/preload | Preload messages for channels (alternative)
[**register_webauthn_credential**](UsersApi.md#register_webauthn_credential) | **POST** /users/@me/mfa/webauthn/credentials | Register WebAuthn credential
[**remove_phone_from_account**](UsersApi.md#remove_phone_from_account) | **DELETE** /users/@me/phone | Remove phone number from account
[**remove_relationship**](UsersApi.md#remove_relationship) | **DELETE** /users/@me/relationships/{user_id} | Remove relationship
[**request_bounced_email_replacement**](UsersApi.md#request_bounced_email_replacement) | **POST** /users/@me/email-change/bounced/request-new | Request replacement email for bounced address
[**request_bulk_message_deletion**](UsersApi.md#request_bulk_message_deletion) | **POST** /users/@me/messages/delete | Request bulk message deletion
[**request_data_harvest**](UsersApi.md#request_data_harvest) | **POST** /users/@me/harvest | Request data harvest
[**request_new_email_address**](UsersApi.md#request_new_email_address) | **POST** /users/@me/email-change/request-new | Request new email address
[**resend_bounced_email_replacement_code**](UsersApi.md#resend_bounced_email_replacement_code) | **POST** /users/@me/email-change/bounced/resend-new | Resend replacement email code
[**resend_new_email_confirmation**](UsersApi.md#resend_new_email_confirmation) | **POST** /users/@me/email-change/resend-new | Resend new email confirmation
[**resend_original_email_confirmation**](UsersApi.md#resend_original_email_confirmation) | **POST** /users/@me/email-change/resend-original | Resend original email confirmation
[**resend_password_change_code**](UsersApi.md#resend_password_change_code) | **POST** /users/@me/password-change/resend | Resend password change verification code
[**reset_current_user_premium_state**](UsersApi.md#reset_current_user_premium_state) | **POST** /users/@me/premium/reset | Reset current user premium state
[**save_message**](UsersApi.md#save_message) | **POST** /users/@me/saved-messages | Save message
[**send_friend_request**](UsersApi.md#send_friend_request) | **POST** /users/@me/relationships/{user_id} | Send friend request
[**send_friend_request_by_tag**](UsersApi.md#send_friend_request_by_tag) | **POST** /users/@me/relationships | Send friend request by tag
[**send_phone_verification_code**](UsersApi.md#send_phone_verification_code) | **POST** /users/@me/phone/send-verification | Send phone verification code
[**send_sudo_sms_code**](UsersApi.md#send_sudo_sms_code) | **POST** /users/@me/sudo/mfa/sms/send | Send sudo SMS code
[**set_note_on_user**](UsersApi.md#set_note_on_user) | **PUT** /users/@me/notes/{target_id} | Set note on user
[**start_email_change**](UsersApi.md#start_email_change) | **POST** /users/@me/email-change/start | Start email change
[**start_password_change**](UsersApi.md#start_password_change) | **POST** /users/@me/password-change/start | Start password change
[**subscribe_to_push_notifications**](UsersApi.md#subscribe_to_push_notifications) | **POST** /users/@me/push/subscribe | Subscribe to push notifications
[**test_bulk_message_deletion**](UsersApi.md#test_bulk_message_deletion) | **POST** /users/@me/messages/delete/test | Test bulk message deletion
[**unpin_direct_message_channel**](UsersApi.md#unpin_direct_message_channel) | **DELETE** /users/@me/channels/{channel_id}/pin | Unpin direct message channel
[**unsave_message**](UsersApi.md#unsave_message) | **DELETE** /users/@me/saved-messages/{message_id} | Unsave message
[**unsubscribe_from_push_notifications**](UsersApi.md#unsubscribe_from_push_notifications) | **DELETE** /users/@me/push/subscriptions/{subscription_id} | Unsubscribe from push notifications
[**update_current_user**](UsersApi.md#update_current_user) | **PATCH** /users/@me | Update current user profile
[**update_current_user_settings**](UsersApi.md#update_current_user_settings) | **PATCH** /users/@me/settings | Update current user settings
[**update_dm_notification_settings**](UsersApi.md#update_dm_notification_settings) | **PATCH** /users/@me/guilds/@me/settings | Update DM notification settings
[**update_guild_settings_for_user**](UsersApi.md#update_guild_settings_for_user) | **PATCH** /users/@me/guilds/{guild_id}/settings | Update guild settings for user
[**update_relationship_nickname**](UsersApi.md#update_relationship_nickname) | **PATCH** /users/@me/relationships/{user_id} | Update relationship nickname
[**update_scheduled_message**](UsersApi.md#update_scheduled_message) | **PATCH** /users/@me/scheduled-messages/{scheduled_message_id} | Update scheduled message
[**update_webauthn_credential**](UsersApi.md#update_webauthn_credential) | **PATCH** /users/@me/mfa/webauthn/credentials/{credential_id} | Update WebAuthn credential
[**verify_bounced_email_replacement**](UsersApi.md#verify_bounced_email_replacement) | **POST** /users/@me/email-change/bounced/verify-new | Verify replacement email for bounced address
[**verify_new_email_address**](UsersApi.md#verify_new_email_address) | **POST** /users/@me/email-change/verify-new | Verify new email address
[**verify_original_email_address**](UsersApi.md#verify_original_email_address) | **POST** /users/@me/email-change/verify-original | Verify original email address
[**verify_password_change_code**](UsersApi.md#verify_password_change_code) | **POST** /users/@me/password-change/verify | Verify password change code
[**verify_phone_code**](UsersApi.md#verify_phone_code) | **POST** /users/@me/phone/verify | Verify phone code



## accept_or_update_friend_request

> models::RelationshipResponse accept_or_update_friend_request(user_id, relationship_type_put_request)
Accept or update friend request

Accepts a pending incoming friend request from a user or updates the relationship type. Can also be used to change friend relationship to blocked status. Returns updated relationship object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user | [required] |
**relationship_type_put_request** | [**RelationshipTypePutRequest**](RelationshipTypePutRequest.md) |  | [required] |

### Return type

[**models::RelationshipResponse**](RelationshipResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_phone_to_account

> add_phone_to_account(phone_add_request)
Add phone number to account

Add or update the phone number associated with the current account. Requires sudo mode verification. Phone must be verified before use.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_add_request** | [**PhoneAddRequest**](PhoneAddRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_bulk_message_deletion2

> models::SuccessResponse cancel_bulk_message_deletion2()
Cancel bulk message deletion

Cancels an in-progress bulk message deletion request. Can only be used if the deletion has not yet completed. Returns success status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_scheduled_message

> cancel_scheduled_message(scheduled_message_id)
Cancel scheduled message

Cancels and deletes a scheduled message before it is sent. The message will not be delivered if cancelled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheduled_message_id** | **String** | The scheduled message id | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_username_tag_availability

> models::UserTagCheckResponse check_username_tag_availability(username, discriminator)
Check username tag availability

Checks if a username and discriminator combination is available for registration. Returns whether the tag is taken by another user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**discriminator** | **String** |  | [required] |

### Return type

[**models::UserTagCheckResponse**](UserTagCheckResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_password_change

> complete_password_change(password_change_complete_request)
Complete password change

Completes the password change after email verification. Requires the verification proof and new password. Invalidates all existing sessions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_change_complete_request** | [**PasswordChangeCompleteRequest**](PasswordChangeCompleteRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_private_channel

> models::ChannelResponse create_private_channel(create_private_channel_request)
Create private channel

Creates a new private channel (direct message) between the current user and one or more recipients. Returns the newly created channel object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_private_channel_request** | [**CreatePrivateChannelRequest**](CreatePrivateChannelRequest.md) |  | [required] |

### Return type

[**models::ChannelResponse**](ChannelResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_current_user_account

> delete_current_user_account(sudo_verification_schema)
Delete current user account

Permanently deletes the current user's account and all associated data. Requires sudo mode verification. This action is irreversible and will remove all user data, messages, and connections.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sudo_verification_schema** | [**SudoVerificationSchema**](SudoVerificationSchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_mention

> delete_mention(message_id)
Delete mention

Removes a mention from the current user's mention history. Does not delete the original message, only removes it from the user's personal mention list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | The ID of the message | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webauthn_credential

> delete_webauthn_credential(credential_id, sudo_verification_schema)
Delete WebAuthn credential

Remove a registered WebAuthn credential from the current account. Requires sudo mode verification for security.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **String** | The credential id | [required] |
**sudo_verification_schema** | [**SudoVerificationSchema**](SudoVerificationSchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_current_user_account

> disable_current_user_account(sudo_verification_schema)
Disable current user account

Temporarily disables the current user's account. Requires sudo mode verification. The account can be re-enabled by logging in again. User data is preserved but the account will be inaccessible during the disabled period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sudo_verification_schema** | [**SudoVerificationSchema**](SudoVerificationSchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_sms_mfa

> disable_sms_mfa(sudo_verification_schema)
Disable SMS multi-factor authentication

Disable SMS-based multi-factor authentication on the current account. Requires sudo mode verification for security.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sudo_verification_schema** | [**SudoVerificationSchema**](SudoVerificationSchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_totp_mfa

> disable_totp_mfa(disable_totp_request)
Disable TOTP multi-factor authentication

Disable TOTP multi-factor authentication on the current account. Requires sudo mode verification for security.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**disable_totp_request** | [**DisableTotpRequest**](DisableTotpRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_sms_mfa

> enable_sms_mfa(sudo_verification_schema)
Enable SMS multi-factor authentication

Enable SMS-based multi-factor authentication on the current account. Requires sudo mode verification and a verified phone number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sudo_verification_schema** | [**SudoVerificationSchema**](SudoVerificationSchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_totp_mfa

> models::MfaBackupCodesResponse enable_totp_mfa(enable_mfa_totp_request)
Enable TOTP multi-factor authentication

Enable time-based one-time password (TOTP) MFA on the current account. Returns backup codes for account recovery. Requires sudo mode verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enable_mfa_totp_request** | [**EnableMfaTotpRequest**](EnableMfaTotpRequest.md) |  | [required] |

### Return type

[**models::MfaBackupCodesResponse**](MfaBackupCodesResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forget_authorized_ips

> forget_authorized_ips(sudo_verification_schema)
Forget authorized IPs for current user

Clears all authorized IP addresses for the current user. After calling this endpoint, the user will be required to re-authorize any new IP addresses they log in from. Requires sudo mode verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sudo_verification_schema** | [**SudoVerificationSchema**](SudoVerificationSchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_backup_codes_mfa

> models::MfaBackupCodesResponse get_backup_codes_mfa(mfa_backup_codes_request)
Get backup codes for multi-factor authentication

Generate and retrieve new backup codes for account recovery. Requires sudo mode verification. Old codes are invalidated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mfa_backup_codes_request** | [**MfaBackupCodesRequest**](MfaBackupCodesRequest.md) |  | [required] |

### Return type

[**models::MfaBackupCodesResponse**](MfaBackupCodesResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> models::UserPrivateResponse get_current_user()
Get current user profile

Retrieves the current authenticated user's profile information, including account details and settings. OAuth2 bearer tokens require identify scope, and email is returned only when the email scope is also present. Returns full user object with private fields visible only to the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserPrivateResponse**](UserPrivateResponse.md)

### Authorization

[oauth2Token](../README.md#oauth2Token), [sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user_settings

> models::UserSettingsResponse get_current_user_settings()
Get current user settings

Retrieves the current user's settings and preferences, including notification settings, privacy options, and display preferences. Only accessible to the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserSettingsResponse**](UserSettingsResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_harvest_download_url

> models::HarvestDownloadUrlResponse get_data_harvest_download_url(harvest_id)
Get data harvest download URL

Retrieves the download URL for a completed data harvest. The URL is temporary and expires after a set time. Can only be accessed for completed harvests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**harvest_id** | **String** | The harvestId | [required] |

### Return type

[**models::HarvestDownloadUrlResponse**](HarvestDownloadUrlResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_harvest_status

> models::HarvestStatusResponseSchema get_data_harvest_status(harvest_id)
Get data harvest status

Retrieves detailed status information for a specific data harvest. Shows progress, completion status, and other metadata about the harvest request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**harvest_id** | **String** | The harvestId | [required] |

### Return type

[**models::HarvestStatusResponseSchema**](HarvestStatusResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_data_harvest

> models::HarvestStatusResponseSchemaNullable get_latest_data_harvest()
Get latest data harvest

Retrieves the status of the most recent data harvest request. Returns null if no harvest has been requested yet. Shows progress and estimated completion time.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HarvestStatusResponseSchemaNullable**](HarvestStatusResponseSchemaNullable.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_note_on_user

> models::UserNoteResponse get_note_on_user(target_id)
Get note on user

Retrieves a specific note the current user has written about another user. Returns the note text and metadata. These are private notes visible only to the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_id** | **String** | The target id | [required] |

### Return type

[**models::UserNoteResponse**](UserNoteResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scheduled_message

> models::ScheduledMessageResponseSchema get_scheduled_message(scheduled_message_id)
Get scheduled message

Retrieves details of a specific scheduled message by ID. Returns the message content, scheduled send time, and status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheduled_message_id** | **String** | The scheduled message id | [required] |

### Return type

[**models::ScheduledMessageResponseSchema**](ScheduledMessageResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sudo_webauthn_authentication_options

> models::WebAuthnChallengeResponse get_sudo_webauthn_authentication_options()
Get sudo WebAuthn authentication options

Generate WebAuthn challenge for sudo mode verification using a registered security key or biometric device.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebAuthnChallengeResponse**](WebAuthnChallengeResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_id

> models::UserPartialResponse get_user_by_id(user_id)
Get user by ID

Retrieves public user information by user ID. Returns basic profile details like username, avatar, and status. Does not include private or sensitive user data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user | [required] |

### Return type

[**models::UserPartialResponse**](UserPartialResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_profile

> models::UserProfileFullResponse get_user_profile(target_id, guild_id, with_mutual_friends, with_mutual_guilds)
Get user profile

Retrieves detailed profile information for a user, including bio, custom status, and badges. Optionally includes mutual friends and mutual guilds if requested. May respect privacy settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_id** | **String** | The target id | [required] |
**guild_id** | Option<**String**> |  |  |
**with_mutual_friends** | Option<**String**> |  |  |
**with_mutual_guilds** | Option<**String**> |  |  |

### Return type

[**models::UserProfileFullResponse**](UserProfileFullResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webauthn_registration_options

> models::WebAuthnChallengeResponse get_webauthn_registration_options(sudo_verification_schema)
Get WebAuthn registration options

Generate challenge and options to register a new WebAuthn credential. Requires sudo mode verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sudo_verification_schema** | [**SudoVerificationSchema**](SudoVerificationSchema.md) |  | [required] |

### Return type

[**models::WebAuthnChallengeResponse**](WebAuthnChallengeResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_current_user_notes

> std::collections::HashMap<String, String> list_current_user_notes()
List current user notes

Retrieves all notes the current user has written about other users. Returns a record of user IDs to notes. These are private notes visible only to the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

**std::collections::HashMap<String, String>**

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_mentions_for_current_user

> Vec<models::MessageResponseSchema> list_mentions_for_current_user(limit, roles, everyone, guilds, before)
List mentions for current user

Retrieves messages where the current user was mentioned. Supports filtering by role mentions, everyone mentions, and specific guilds. Returns paginated list of messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**String**> |  |  |
**roles** | Option<**String**> |  |  |
**everyone** | Option<**String**> |  |  |
**guilds** | Option<**String**> |  |  |
**before** | Option<**String**> |  |  |

### Return type

[**Vec<models::MessageResponseSchema>**](MessageResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_private_channels

> Vec<models::ChannelResponse> list_private_channels()
List private channels

Retrieves all private channels (direct messages) accessible to the current user. Returns list of channel objects with metadata including recipient information.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ChannelResponse>**](ChannelResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_push_subscriptions

> models::PushSubscriptionsListResponse list_push_subscriptions()
List push subscriptions

Retrieves all push notification subscriptions for the current user, including subscription IDs and user agent information for each subscription.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PushSubscriptionsListResponse**](PushSubscriptionsListResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_saved_messages

> Vec<models::SavedMessageEntryResponse> list_saved_messages(limit)
List saved messages

Retrieves all messages saved by the current user. Messages are saved privately for easy reference. Returns paginated list of saved messages with metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**String**> |  |  |

### Return type

[**Vec<models::SavedMessageEntryResponse>**](SavedMessageEntryResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scheduled_messages

> Vec<models::ScheduledMessageResponseSchema> list_scheduled_messages()
List scheduled messages

Retrieves all scheduled messages for the current user. Returns list of messages that are scheduled to be sent at a future date and time.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ScheduledMessageResponseSchema>**](ScheduledMessageResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sudo_mfa_methods

> models::SudoMfaMethodsResponse list_sudo_mfa_methods()
List sudo multi-factor authentication methods

Retrieve all available MFA methods for sudo mode verification (TOTP, SMS, WebAuthn). Requires authentication.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SudoMfaMethodsResponse**](SudoMfaMethodsResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_gifts

> Vec<models::GiftCodeMetadataResponse> list_user_gifts()
List user gifts

Lists all gift codes created by the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GiftCodeMetadataResponse>**](GiftCodeMetadataResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_relationships

> Vec<models::RelationshipResponse> list_user_relationships()
List user relationships

Retrieves all relationships for the current user, including friends, friend requests (incoming and outgoing), and blocked users. Returns list of relationship objects with type and metadata.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::RelationshipResponse>**](RelationshipResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webauthn_credentials

> Vec<models::WebAuthnCredentialResponse> list_webauthn_credentials()
List WebAuthn credentials

Retrieve all registered WebAuthn credentials (security keys, biometric devices) for the current user. Requires authentication.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::WebAuthnCredentialResponse>**](WebAuthnCredentialResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pin_direct_message_channel

> pin_direct_message_channel(channel_id)
Pin direct message channel

Pins a private message channel for the current user. Pinned channels appear at the top of the channel list for easy access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## preload_messages_for_channels

> std::collections::HashMap<String, models::PreloadMessagesResponseValue> preload_messages_for_channels(preload_messages_request)
Preload messages for channels

Preloads and caches messages for multiple channels to improve performance when opening those channels. Returns preloaded message data for the specified channels.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preload_messages_request** | [**PreloadMessagesRequest**](PreloadMessagesRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, models::PreloadMessagesResponseValue>**](PreloadMessagesResponse_value.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## preload_messages_for_channels_alt

> std::collections::HashMap<String, models::PreloadMessagesResponseValue> preload_messages_for_channels_alt(preload_messages_request)
Preload messages for channels (alternative)

Alternative endpoint to preload and cache messages for multiple channels to improve performance when opening those channels. Returns preloaded message data for the specified channels.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preload_messages_request** | [**PreloadMessagesRequest**](PreloadMessagesRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, models::PreloadMessagesResponseValue>**](PreloadMessagesResponse_value.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_webauthn_credential

> register_webauthn_credential(web_authn_register_request)
Register WebAuthn credential

Complete registration of a new WebAuthn credential (security key or biometric device). Requires sudo mode verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_authn_register_request** | [**WebAuthnRegisterRequest**](WebAuthnRegisterRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_phone_from_account

> remove_phone_from_account(sudo_verification_schema)
Remove phone number from account

Remove the phone number from the current account. Requires sudo mode verification. SMS MFA will be disabled if enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sudo_verification_schema** | [**SudoVerificationSchema**](SudoVerificationSchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_relationship

> remove_relationship(user_id)
Remove relationship

Removes a relationship with another user by ID. Removes friends, cancels friend requests (incoming or outgoing), or unblocks a blocked user depending on current relationship type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_bounced_email_replacement

> models::EmailChangeRequestNewResponse request_bounced_email_replacement(email_change_bounced_request_new_request)
Request replacement email for bounced address

Starts a dedicated bounced-email recovery flow. Sends a verification code to the replacement email without requiring verification of the old bounced email address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_change_bounced_request_new_request** | [**EmailChangeBouncedRequestNewRequest**](EmailChangeBouncedRequestNewRequest.md) |  | [required] |

### Return type

[**models::EmailChangeRequestNewResponse**](EmailChangeRequestNewResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_bulk_message_deletion

> request_bulk_message_deletion(sudo_verification_schema)
Request bulk message deletion

Initiates bulk deletion of all messages sent by the current user. Requires sudo mode verification. The deletion process is asynchronous and may take time to complete. User data remains intact.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sudo_verification_schema** | [**SudoVerificationSchema**](SudoVerificationSchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_data_harvest

> models::HarvestCreationResponseSchema request_data_harvest()
Request data harvest

Requests a data harvest of all user data and content. Initiates an asynchronous process to compile and prepare all data for download in a portable format. Returns harvest ID and status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HarvestCreationResponseSchema**](HarvestCreationResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_new_email_address

> models::EmailChangeRequestNewResponse request_new_email_address(email_change_request_new_request)
Request new email address

Requests to change email to a new address. Requires proof of original email verification. Sends confirmation code to new email address for verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_change_request_new_request** | [**EmailChangeRequestNewRequest**](EmailChangeRequestNewRequest.md) |  | [required] |

### Return type

[**models::EmailChangeRequestNewResponse**](EmailChangeRequestNewResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_bounced_email_replacement_code

> resend_bounced_email_replacement_code(email_change_ticket_request)
Resend replacement email code

Resends the verification code for the bounced-email recovery flow to the replacement email address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_change_ticket_request** | [**EmailChangeTicketRequest**](EmailChangeTicketRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_new_email_confirmation

> resend_new_email_confirmation(email_change_ticket_request)
Resend new email confirmation

Resends a confirmation code to the new email address during the email change process. Use this if the new email confirmation was not received. Requires valid email change ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_change_ticket_request** | [**EmailChangeTicketRequest**](EmailChangeTicketRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_original_email_confirmation

> resend_original_email_confirmation(email_change_ticket_request)
Resend original email confirmation

Resends a confirmation code to the user's original email address during the email change process. Use this if the original confirmation email was not received. Requires valid email change ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_change_ticket_request** | [**EmailChangeTicketRequest**](EmailChangeTicketRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_password_change_code

> resend_password_change_code(password_change_ticket_request)
Resend password change verification code

Resends the verification code for a password change. Use if the original code was not received. Requires a valid password change ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_change_ticket_request** | [**PasswordChangeTicketRequest**](PasswordChangeTicketRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_current_user_premium_state

> reset_current_user_premium_state()
Reset current user premium state

Staff-only endpoint that clears premium status and related premium metadata for the current user account.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_message

> save_message(save_message_request)
Save message

Saves a message for the current user. Saved messages can be accessed later from the saved messages list. Messages are saved privately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**save_message_request** | [**SaveMessageRequest**](SaveMessageRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_friend_request

> models::RelationshipResponse send_friend_request(user_id)
Send friend request

Sends a friend request to a user identified by user ID. Returns the new relationship object. Can fail if user not found or request already sent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user | [required] |

### Return type

[**models::RelationshipResponse**](RelationshipResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_friend_request_by_tag

> models::RelationshipResponse send_friend_request_by_tag(friend_request_by_tag_request)
Send friend request by tag

Sends a friend request to a user identified by username tag (username#discriminator). Returns the new relationship object. Can fail if user not found or request already sent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**friend_request_by_tag_request** | [**FriendRequestByTagRequest**](FriendRequestByTagRequest.md) |  | [required] |

### Return type

[**models::RelationshipResponse**](RelationshipResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_phone_verification_code

> send_phone_verification_code(phone_send_verification_request)
Send phone verification code

Request a verification code to be sent via SMS to the provided phone number. Requires authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_send_verification_request** | [**PhoneSendVerificationRequest**](PhoneSendVerificationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_sudo_sms_code

> send_sudo_sms_code()
Send sudo SMS code

Request an SMS code to be sent for sudo mode verification. Used before entering sensitive account settings.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_note_on_user

> set_note_on_user(target_id, user_note_update_request)
Set note on user

Creates or updates a private note on another user. The note is visible only to the authenticated user. Send null or empty string to delete an existing note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_id** | **String** | The target id | [required] |
**user_note_update_request** | [**UserNoteUpdateRequest**](UserNoteUpdateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_email_change

> models::EmailChangeStartResponse start_email_change(body)
Start email change

Initiates an email change process. Generates a ticket for verifying the original email address before requesting a new email. Returns ticket for use in subsequent email change steps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::EmailChangeStartResponse**](EmailChangeStartResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_password_change

> models::PasswordChangeStartResponse start_password_change(body)
Start password change

Initiates a password change process. Sends a verification code to the user's email address. Returns a ticket for use in subsequent password change steps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::PasswordChangeStartResponse**](PasswordChangeStartResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_to_push_notifications

> models::PushSubscribeResponse subscribe_to_push_notifications(push_subscribe_request)
Subscribe to push notifications

Registers a new push notification subscription for the current user. Takes push endpoint and encryption keys from a Web Push API subscription. Returns subscription ID for future reference.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**push_subscribe_request** | [**PushSubscribeRequest**](PushSubscribeRequest.md) |  | [required] |

### Return type

[**models::PushSubscribeResponse**](PushSubscribeResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_bulk_message_deletion

> test_bulk_message_deletion()
Test bulk message deletion

Staff-only endpoint for testing bulk message deletion functionality. Creates a test deletion request with a 1-minute delay. Only accessible to users with staff privileges.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpin_direct_message_channel

> unpin_direct_message_channel(channel_id)
Unpin direct message channel

Unpins a private message channel for the current user. The channel will return to its normal position in the channel list based on activity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsave_message

> unsave_message(message_id)
Unsave message

Removes a message from the current user's saved messages. Does not delete the original message, only removes it from the user's saved collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | The ID of the message | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe_from_push_notifications

> models::SuccessResponse unsubscribe_from_push_notifications(subscription_id)
Unsubscribe from push notifications

Unregisters a push notification subscription for the current user. Push notifications will no longer be sent to this subscription endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | The subscription id | [required] |

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_current_user

> models::UserPrivateResponse update_current_user(user_update_with_verification_request)
Update current user profile

Updates the authenticated user's profile information such as username, avatar, and bio. Requires sudo mode verification for security-sensitive changes. Only default users can modify their own profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_update_with_verification_request** | [**UserUpdateWithVerificationRequest**](UserUpdateWithVerificationRequest.md) |  | [required] |

### Return type

[**models::UserPrivateResponse**](UserPrivateResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_current_user_settings

> models::UserSettingsResponse update_current_user_settings(user_settings_update_request)
Update current user settings

Updates the current user's settings and preferences. Allows modification of notification settings, privacy options, display preferences, and other user-configurable options. Returns updated settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_settings_update_request** | [**UserSettingsUpdateRequest**](UserSettingsUpdateRequest.md) |  | [required] |

### Return type

[**models::UserSettingsResponse**](UserSettingsResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dm_notification_settings

> models::UserGuildSettingsResponse update_dm_notification_settings(user_guild_settings_update_request)
Update DM notification settings

Updates the user's notification settings for direct messages and group DMs. Controls how DM notifications are handled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_guild_settings_update_request** | [**UserGuildSettingsUpdateRequest**](UserGuildSettingsUpdateRequest.md) |  | [required] |

### Return type

[**models::UserGuildSettingsResponse**](UserGuildSettingsResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_settings_for_user

> models::UserGuildSettingsResponse update_guild_settings_for_user(guild_id, user_guild_settings_update_request)
Update guild settings for user

Updates the user's settings for a specific guild, such as notification preferences and visibility settings. Guild-specific settings override default settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**user_guild_settings_update_request** | [**UserGuildSettingsUpdateRequest**](UserGuildSettingsUpdateRequest.md) |  | [required] |

### Return type

[**models::UserGuildSettingsResponse**](UserGuildSettingsResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_relationship_nickname

> models::RelationshipResponse update_relationship_nickname(user_id, relationship_nickname_update_request)
Update relationship nickname

Updates the nickname associated with a relationship (friend or blocked user). Nicknames are personal labels that override the user's display name in the current user's view. Returns updated relationship object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user | [required] |
**relationship_nickname_update_request** | [**RelationshipNicknameUpdateRequest**](RelationshipNicknameUpdateRequest.md) |  | [required] |

### Return type

[**models::RelationshipResponse**](RelationshipResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_scheduled_message

> models::ScheduledMessageResponseSchema update_scheduled_message(scheduled_message_id)
Update scheduled message

Updates an existing scheduled message before it is sent. Can modify message content, scheduled time, and timezone. Returns updated scheduled message details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheduled_message_id** | **String** | The scheduled message id | [required] |

### Return type

[**models::ScheduledMessageResponseSchema**](ScheduledMessageResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webauthn_credential

> update_webauthn_credential(credential_id, web_authn_credential_update_request)
Update WebAuthn credential

Update the name or settings of a registered WebAuthn credential. Requires sudo mode verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **String** | The credential id | [required] |
**web_authn_credential_update_request** | [**WebAuthnCredentialUpdateRequest**](WebAuthnCredentialUpdateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_bounced_email_replacement

> models::UserPrivateResponse verify_bounced_email_replacement(email_change_bounced_verify_new_request)
Verify replacement email for bounced address

Completes bounced-email recovery by verifying the replacement email code, updating the account email, and clearing email-related suspicious-activity requirements.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_change_bounced_verify_new_request** | [**EmailChangeBouncedVerifyNewRequest**](EmailChangeBouncedVerifyNewRequest.md) |  | [required] |

### Return type

[**models::UserPrivateResponse**](UserPrivateResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_new_email_address

> models::EmailTokenResponse verify_new_email_address(email_change_verify_new_request)
Verify new email address

Completes the email change process by verifying the new email address with a confirmation code. Returns an email token that confirms the email change. After this step, the user may need to re-authenticate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_change_verify_new_request** | [**EmailChangeVerifyNewRequest**](EmailChangeVerifyNewRequest.md) |  | [required] |

### Return type

[**models::EmailTokenResponse**](EmailTokenResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_original_email_address

> models::EmailChangeVerifyOriginalResponse verify_original_email_address(email_change_verify_original_request)
Verify original email address

Verifies ownership of the original email address by validating a confirmation code sent to that address. Must be completed before requesting a new email address. Returns proof token for use in new email request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_change_verify_original_request** | [**EmailChangeVerifyOriginalRequest**](EmailChangeVerifyOriginalRequest.md) |  | [required] |

### Return type

[**models::EmailChangeVerifyOriginalResponse**](EmailChangeVerifyOriginalResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_password_change_code

> models::PasswordChangeVerifyResponse verify_password_change_code(password_change_verify_request)
Verify password change code

Verifies the email code sent during password change. Returns a proof token needed to complete the password change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_change_verify_request** | [**PasswordChangeVerifyRequest**](PasswordChangeVerifyRequest.md) |  | [required] |

### Return type

[**models::PasswordChangeVerifyResponse**](PasswordChangeVerifyResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_phone_code

> models::PhoneVerifyResponse verify_phone_code(phone_verify_request)
Verify phone code

Verify a phone number by confirming the SMS verification code. Returns phone verification status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_verify_request** | [**PhoneVerifyRequest**](PhoneVerifyRequest.md) |  | [required] |

### Return type

[**models::PhoneVerifyResponse**](PhoneVerifyResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

