# \OAuth2Api

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_oauth_application**](OAuth2Api.md#create_oauth_application) | **POST** /oauth2/applications | Create OAuth2 application
[**delete_oauth_application**](OAuth2Api.md#delete_oauth_application) | **DELETE** /oauth2/applications/{id} | Delete application
[**delete_user_oauth2_authorization**](OAuth2Api.md#delete_user_oauth2_authorization) | **DELETE** /oauth2/@me/authorizations/{applicationId} | Revoke OAuth2 authorization
[**exchange_oauth2_token**](OAuth2Api.md#exchange_oauth2_token) | **POST** /oauth2/token | Exchange OAuth2 token
[**get_current_user_applications**](OAuth2Api.md#get_current_user_applications) | **GET** /applications/@me | List current user applications
[**get_current_user_oauth2**](OAuth2Api.md#get_current_user_oauth2) | **GET** /oauth2/@me | Get current OAuth2 user
[**get_oauth2_userinfo**](OAuth2Api.md#get_oauth2_userinfo) | **GET** /oauth2/userinfo | Get OAuth2 user information
[**get_oauth_application**](OAuth2Api.md#get_oauth_application) | **GET** /oauth2/applications/{id} | Get application
[**get_public_application**](OAuth2Api.md#get_public_application) | **GET** /oauth2/applications/{id}/public | Get public application
[**introspect_oauth2_token**](OAuth2Api.md#introspect_oauth2_token) | **POST** /oauth2/introspect | Introspect OAuth2 token
[**list_user_applications**](OAuth2Api.md#list_user_applications) | **GET** /users/@me/applications | List user applications
[**list_user_applications2**](OAuth2Api.md#list_user_applications2) | **GET** /oauth2/applications/@me | List user applications
[**list_user_oauth2_authorizations**](OAuth2Api.md#list_user_oauth2_authorizations) | **GET** /oauth2/@me/authorizations | List user OAuth2 authorizations
[**provide_oauth2_consent**](OAuth2Api.md#provide_oauth2_consent) | **POST** /oauth2/authorize/consent | Grant OAuth2 consent
[**reset_bot_token2**](OAuth2Api.md#reset_bot_token2) | **POST** /oauth2/applications/{id}/bot/reset-token | Reset bot token
[**reset_client_secret2**](OAuth2Api.md#reset_client_secret2) | **POST** /oauth2/applications/{id}/client-secret/reset | Reset client secret
[**revoke_oauth2_token**](OAuth2Api.md#revoke_oauth2_token) | **POST** /oauth2/token/revoke | Revoke OAuth2 token
[**update_bot_profile**](OAuth2Api.md#update_bot_profile) | **PATCH** /oauth2/applications/{id}/bot | Update bot profile
[**update_oauth_application**](OAuth2Api.md#update_oauth_application) | **PATCH** /oauth2/applications/{id} | Update application



## create_oauth_application

> models::ApplicationResponse create_oauth_application(application_create_request)
Create OAuth2 application

Creates a new OAuth2 application (client). Returns client credentials including ID and secret. Application can be used for authorization flows and API access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_create_request** | [**ApplicationCreateRequest**](ApplicationCreateRequest.md) |  | [required] |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_oauth_application

> delete_oauth_application(id, sudo_verification_schema)
Delete application

Permanently deletes an OAuth2 application. Requires sudo mode authentication. Invalidates all issued tokens and revokes all user authorizations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id | [required] |
**sudo_verification_schema** | [**SudoVerificationSchema**](SudoVerificationSchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_oauth2_authorization

> delete_user_oauth2_authorization(application_id)
Revoke OAuth2 authorization

Revokes user authorization for a third-party application. Immediately invalidates all tokens issued to that application. User regains control of delegated access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The applicationId | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exchange_oauth2_token

> models::OAuth2TokenResponse exchange_oauth2_token(grant_type, code, redirect_uri, client_id, client_secret, refresh_token)
Exchange OAuth2 token

Exchanges authorization code or other grant type for access tokens. Supports authorization code, refresh token, and client credentials flows. Client authentication via authorization header or client credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | Option<**String**> | The grant type for refreshing an access token |  |
**code** | Option<**String**> | The authorization code received from the authorize endpoint |  |
**redirect_uri** | Option<**String**> | The redirect URI used in the authorization request |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> | The application client secret |  |
**refresh_token** | Option<**String**> | The refresh token to exchange for a new access token |  |

### Return type

[**models::OAuth2TokenResponse**](OAuth2TokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user_applications

> models::ApplicationsMeResponse get_current_user_applications()
List current user applications

Lists all OAuth2 applications registered by the authenticated user. Includes application credentials and metadata. Requires valid OAuth2 access token.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApplicationsMeResponse**](ApplicationsMeResponse.md)

### Authorization

[botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user_oauth2

> models::OAuth2MeResponse get_current_user_oauth2()
Get current OAuth2 user

Retrieves current authorization details for a valid OAuth2 bearer token. Includes OAuth2 metadata and user details when identify is present.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OAuth2MeResponse**](OAuth2MeResponse.md)

### Authorization

[oauth2Token](../README.md#oauth2Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oauth2_userinfo

> models::OAuth2UserInfoResponse get_oauth2_userinfo()
Get OAuth2 user information

Retrieves authenticated user information using a valid access token. Requires identify scope and supports email scope for email fields.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OAuth2UserInfoResponse**](OAuth2UserInfoResponse.md)

### Authorization

[oauth2Token](../README.md#oauth2Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oauth_application

> models::ApplicationResponse get_oauth_application(id)
Get application

Retrieves details of a specific OAuth2 application owned by the user. Returns full application configuration and credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id | [required] |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_application

> models::ApplicationPublicResponse get_public_application(id)
Get public application

Retrieves public information about an OAuth2 application without authentication. Allows clients to discover application metadata before initiating authorization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id | [required] |

### Return type

[**models::ApplicationPublicResponse**](ApplicationPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## introspect_oauth2_token

> models::OAuth2IntrospectResponse introspect_oauth2_token(token, client_id, client_secret)
Introspect OAuth2 token

Verifies token validity and retrieves metadata. Returns active status, scope, expiration, and user information. Client authentication via authorization header or client credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | The token to introspect | [required] |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> | The application client secret |  |

### Return type

[**models::OAuth2IntrospectResponse**](OAuth2IntrospectResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_applications

> Vec<models::ApplicationResponse> list_user_applications()
List user applications

Lists all OAuth2 applications owned by the authenticated user. Includes application credentials, metadata, and configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApplicationResponse>**](ApplicationResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_applications2

> Vec<models::ApplicationResponse> list_user_applications2()
List user applications

Lists all OAuth2 applications owned by the authenticated user. Includes application credentials, metadata, and configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApplicationResponse>**](ApplicationResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_oauth2_authorizations

> Vec<models::OAuth2AuthorizationResponse> list_user_oauth2_authorizations()
List user OAuth2 authorizations

Lists all third-party applications the user has authorized. Shows granted scopes and authorization metadata. Allows user to review and manage delegated access.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::OAuth2AuthorizationResponse>**](OAuth2AuthorizationResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## provide_oauth2_consent

> models::OAuth2ConsentResponse provide_oauth2_consent(authorize_consent_request)
Grant OAuth2 consent

User grants permission for an OAuth2 application to access authorized scopes. Used in authorization code flow to complete the authorization process after user review.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorize_consent_request** | [**AuthorizeConsentRequest**](AuthorizeConsentRequest.md) |  | [required] |

### Return type

[**models::OAuth2ConsentResponse**](OAuth2ConsentResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_bot_token2

> models::BotTokenResetResponse reset_bot_token2(id, sudo_verification_schema)
Reset bot token

Rotates the bot token for an OAuth2 application. Requires sudo mode authentication. Invalidates all previously issued bot tokens. Used for security rotation and compromise mitigation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id | [required] |
**sudo_verification_schema** | [**SudoVerificationSchema**](SudoVerificationSchema.md) |  | [required] |

### Return type

[**models::BotTokenResetResponse**](BotTokenResetResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_client_secret2

> models::ApplicationResponse reset_client_secret2(id, sudo_verification_schema)
Reset client secret

Rotates the client secret for an OAuth2 application. Requires sudo mode authentication. Essential security operation for protecting client credentials. Existing access tokens remain valid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id | [required] |
**sudo_verification_schema** | [**SudoVerificationSchema**](SudoVerificationSchema.md) |  | [required] |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_oauth2_token

> revoke_oauth2_token(token, token_type_hint, client_id, client_secret)
Revoke OAuth2 token

Revokes an access or refresh token, immediately invalidating it. Client authentication required via authorization header or client credentials. Returns 200 on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | The token to revoke | [required] |
**token_type_hint** | Option<**String**> | A hint about the type of token being revoked |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> | The application client secret |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_bot_profile

> models::BotProfileResponse update_bot_profile(id, bot_profile_update_request)
Update bot profile

Modifies bot profile information such as name, avatar, and status. Changes apply to the bot account associated with this OAuth2 application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id | [required] |
**bot_profile_update_request** | [**BotProfileUpdateRequest**](BotProfileUpdateRequest.md) |  | [required] |

### Return type

[**models::BotProfileResponse**](BotProfileResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_oauth_application

> models::ApplicationResponse update_oauth_application(id, application_update_request)
Update application

Modifies OAuth2 application configuration such as name, description, and redirect URIs. Does not rotate credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id | [required] |
**application_update_request** | [**ApplicationUpdateRequest**](ApplicationUpdateRequest.md) |  | [required] |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

