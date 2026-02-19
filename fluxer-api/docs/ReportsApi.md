# \ReportsApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dsa_report**](ReportsApi.md#create_dsa_report) | **POST** /reports/dsa | Create DSA report
[**report_guild**](ReportsApi.md#report_guild) | **POST** /reports/guild | Report guild
[**report_message**](ReportsApi.md#report_message) | **POST** /reports/message | Report message
[**report_user**](ReportsApi.md#report_user) | **POST** /reports/user | Report user
[**send_dsa_report_email**](ReportsApi.md#send_dsa_report_email) | **POST** /reports/dsa/email/send | Send DSA report email
[**verify_dsa_report_email**](ReportsApi.md#verify_dsa_report_email) | **POST** /reports/dsa/email/verify | Verify DSA report email



## create_dsa_report

> models::ReportResponse create_dsa_report(dsa_report_request)
Create DSA report

Creates a DSA complaint report with verified email for Digital Services Act compliance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dsa_report_request** | [**DsaReportRequest**](DsaReportRequest.md) |  | [required] |

### Return type

[**models::ReportResponse**](ReportResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_guild

> models::ReportResponse report_guild(report_guild_request)
Report guild

Submits a report about a guild to moderators for policy violation review.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_guild_request** | [**ReportGuildRequest**](ReportGuildRequest.md) |  | [required] |

### Return type

[**models::ReportResponse**](ReportResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_message

> models::ReportResponse report_message(report_message_request)
Report message

Submits a report about a message to moderators for content violation review.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_message_request** | [**ReportMessageRequest**](ReportMessageRequest.md) |  | [required] |

### Return type

[**models::ReportResponse**](ReportResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_user

> models::ReportResponse report_user(report_user_request)
Report user

Submits a report about a user to moderators for content violation or behaviour review.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_user_request** | [**ReportUserRequest**](ReportUserRequest.md) |  | [required] |

### Return type

[**models::ReportResponse**](ReportResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_dsa_report_email

> models::OkResponse send_dsa_report_email(dsa_report_email_send_request)
Send DSA report email

Initiates DSA (Digital Services Act) report submission by sending verification email to reporter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dsa_report_email_send_request** | [**DsaReportEmailSendRequest**](DsaReportEmailSendRequest.md) |  | [required] |

### Return type

[**models::OkResponse**](OkResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_dsa_report_email

> models::TicketResponse verify_dsa_report_email(dsa_report_email_verify_request)
Verify DSA report email

Verifies the DSA report email and creates a report ticket for legal compliance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dsa_report_email_verify_request** | [**DsaReportEmailVerifyRequest**](DsaReportEmailVerifyRequest.md) |  | [required] |

### Return type

[**models::TicketResponse**](TicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

