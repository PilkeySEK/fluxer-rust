# \AuthApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authenticate_with_webauthn**](AuthApi.md#authenticate_with_webauthn) | **POST** /auth/webauthn/authenticate | Authenticate with WebAuthn
[**authorize_ip_address**](AuthApi.md#authorize_ip_address) | **POST** /auth/authorize-ip | Authorize IP address
[**cancel_handoff**](AuthApi.md#cancel_handoff) | **DELETE** /auth/handoff/{code} | Cancel handoff
[**complete_handoff**](AuthApi.md#complete_handoff) | **POST** /auth/handoff/complete | Complete handoff
[**complete_sso**](AuthApi.md#complete_sso) | **POST** /auth/sso/complete | Complete SSO
[**forgot_password**](AuthApi.md#forgot_password) | **POST** /auth/forgot | Forgot password
[**get_handoff_status**](AuthApi.md#get_handoff_status) | **GET** /auth/handoff/{code}/status | Get handoff status
[**get_sso_status**](AuthApi.md#get_sso_status) | **GET** /auth/sso/status | Get SSO status
[**get_username_suggestions**](AuthApi.md#get_username_suggestions) | **POST** /auth/username-suggestions | Get username suggestions
[**get_webauthn_authentication_options**](AuthApi.md#get_webauthn_authentication_options) | **POST** /auth/webauthn/authentication-options | Get WebAuthn authentication options
[**get_webauthn_mfa_options**](AuthApi.md#get_webauthn_mfa_options) | **POST** /auth/login/mfa/webauthn/authentication-options | Get WebAuthn MFA options
[**initiate_handoff**](AuthApi.md#initiate_handoff) | **POST** /auth/handoff/initiate | Initiate handoff
[**list_auth_sessions**](AuthApi.md#list_auth_sessions) | **GET** /auth/sessions | List auth sessions
[**login_user**](AuthApi.md#login_user) | **POST** /auth/login | Login account
[**login_with_sms_mfa**](AuthApi.md#login_with_sms_mfa) | **POST** /auth/login/mfa/sms | Login with SMS MFA
[**login_with_totp**](AuthApi.md#login_with_totp) | **POST** /auth/login/mfa/totp | Login with TOTP
[**login_with_webauthn_mfa**](AuthApi.md#login_with_webauthn_mfa) | **POST** /auth/login/mfa/webauthn | Login with WebAuthn MFA
[**logout_all_sessions**](AuthApi.md#logout_all_sessions) | **POST** /auth/sessions/logout | Logout all sessions
[**logout_user**](AuthApi.md#logout_user) | **POST** /auth/logout | Logout account
[**poll_ip_authorization**](AuthApi.md#poll_ip_authorization) | **GET** /auth/ip-authorization/poll | Poll IP authorization
[**register_account**](AuthApi.md#register_account) | **POST** /auth/register | Register account
[**resend_ip_authorization**](AuthApi.md#resend_ip_authorization) | **POST** /auth/ip-authorization/resend | Resend IP authorization
[**resend_verification_email**](AuthApi.md#resend_verification_email) | **POST** /auth/verify/resend | Resend verification email
[**reset_password**](AuthApi.md#reset_password) | **POST** /auth/reset | Reset password
[**revert_email_change**](AuthApi.md#revert_email_change) | **POST** /auth/email-revert | Revert email change
[**send_sms_mfa_code**](AuthApi.md#send_sms_mfa_code) | **POST** /auth/login/mfa/sms/send | Send SMS MFA code
[**start_sso**](AuthApi.md#start_sso) | **POST** /auth/sso/start | Start SSO
[**verify_email**](AuthApi.md#verify_email) | **POST** /auth/verify | Verify email



## authenticate_with_webauthn

> models::AuthTokenWithUserIdResponse authenticate_with_webauthn(web_authn_authenticate_request)
Authenticate with WebAuthn

Complete passwordless login using WebAuthn (biometrics or security key). Returns authentication token on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_authn_authenticate_request** | [**WebAuthnAuthenticateRequest**](WebAuthnAuthenticateRequest.md) |  | [required] |

### Return type

[**models::AuthTokenWithUserIdResponse**](AuthTokenWithUserIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authorize_ip_address

> authorize_ip_address(authorize_ip_request)
Authorize IP address

Verify and authorize a new IP address using the confirmation code sent via email. Completes IP authorization flow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorize_ip_request** | [**AuthorizeIpRequest**](AuthorizeIpRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_handoff

> cancel_handoff(code)
Cancel handoff

Cancel an ongoing handoff session. The handoff code will no longer be valid for authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_handoff

> complete_handoff(handoff_complete_request)
Complete handoff

Complete the handoff process and authenticate on the target device using the handoff code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handoff_complete_request** | [**HandoffCompleteRequest**](HandoffCompleteRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_sso

> models::SsoCompleteResponse complete_sso(sso_complete_request)
Complete SSO

Complete the SSO authentication flow with the authorization code from the SSO provider. Returns authentication token and user information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sso_complete_request** | [**SsoCompleteRequest**](SsoCompleteRequest.md) |  | [required] |

### Return type

[**models::SsoCompleteResponse**](SsoCompleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forgot_password

> forgot_password(forgot_password_request)
Forgot password

Initiate password reset process by email. A password reset link will be sent to the user's email address. Requires CAPTCHA verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**forgot_password_request** | [**ForgotPasswordRequest**](ForgotPasswordRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_handoff_status

> models::HandoffStatusResponse get_handoff_status(code)
Get handoff status

Check the status of a handoff session. Returns whether the handoff has been completed or is still pending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code | [required] |

### Return type

[**models::HandoffStatusResponse**](HandoffStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sso_status

> models::SsoStatusResponse get_sso_status()
Get SSO status

Retrieve the current status of the SSO authentication session without authentication required.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SsoStatusResponse**](SsoStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_username_suggestions

> models::UsernameSuggestionsResponse get_username_suggestions(username_suggestions_request)
Get username suggestions

Generate username suggestions based on a provided global name for new account registration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username_suggestions_request** | [**UsernameSuggestionsRequest**](UsernameSuggestionsRequest.md) |  | [required] |

### Return type

[**models::UsernameSuggestionsResponse**](UsernameSuggestionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webauthn_authentication_options

> serde_json::Value get_webauthn_authentication_options()
Get WebAuthn authentication options

Retrieve WebAuthn authentication challenge and options for passwordless login with biometrics or security keys.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webauthn_mfa_options

> serde_json::Value get_webauthn_mfa_options(mfa_ticket_request)
Get WebAuthn MFA options

Retrieve WebAuthn challenge and options for multi-factor authentication. Requires the MFA ticket from initial login.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mfa_ticket_request** | [**MfaTicketRequest**](MfaTicketRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initiate_handoff

> models::HandoffInitiateResponse initiate_handoff()
Initiate handoff

Start a handoff session to transfer authentication between devices. Returns a handoff code for device linking.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HandoffInitiateResponse**](HandoffInitiateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_auth_sessions

> Vec<models::AuthSessionResponse> list_auth_sessions()
List auth sessions

Retrieve all active authentication sessions for the current user. Requires authentication.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AuthSessionResponse>**](AuthSessionResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_user

> models::AuthLoginResponse login_user(login_request)
Login account

Authenticate with email and password. Returns authentication token if credentials are valid and MFA is not required. If MFA is enabled, returns a ticket for MFA verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_request** | [**LoginRequest**](LoginRequest.md) |  | [required] |

### Return type

[**models::AuthLoginResponse**](AuthLoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_with_sms_mfa

> models::AuthTokenWithUserIdResponse login_with_sms_mfa(mfa_sms_request)
Login with SMS MFA

Complete login by verifying the SMS code sent during MFA authentication. Requires the MFA ticket from initial login attempt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mfa_sms_request** | [**MfaSmsRequest**](MfaSmsRequest.md) |  | [required] |

### Return type

[**models::AuthTokenWithUserIdResponse**](AuthTokenWithUserIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_with_totp

> models::AuthTokenWithUserIdResponse login_with_totp(mfa_totp_request)
Login with TOTP

Complete login by verifying TOTP code during multi-factor authentication. Requires the MFA ticket from initial login attempt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mfa_totp_request** | [**MfaTotpRequest**](MfaTotpRequest.md) |  | [required] |

### Return type

[**models::AuthTokenWithUserIdResponse**](AuthTokenWithUserIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_with_webauthn_mfa

> models::AuthTokenWithUserIdResponse login_with_webauthn_mfa(web_authn_mfa_request)
Login with WebAuthn MFA

Complete login by verifying WebAuthn response during MFA. Requires the MFA ticket from initial login attempt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_authn_mfa_request** | [**WebAuthnMfaRequest**](WebAuthnMfaRequest.md) |  | [required] |

### Return type

[**models::AuthTokenWithUserIdResponse**](AuthTokenWithUserIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout_all_sessions

> logout_all_sessions(logout_auth_sessions_request)
Logout all sessions

Invalidate all active authentication sessions for the current user. Requires sudo mode verification for security.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logout_auth_sessions_request** | [**LogoutAuthSessionsRequest**](LogoutAuthSessionsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout_user

> logout_user()
Logout account

Invalidate the current authentication token and end the session. The auth token in the Authorization header will no longer be valid.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## poll_ip_authorization

> models::IpAuthorizationPollResponse poll_ip_authorization(ticket)
Poll IP authorization

Poll the status of an IP authorization request. Use the ticket parameter to check if verification has been completed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket** | **String** |  | [required] |

### Return type

[**models::IpAuthorizationPollResponse**](IpAuthorizationPollResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_account

> models::AuthRegisterResponse register_account(register_request)
Register account

Create a new user account with email and password. Requires CAPTCHA verification. User account is created but must verify email before logging in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_request** | [**RegisterRequest**](RegisterRequest.md) |  | [required] |

### Return type

[**models::AuthRegisterResponse**](AuthRegisterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_ip_authorization

> resend_ip_authorization(mfa_ticket_request)
Resend IP authorization

Request a new IP authorization verification code to be sent via email. Use if the original code was lost or expired.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mfa_ticket_request** | [**MfaTicketRequest**](MfaTicketRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_verification_email

> resend_verification_email()
Resend verification email

Request a new email verification code to be sent. Requires authentication. Use this if the original verification email was lost or expired.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_password

> models::AuthLoginResponse reset_password(reset_password_request)
Reset password

Complete the password reset flow using the token from the reset email. Returns authentication token after successful password reset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reset_password_request** | [**ResetPasswordRequest**](ResetPasswordRequest.md) |  | [required] |

### Return type

[**models::AuthLoginResponse**](AuthLoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revert_email_change

> models::AuthLoginResponse revert_email_change(email_revert_request)
Revert email change

Revert a pending email change using the verification token sent to the old email. Returns authentication token after successful revert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_revert_request** | [**EmailRevertRequest**](EmailRevertRequest.md) |  | [required] |

### Return type

[**models::AuthLoginResponse**](AuthLoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_sms_mfa_code

> send_sms_mfa_code(mfa_ticket_request)
Send SMS MFA code

Request an SMS code to be sent to the user's registered phone number during MFA login. Requires the MFA ticket from initial login attempt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mfa_ticket_request** | [**MfaTicketRequest**](MfaTicketRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_sso

> models::SsoStartResponse start_sso(sso_start_request)
Start SSO

Initiate a new Single Sign-On (SSO) session. Returns a session URL to be completed with SSO provider credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sso_start_request** | [**SsoStartRequest**](SsoStartRequest.md) |  | [required] |

### Return type

[**models::SsoStartResponse**](SsoStartResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_email

> verify_email(verify_email_request)
Verify email

Verify user email address using the code sent during registration. Email verification is required before the account becomes fully usable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_email_request** | [**VerifyEmailRequest**](VerifyEmailRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

