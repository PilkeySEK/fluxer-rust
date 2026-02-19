# \AdminApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_email_ban**](AdminApi.md#add_email_ban) | **POST** /admin/bans/email/add | Add email ban
[**add_ip_ban**](AdminApi.md#add_ip_ban) | **POST** /admin/bans/ip/add | Add IP ban
[**add_phone_ban**](AdminApi.md#add_phone_ban) | **POST** /admin/bans/phone/add | Add phone ban
[**add_snowflake_reservation**](AdminApi.md#add_snowflake_reservation) | **POST** /admin/snowflake-reservations/add | Add snowflake reservation
[**admin_resend_verification_email**](AdminApi.md#admin_resend_verification_email) | **POST** /admin/users/resend-verification-email | Resend verification email
[**approve_discovery_application**](AdminApi.md#approve_discovery_application) | **POST** /admin/discovery/applications/{guild_id}/approve | Approve discovery application
[**approve_system_dm_job**](AdminApi.md#approve_system_dm_job) | **POST** /admin/system-dm-jobs/{job_id}/approve | Approve system DM job
[**ban_guild_member**](AdminApi.md#ban_guild_member) | **POST** /admin/guilds/ban-member | Ban guild member
[**bulk_add_guild_members**](AdminApi.md#bulk_add_guild_members) | **POST** /admin/bulk/add-guild-members | Bulk add guild members
[**bulk_update_guild_features**](AdminApi.md#bulk_update_guild_features) | **POST** /admin/bulk/update-guild-features | Bulk update guild features
[**bulk_update_user_flags**](AdminApi.md#bulk_update_user_flags) | **POST** /admin/bulk/update-user-flags | Bulk update user flags
[**cancel_account_deletion**](AdminApi.md#cancel_account_deletion) | **POST** /admin/users/cancel-deletion | Cancel account deletion
[**cancel_bulk_message_deletion**](AdminApi.md#cancel_bulk_message_deletion) | **POST** /admin/users/cancel-bulk-message-deletion | Cancel bulk message deletion
[**change_user_dob**](AdminApi.md#change_user_dob) | **POST** /admin/users/change-dob | Change user DOB
[**change_user_email**](AdminApi.md#change_user_email) | **POST** /admin/users/change-email | Change user email
[**change_user_username**](AdminApi.md#change_user_username) | **POST** /admin/users/change-username | Change user username
[**check_email_ban_status**](AdminApi.md#check_email_ban_status) | **POST** /admin/bans/email/check | Check email ban status
[**check_ip_ban_status**](AdminApi.md#check_ip_ban_status) | **POST** /admin/bans/ip/check | Check IP ban status
[**check_phone_ban_status**](AdminApi.md#check_phone_ban_status) | **POST** /admin/bans/phone/check | Check phone ban status
[**clear_guild_fields**](AdminApi.md#clear_guild_fields) | **POST** /admin/guilds/clear-fields | Clear guild fields
[**clear_user_fields**](AdminApi.md#clear_user_fields) | **POST** /admin/users/clear-fields | Clear user fields
[**create_admin_api_key**](AdminApi.md#create_admin_api_key) | **POST** /admin/api-keys | Create admin API key
[**create_system_dm_job**](AdminApi.md#create_system_dm_job) | **POST** /admin/system-dm-jobs | Create system DM job
[**create_voice_region**](AdminApi.md#create_voice_region) | **POST** /admin/voice/regions/create | Create voice region
[**create_voice_server**](AdminApi.md#create_voice_server) | **POST** /admin/voice/servers/create | Create voice server
[**delete_admin_api_key**](AdminApi.md#delete_admin_api_key) | **DELETE** /admin/api-keys/{keyId} | Delete admin API key
[**delete_all_user_messages**](AdminApi.md#delete_all_user_messages) | **POST** /admin/messages/delete-all | Delete all user messages
[**delete_guild**](AdminApi.md#delete_guild) | **POST** /admin/guilds/delete | Delete guild
[**delete_message**](AdminApi.md#delete_message) | **POST** /admin/messages/delete | Delete single message
[**delete_snowflake_reservation**](AdminApi.md#delete_snowflake_reservation) | **POST** /admin/snowflake-reservations/delete | Delete snowflake reservation
[**delete_user_webauthn_credential**](AdminApi.md#delete_user_webauthn_credential) | **POST** /admin/users/delete-webauthn-credential | Delete user WebAuthn credential
[**delete_voice_region**](AdminApi.md#delete_voice_region) | **POST** /admin/voice/regions/delete | Delete voice region
[**delete_voice_server**](AdminApi.md#delete_voice_server) | **POST** /admin/voice/servers/delete | Delete voice server
[**disable_user_mfa**](AdminApi.md#disable_user_mfa) | **POST** /admin/users/disable-mfa | Disable user MFA
[**disable_user_suspicious**](AdminApi.md#disable_user_suspicious) | **POST** /admin/users/disable-suspicious | Disable user for suspicious activity
[**expand_visionary_slots**](AdminApi.md#expand_visionary_slots) | **POST** /admin/visionary-slots/expand | Expand visionary slots
[**force_add_user_to_guild**](AdminApi.md#force_add_user_to_guild) | **POST** /admin/guilds/force-add-user | Force add user to guild
[**generate_gift_subscription_codes**](AdminApi.md#generate_gift_subscription_codes) | **POST** /admin/codes/gift | Generate gift subscription codes
[**get_archive_details**](AdminApi.md#get_archive_details) | **GET** /admin/archives/{subjectType}/{subjectId}/{archiveId} | Get archive details
[**get_archive_download_url**](AdminApi.md#get_archive_download_url) | **GET** /admin/archives/{subjectType}/{subjectId}/{archiveId}/download | Get archive download URL
[**get_authenticated_admin_user**](AdminApi.md#get_authenticated_admin_user) | **GET** /admin/users/me | Get authenticated admin user
[**get_gateway_node_statistics**](AdminApi.md#get_gateway_node_statistics) | **GET** /admin/gateway/stats | Get gateway node statistics
[**get_guild_memory_statistics**](AdminApi.md#get_guild_memory_statistics) | **POST** /admin/gateway/memory-stats | Get guild memory statistics
[**get_instance_config**](AdminApi.md#get_instance_config) | **POST** /admin/instance-config/get | Get instance configuration
[**get_legal_hold_status**](AdminApi.md#get_legal_hold_status) | **GET** /admin/reports/{report_id}/legal-hold | Get legal hold status
[**get_limit_config**](AdminApi.md#get_limit_config) | **POST** /admin/limit-config/get | Get limit configuration
[**get_message_shred_status**](AdminApi.md#get_message_shred_status) | **POST** /admin/messages/shred-status | Get message shred status
[**get_ncmec_submission_status**](AdminApi.md#get_ncmec_submission_status) | **GET** /admin/reports/{report_id}/ncmec-status | Get NCMEC submission status
[**get_report**](AdminApi.md#get_report) | **GET** /admin/reports/{report_id} | Get report details
[**get_search_index_refresh_status**](AdminApi.md#get_search_index_refresh_status) | **POST** /admin/search/refresh-status | Get search index refresh status
[**get_user_change_log**](AdminApi.md#get_user_change_log) | **POST** /admin/users/change-log | Get user change log
[**get_voice_region**](AdminApi.md#get_voice_region) | **POST** /admin/voice/regions/get | Get voice region
[**get_voice_server**](AdminApi.md#get_voice_server) | **POST** /admin/voice/servers/get | Get voice server
[**kick_guild_member**](AdminApi.md#kick_guild_member) | **POST** /admin/guilds/kick-member | Kick guild member
[**list_admin_api_keys**](AdminApi.md#list_admin_api_keys) | **GET** /admin/api-keys | List admin API keys
[**list_archives**](AdminApi.md#list_archives) | **POST** /admin/archives/list | List archives
[**list_audit_logs**](AdminApi.md#list_audit_logs) | **POST** /admin/audit-logs | List audit logs
[**list_discovery_applications**](AdminApi.md#list_discovery_applications) | **GET** /admin/discovery/applications | List discovery applications
[**list_email_bans**](AdminApi.md#list_email_bans) | **POST** /admin/bans/email/list | List email bans
[**list_guild_emojis**](AdminApi.md#list_guild_emojis) | **GET** /admin/guilds/{guild_id}/emojis | List guild emojis
[**list_guild_members**](AdminApi.md#list_guild_members) | **POST** /admin/guilds/list-members | List guild members
[**list_guild_stickers**](AdminApi.md#list_guild_stickers) | **GET** /admin/guilds/{guild_id}/stickers | List guild stickers
[**list_ip_bans**](AdminApi.md#list_ip_bans) | **POST** /admin/bans/ip/list | List IP bans
[**list_phone_bans**](AdminApi.md#list_phone_bans) | **POST** /admin/bans/phone/list | List phone bans
[**list_reports**](AdminApi.md#list_reports) | **POST** /admin/reports/list | List reports
[**list_snowflake_reservations**](AdminApi.md#list_snowflake_reservations) | **POST** /admin/snowflake-reservations/list | List snowflake reservations
[**list_system_dm_jobs**](AdminApi.md#list_system_dm_jobs) | **GET** /admin/system-dm-jobs | List system DM jobs
[**list_user_dm_channels**](AdminApi.md#list_user_dm_channels) | **POST** /admin/users/list-dm-channels | List user DM channels
[**list_user_guilds**](AdminApi.md#list_user_guilds) | **POST** /admin/users/list-guilds | List user guilds
[**list_user_sessions**](AdminApi.md#list_user_sessions) | **POST** /admin/users/list-sessions | List user sessions
[**list_user_webauthn_credentials**](AdminApi.md#list_user_webauthn_credentials) | **POST** /admin/users/list-webauthn-credentials | List user WebAuthn credentials
[**list_visionary_slots**](AdminApi.md#list_visionary_slots) | **GET** /admin/visionary-slots | List all visionary slots
[**list_voice_regions**](AdminApi.md#list_voice_regions) | **POST** /admin/voice/regions/list | List voice regions
[**list_voice_servers**](AdminApi.md#list_voice_servers) | **POST** /admin/voice/servers/list | List voice servers
[**lookup_guild**](AdminApi.md#lookup_guild) | **POST** /admin/guilds/lookup | Look up guild
[**lookup_message**](AdminApi.md#lookup_message) | **POST** /admin/messages/lookup | Look up message details
[**lookup_message_by_attachment**](AdminApi.md#lookup_message_by_attachment) | **POST** /admin/messages/lookup-by-attachment | Look up message by attachment
[**lookup_user**](AdminApi.md#lookup_user) | **POST** /admin/users/lookup | Lookup user
[**purge_guild_assets**](AdminApi.md#purge_guild_assets) | **POST** /admin/assets/purge | Purge guild assets
[**queue_message_shred**](AdminApi.md#queue_message_shred) | **POST** /admin/messages/shred | Queue message shred operation
[**refresh_search_index**](AdminApi.md#refresh_search_index) | **POST** /admin/search/refresh-index | Refresh search index
[**reject_discovery_application**](AdminApi.md#reject_discovery_application) | **POST** /admin/discovery/applications/{guild_id}/reject | Reject discovery application
[**release_legal_hold_on_evidence**](AdminApi.md#release_legal_hold_on_evidence) | **DELETE** /admin/reports/{report_id}/legal-hold | Release legal hold on evidence
[**reload_all_specified_guilds**](AdminApi.md#reload_all_specified_guilds) | **POST** /admin/gateway/reload-all | Reload specified guilds
[**reload_guild**](AdminApi.md#reload_guild) | **POST** /admin/guilds/reload | Reload guild
[**remove_email_ban**](AdminApi.md#remove_email_ban) | **POST** /admin/bans/email/remove | Remove email ban
[**remove_from_discovery**](AdminApi.md#remove_from_discovery) | **POST** /admin/discovery/guilds/{guild_id}/remove | Remove guild from discovery
[**remove_ip_ban**](AdminApi.md#remove_ip_ban) | **POST** /admin/bans/ip/remove | Remove IP ban
[**remove_phone_ban**](AdminApi.md#remove_phone_ban) | **POST** /admin/bans/phone/remove | Remove phone ban
[**reserve_visionary_slot**](AdminApi.md#reserve_visionary_slot) | **POST** /admin/visionary-slots/reserve | Reserve or unreserve a visionary slot
[**resolve_report**](AdminApi.md#resolve_report) | **POST** /admin/reports/resolve | Resolve report
[**schedule_account_deletion**](AdminApi.md#schedule_account_deletion) | **POST** /admin/users/schedule-deletion | Schedule account deletion
[**schedule_bulk_user_deletion**](AdminApi.md#schedule_bulk_user_deletion) | **POST** /admin/bulk/schedule-user-deletion | Schedule bulk user deletion
[**search_audit_logs**](AdminApi.md#search_audit_logs) | **POST** /admin/audit-logs/search | Search audit logs
[**search_guilds**](AdminApi.md#search_guilds) | **POST** /admin/guilds/search | Search guilds
[**search_reports**](AdminApi.md#search_reports) | **POST** /admin/reports/search | Search reports
[**search_users**](AdminApi.md#search_users) | **POST** /admin/users/search | Search users
[**send_password_reset**](AdminApi.md#send_password_reset) | **POST** /admin/users/send-password-reset | Send password reset
[**set_legal_hold_on_evidence**](AdminApi.md#set_legal_hold_on_evidence) | **POST** /admin/reports/{report_id}/legal-hold | Set legal hold on evidence
[**set_user_acls**](AdminApi.md#set_user_acls) | **POST** /admin/users/set-acls | Set user ACLs
[**set_user_bot_status**](AdminApi.md#set_user_bot_status) | **POST** /admin/users/set-bot-status | Set user bot status
[**set_user_system_status**](AdminApi.md#set_user_system_status) | **POST** /admin/users/set-system-status | Set user system status
[**set_user_traits**](AdminApi.md#set_user_traits) | **POST** /admin/users/set-traits | Set user traits
[**shrink_visionary_slots**](AdminApi.md#shrink_visionary_slots) | **POST** /admin/visionary-slots/shrink | Shrink visionary slots
[**shutdown_guild**](AdminApi.md#shutdown_guild) | **POST** /admin/guilds/shutdown | Shutdown guild
[**submit_report_to_ncmec**](AdminApi.md#submit_report_to_ncmec) | **POST** /admin/reports/{report_id}/ncmec-submit | Submit report to NCMEC
[**swap_visionary_slots**](AdminApi.md#swap_visionary_slots) | **POST** /admin/visionary-slots/swap | Swap visionary slot reservations
[**temp_ban_user**](AdminApi.md#temp_ban_user) | **POST** /admin/users/temp-ban | Temp ban user
[**terminate_user_sessions**](AdminApi.md#terminate_user_sessions) | **POST** /admin/users/terminate-sessions | Terminate user sessions
[**transfer_guild_ownership**](AdminApi.md#transfer_guild_ownership) | **POST** /admin/guilds/transfer-ownership | Transfer guild ownership
[**trigger_guild_archive**](AdminApi.md#trigger_guild_archive) | **POST** /admin/archives/guild | Trigger guild archive
[**trigger_user_archive**](AdminApi.md#trigger_user_archive) | **POST** /admin/archives/user | Trigger user archive
[**unban_user**](AdminApi.md#unban_user) | **POST** /admin/users/unban | Unban user
[**unlink_user_phone**](AdminApi.md#unlink_user_phone) | **POST** /admin/users/unlink-phone | Unlink user phone
[**update_guild_features**](AdminApi.md#update_guild_features) | **POST** /admin/guilds/update-features | Update guild features
[**update_guild_name**](AdminApi.md#update_guild_name) | **POST** /admin/guilds/update-name | Update guild name
[**update_guild_settings**](AdminApi.md#update_guild_settings) | **POST** /admin/guilds/update-settings | Update guild settings
[**update_guild_vanity**](AdminApi.md#update_guild_vanity) | **POST** /admin/guilds/update-vanity | Update guild vanity
[**update_instance_config**](AdminApi.md#update_instance_config) | **POST** /admin/instance-config/update | Update instance configuration
[**update_limit_config**](AdminApi.md#update_limit_config) | **POST** /admin/limit-config/update | Update limit configuration
[**update_suspicious_activity_flags**](AdminApi.md#update_suspicious_activity_flags) | **POST** /admin/users/update-suspicious-activity-flags | Update suspicious activity flags
[**update_user_flags**](AdminApi.md#update_user_flags) | **POST** /admin/users/update-flags | Update user flags
[**update_voice_region**](AdminApi.md#update_voice_region) | **POST** /admin/voice/regions/update | Update voice region
[**update_voice_server**](AdminApi.md#update_voice_server) | **POST** /admin/voice/servers/update | Update voice server
[**verify_user_email**](AdminApi.md#verify_user_email) | **POST** /admin/users/verify-email | Verify user email



## add_email_ban

> add_email_ban(ban_email_request)
Add email ban

Ban one or more email addresses from registering or creating accounts. Users attempting to use banned emails will be blocked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_email_request** | [**BanEmailRequest**](BanEmailRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_ip_ban

> add_ip_ban(ban_ip_request)
Add IP ban

Ban one or more IP addresses from accessing the platform. Users connecting from banned IPs will be denied service. Can be applied retroactively.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_ip_request** | [**BanIpRequest**](BanIpRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_phone_ban

> add_phone_ban(ban_phone_request)
Add phone ban

Ban one or more phone numbers from account verification or SMS operations. Users attempting to use banned numbers will be blocked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_phone_request** | [**BanPhoneRequest**](BanPhoneRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_snowflake_reservation

> models::SuccessResponse add_snowflake_reservation(add_snowflake_reservation_request)
Add snowflake reservation

Reserves a snowflake ID range for future allocation. Creates audit log entry. Requires INSTANCE_SNOWFLAKE_RESERVATION_MANAGE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_snowflake_reservation_request** | [**AddSnowflakeReservationRequest**](AddSnowflakeReservationRequest.md) |  | [required] |

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_resend_verification_email

> admin_resend_verification_email(resend_verification_email_request)
Resend verification email

Resend the account verification email for a user. Creates audit log entry and honours email verification resend limits. Requires USER_UPDATE_EMAIL permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resend_verification_email_request** | [**ResendVerificationEmailRequest**](ResendVerificationEmailRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## approve_discovery_application

> models::DiscoveryApplicationResponse approve_discovery_application(guild_id, discovery_admin_review_request)
Approve discovery application

Approve a pending discovery application. Requires DISCOVERY_REVIEW permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**discovery_admin_review_request** | [**DiscoveryAdminReviewRequest**](DiscoveryAdminReviewRequest.md) |  | [required] |

### Return type

[**models::DiscoveryApplicationResponse**](DiscoveryApplicationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## approve_system_dm_job

> models::SystemDmJobResponse approve_system_dm_job(job_id)
Approve system DM job

Approves and queues a system DM job for immediate execution. Creates audit log entry. Job will begin sending messages to target users. Requires SYSTEM_DM_SEND permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The job id | [required] |

### Return type

[**models::SystemDmJobResponse**](SystemDmJobResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ban_guild_member

> ban_guild_member(ban_guild_member_request)
Ban guild member

Permanently bans a user from a guild. Prevents user from joining. Logged to audit log. Requires GUILD_BAN_MEMBER permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_guild_member_request** | [**BanGuildMemberRequest**](BanGuildMemberRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_add_guild_members

> models::BulkOperationResponse bulk_add_guild_members(bulk_add_guild_members_request)
Bulk add guild members

Add multiple users to guilds in a batch operation. Bypasses normal invitation flow for administrative account setup.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_add_guild_members_request** | [**BulkAddGuildMembersRequest**](BulkAddGuildMembersRequest.md) |  | [required] |

### Return type

[**models::BulkOperationResponse**](BulkOperationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_update_guild_features

> models::BulkOperationResponse bulk_update_guild_features(bulk_update_guild_features_request)
Bulk update guild features

Modify guild configuration and capabilities across multiple servers in a single operation. Includes feature flags, boost levels, and other guild attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_update_guild_features_request** | [**BulkUpdateGuildFeaturesRequest**](BulkUpdateGuildFeaturesRequest.md) |  | [required] |

### Return type

[**models::BulkOperationResponse**](BulkOperationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_update_user_flags

> models::BulkOperationResponse bulk_update_user_flags(bulk_update_user_flags_request)
Bulk update user flags

Modify user flags (e.g., verified, bot, system) for multiple users in a single operation. Used for mass account updates or corrections.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_update_user_flags_request** | [**BulkUpdateUserFlagsRequest**](BulkUpdateUserFlagsRequest.md) |  | [required] |

### Return type

[**models::BulkOperationResponse**](BulkOperationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_account_deletion

> models::UserMutationResponse cancel_account_deletion(disable_mfa_request)
Cancel account deletion

Cancel a scheduled account deletion. User account restoration prevents data loss. Creates audit log entry. Requires USER_DELETE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**disable_mfa_request** | [**DisableMfaRequest**](DisableMfaRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_bulk_message_deletion

> models::UserMutationResponse cancel_bulk_message_deletion(cancel_bulk_message_deletion_request)
Cancel bulk message deletion

Cancel a scheduled bulk message deletion job for a user. Prevents deletion of user messages across guilds. Creates audit log entry. Requires USER_CANCEL_BULK_MESSAGE_DELETION permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cancel_bulk_message_deletion_request** | [**CancelBulkMessageDeletionRequest**](CancelBulkMessageDeletionRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_user_dob

> models::UserMutationResponse change_user_dob(change_dob_request)
Change user DOB

Update user date of birth. May affect age-restricted content access. Creates audit log entry. Requires USER_UPDATE_DOB permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_dob_request** | [**ChangeDobRequest**](ChangeDobRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_user_email

> models::UserMutationResponse change_user_email(change_email_request)
Change user email

Change user email address. New email must be valid and unique. Marks email as verified. Creates audit log entry. Requires USER_UPDATE_EMAIL permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_email_request** | [**ChangeEmailRequest**](ChangeEmailRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_user_username

> models::UserMutationResponse change_user_username(change_username_request)
Change user username

Change user username. New username must meet requirements and be unique. Creates audit log entry. Requires USER_UPDATE_USERNAME permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_username_request** | [**ChangeUsernameRequest**](ChangeUsernameRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_email_ban_status

> models::BanCheckResponseSchema check_email_ban_status(ban_email_request)
Check email ban status

Query whether one or more email addresses are currently banned from registration. Returns the ban status and metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_email_request** | [**BanEmailRequest**](BanEmailRequest.md) |  | [required] |

### Return type

[**models::BanCheckResponseSchema**](BanCheckResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_ip_ban_status

> models::BanCheckResponseSchema check_ip_ban_status(ban_ip_request)
Check IP ban status

Query whether one or more IP addresses are currently banned. Returns the ban status and any associated metadata like reason or expiration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_ip_request** | [**BanIpRequest**](BanIpRequest.md) |  | [required] |

### Return type

[**models::BanCheckResponseSchema**](BanCheckResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_phone_ban_status

> models::BanCheckResponseSchema check_phone_ban_status(ban_phone_request)
Check phone ban status

Query whether one or more phone numbers are currently banned. Returns the ban status and metadata for verification or appeal purposes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_phone_request** | [**BanPhoneRequest**](BanPhoneRequest.md) |  | [required] |

### Return type

[**models::BanCheckResponseSchema**](BanCheckResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_guild_fields

> clear_guild_fields(clear_guild_fields_request)
Clear guild fields

Clears specified optional guild fields such as icon, banner, or description. Logged to audit log. Requires GUILD_UPDATE_SETTINGS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**clear_guild_fields_request** | [**ClearGuildFieldsRequest**](ClearGuildFieldsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_user_fields

> models::UserMutationResponse clear_user_fields(clear_user_fields_request)
Clear user fields

Clear or reset user profile fields such as bio, avatar, or status. Creates audit log entry. Requires USER_UPDATE_PROFILE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**clear_user_fields_request** | [**ClearUserFieldsRequest**](ClearUserFieldsRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_admin_api_key

> models::CreateAdminApiKeyResponse create_admin_api_key(create_admin_api_key_request)
Create admin API key

Generates a new API key for administrative operations. The key is returned only once at creation time. Includes expiration settings and access control lists (ACLs) to limit the key's permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_admin_api_key_request** | [**CreateAdminApiKeyRequest**](CreateAdminApiKeyRequest.md) |  | [required] |

### Return type

[**models::CreateAdminApiKeyResponse**](CreateAdminApiKeyResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_system_dm_job

> models::SystemDmJobResponse create_system_dm_job(create_system_dm_job_request)
Create system DM job

Creates a system DM broadcast job to send messages to users matching registration date criteria. Supports date range filtering and guild exclusions. Requires SYSTEM_DM_SEND permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_system_dm_job_request** | [**CreateSystemDmJobRequest**](CreateSystemDmJobRequest.md) |  | [required] |

### Return type

[**models::SystemDmJobResponse**](SystemDmJobResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_voice_region

> models::CreateVoiceRegionResponse create_voice_region(create_voice_region_request)
Create voice region

Creates a new voice server region. Defines geographic location and performance characteristics for voice routing. Creates audit log entry. Requires VOICE_REGION_CREATE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_voice_region_request** | [**CreateVoiceRegionRequest**](CreateVoiceRegionRequest.md) |  | [required] |

### Return type

[**models::CreateVoiceRegionResponse**](CreateVoiceRegionResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_voice_server

> models::CreateVoiceServerResponse create_voice_server(create_voice_server_request)
Create voice server

Creates and provisions a new voice server instance in a region. Configures capacity, codecs, and encryption. Creates audit log entry. Requires VOICE_SERVER_CREATE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_voice_server_request** | [**CreateVoiceServerRequest**](CreateVoiceServerRequest.md) |  | [required] |

### Return type

[**models::CreateVoiceServerResponse**](CreateVoiceServerResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_admin_api_key

> models::DeleteApiKeyResponse delete_admin_api_key(key_id)
Delete admin API key

Revokes an API key, immediately invalidating it for all future operations. This action cannot be undone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The keyId | [required] |

### Return type

[**models::DeleteApiKeyResponse**](DeleteApiKeyResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_user_messages

> models::DeleteAllUserMessagesResponse delete_all_user_messages(delete_all_user_messages_request)
Delete all user messages

Deletes all messages from a specific user across all channels. Permanent operation used for account suspension or policy violation. Requires MESSAGE_DELETE_ALL permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_all_user_messages_request** | [**DeleteAllUserMessagesRequest**](DeleteAllUserMessagesRequest.md) |  | [required] |

### Return type

[**models::DeleteAllUserMessagesResponse**](DeleteAllUserMessagesResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild

> models::SuccessResponse delete_guild(delete_guild_request)
Delete guild

Permanently deletes a guild. Deletes all channels, messages, and settings. Irreversible operation. Logged to audit log. Requires GUILD_DELETE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_guild_request** | [**DeleteGuildRequest**](DeleteGuildRequest.md) |  | [required] |

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_message

> models::DeleteMessageResponse delete_message(delete_message_request)
Delete single message

Deletes a single message permanently. Used for removing inappropriate or harmful content. Logged to audit log. Requires MESSAGE_DELETE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_message_request** | [**DeleteMessageRequest**](DeleteMessageRequest.md) |  | [required] |

### Return type

[**models::DeleteMessageResponse**](DeleteMessageResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_snowflake_reservation

> models::SuccessResponse delete_snowflake_reservation(delete_snowflake_reservation_request)
Delete snowflake reservation

Removes a snowflake ID reservation range. Creates audit log entry. Requires INSTANCE_SNOWFLAKE_RESERVATION_MANAGE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_snowflake_reservation_request** | [**DeleteSnowflakeReservationRequest**](DeleteSnowflakeReservationRequest.md) |  | [required] |

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_webauthn_credential

> delete_user_webauthn_credential(delete_web_authn_credential_request)
Delete user WebAuthn credential

Delete a specific WebAuthn credential (passkey/security key) from a user account. Creates audit log entry. Requires USER_UPDATE_MFA permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_web_authn_credential_request** | [**DeleteWebAuthnCredentialRequest**](DeleteWebAuthnCredentialRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_voice_region

> models::DeleteVoiceResponse delete_voice_region(delete_voice_region_request)
Delete voice region

Deletes a voice region. Removes region from routing and reassigns active connections. Creates audit log entry. Requires VOICE_REGION_DELETE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_voice_region_request** | [**DeleteVoiceRegionRequest**](DeleteVoiceRegionRequest.md) |  | [required] |

### Return type

[**models::DeleteVoiceResponse**](DeleteVoiceResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_voice_server

> models::DeleteVoiceResponse delete_voice_server(delete_voice_server_request)
Delete voice server

Decommissions and removes a voice server instance. Disconnects active sessions and migrates to other servers. Creates audit log entry. Requires VOICE_SERVER_DELETE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_voice_server_request** | [**DeleteVoiceServerRequest**](DeleteVoiceServerRequest.md) |  | [required] |

### Return type

[**models::DeleteVoiceResponse**](DeleteVoiceResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_user_mfa

> disable_user_mfa(disable_mfa_request)
Disable user MFA

Disable two-factor authentication for user account. Removes all authenticators. Creates audit log entry. Requires USER_UPDATE_MFA permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**disable_mfa_request** | [**DisableMfaRequest**](DisableMfaRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_user_suspicious

> models::UserMutationResponse disable_user_suspicious(disable_for_suspicious_activity_request)
Disable user for suspicious activity

Disable user account due to suspicious activity or abuse. Account is locked pending review. User cannot access services. Creates audit log entry. Requires USER_DISABLE_SUSPICIOUS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**disable_for_suspicious_activity_request** | [**DisableForSuspiciousActivityRequest**](DisableForSuspiciousActivityRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## expand_visionary_slots

> models::VisionarySlotOperationResponse expand_visionary_slots(expand_visionary_slots_request)
Expand visionary slots

Create additional visionary slots. New slots are added at the next available indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expand_visionary_slots_request** | [**ExpandVisionarySlotsRequest**](ExpandVisionarySlotsRequest.md) |  | [required] |

### Return type

[**models::VisionarySlotOperationResponse**](VisionarySlotOperationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## force_add_user_to_guild

> models::SuccessResponse force_add_user_to_guild(force_add_user_to_guild_request)
Force add user to guild

Forcefully adds a user to a guild. Bypasses normal invite flow for administrative account recovery. Logged to audit log. Requires GUILD_FORCE_ADD_MEMBER permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_add_user_to_guild_request** | [**ForceAddUserToGuildRequest**](ForceAddUserToGuildRequest.md) |  | [required] |

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_gift_subscription_codes

> models::CodesResponse generate_gift_subscription_codes(generate_gift_codes_request)
Generate gift subscription codes

Create redeemable gift codes that grant subscription benefits (e.g. 1 month, 1 year, lifetime). Each code can be used once to activate benefits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generate_gift_codes_request** | [**GenerateGiftCodesRequest**](GenerateGiftCodesRequest.md) |  | [required] |

### Return type

[**models::CodesResponse**](CodesResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_archive_details

> models::GetArchiveResponseSchema get_archive_details(subject_type, subject_id, archive_id)
Get archive details

Retrieve metadata for a specific archive including its status, creation time, expiration, and file location. Does not return the archive contents themselves.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_type** | **String** | The subjectType | [required] |
**subject_id** | **String** | The subjectId | [required] |
**archive_id** | **String** | The archiveId | [required] |

### Return type

[**models::GetArchiveResponseSchema**](GetArchiveResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_archive_download_url

> models::DownloadUrlResponseSchema get_archive_download_url(subject_type, subject_id, archive_id)
Get archive download URL

Generate a time-limited download link to the archive file. The URL provides direct access to download the compressed archive contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_type** | **String** | The subjectType | [required] |
**subject_id** | **String** | The subjectId | [required] |
**archive_id** | **String** | The archiveId | [required] |

### Return type

[**models::DownloadUrlResponseSchema**](DownloadUrlResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authenticated_admin_user

> models::AdminUsersMeResponse get_authenticated_admin_user()
Get authenticated admin user

Get profile of currently authenticated admin user. Returns admin permissions, roles, and metadata. Requires AUTHENTICATE permission.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AdminUsersMeResponse**](AdminUsersMeResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gateway_node_statistics

> models::NodeStatsResponse get_gateway_node_statistics()
Get gateway node statistics

Returns uptime, process memory, and guild count. Used to monitor gateway health and performance. Requires GATEWAY_MEMORY_STATS permission.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NodeStatsResponse**](NodeStatsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_memory_statistics

> models::GuildMemoryStatsResponse get_guild_memory_statistics(get_process_memory_stats_request)
Get guild memory statistics

Returns heap and resident memory usage per guild. Requires GATEWAY_MEMORY_STATS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_process_memory_stats_request** | [**GetProcessMemoryStatsRequest**](GetProcessMemoryStatsRequest.md) |  | [required] |

### Return type

[**models::GuildMemoryStatsResponse**](GuildMemoryStatsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_config

> models::InstanceConfigResponse get_instance_config()
Get instance configuration

Retrieves instance-wide configuration including manual review settings, webhooks, and SSO configuration. Returns current state and schedule information. Requires INSTANCE_CONFIG_VIEW permission.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::InstanceConfigResponse**](InstanceConfigResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_legal_hold_status

> models::LegalHoldResponse get_legal_hold_status(report_id)
Get legal hold status

Retrieve the current legal hold status of a report. Indicates whether evidence is preserved for legal proceedings and the hold expiration date if set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | The report id | [required] |

### Return type

[**models::LegalHoldResponse**](LegalHoldResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_limit_config

> models::LimitConfigGetResponse get_limit_config()
Get limit configuration

Retrieves rate limit configuration including message limits, upload limits, and request throttles. Shows defaults, metadata, and any modifications from defaults. Requires INSTANCE_LIMIT_CONFIG_VIEW permission.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LimitConfigGetResponse**](LimitConfigGetResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message_shred_status

> models::MessageShredStatusResponse get_message_shred_status(message_shred_status_request)
Get message shred status

Polls status of a queued message shred operation. Returns progress percentage and whether the job is complete. Requires MESSAGE_SHRED permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_shred_status_request** | [**MessageShredStatusRequest**](MessageShredStatusRequest.md) |  | [required] |

### Return type

[**models::MessageShredStatusResponse**](MessageShredStatusResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ncmec_submission_status

> models::NcmecSubmissionStatusResponse get_ncmec_submission_status(report_id)
Get NCMEC submission status

Retrieve the submission status of a report to the National Center for Missing & Exploited Children. Shows whether the report has been submitted and the current status with NCMEC.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | The report id | [required] |

### Return type

[**models::NcmecSubmissionStatusResponse**](NcmecSubmissionStatusResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_report

> models::ReportAdminResponseSchema get_report(report_id)
Get report details

Retrieves detailed information about a specific report including content, reporter, and reason. Requires REPORT_VIEW permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | The report id | [required] |

### Return type

[**models::ReportAdminResponseSchema**](ReportAdminResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_index_refresh_status

> models::IndexRefreshStatusResponse get_search_index_refresh_status(get_index_refresh_status_request)
Get search index refresh status

Polls status of a search index refresh job. Returns completion percentage and current phase. Requires GUILD_LOOKUP permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_index_refresh_status_request** | [**GetIndexRefreshStatusRequest**](GetIndexRefreshStatusRequest.md) |  | [required] |

### Return type

[**models::IndexRefreshStatusResponse**](IndexRefreshStatusResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_change_log

> models::ListUserChangeLogResponseSchema get_user_change_log(list_user_change_log_request)
Get user change log

Retrieve complete change log history for a user. Shows all profile modifications, admin actions, and account changes with timestamps. Requires USER_LOOKUP permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_user_change_log_request** | [**ListUserChangeLogRequest**](ListUserChangeLogRequest.md) |  | [required] |

### Return type

[**models::ListUserChangeLogResponseSchema**](ListUserChangeLogResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_region

> models::GetVoiceRegionResponse get_voice_region(get_voice_region_request)
Get voice region

Gets detailed information about a voice region including assigned servers and capacity. Shows performance metrics and server details. Requires VOICE_REGION_LIST permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_voice_region_request** | [**GetVoiceRegionRequest**](GetVoiceRegionRequest.md) |  | [required] |

### Return type

[**models::GetVoiceRegionResponse**](GetVoiceRegionResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_server

> models::GetVoiceServerResponse get_voice_server(get_voice_server_request)
Get voice server

Gets detailed voice server information including active connections, configuration, and performance metrics. Requires VOICE_SERVER_LIST permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_voice_server_request** | [**GetVoiceServerRequest**](GetVoiceServerRequest.md) |  | [required] |

### Return type

[**models::GetVoiceServerResponse**](GetVoiceServerResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kick_guild_member

> kick_guild_member(kick_guild_member_request)
Kick guild member

Temporarily removes a user from a guild. User can rejoin. Logged to audit log. Requires GUILD_KICK_MEMBER permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kick_guild_member_request** | [**KickGuildMemberRequest**](KickGuildMemberRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_admin_api_keys

> Vec<models::ListAdminApiKeyResponse> list_admin_api_keys()
List admin API keys

Retrieve all API keys created by the authenticated admin. Returns metadata including creation time, last used time, and assigned permissions. The actual key material is not returned.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ListAdminApiKeyResponse>**](ListAdminApiKeyResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_archives

> models::ListArchivesResponseSchema list_archives(list_archives_request)
List archives

Query and filter created archives by type (user or guild), subject ID, requestor, and expiration status. Admins with limited ACLs see only archives matching their permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_archives_request** | [**ListArchivesRequest**](ListArchivesRequest.md) |  | [required] |

### Return type

[**models::ListArchivesResponseSchema**](ListArchivesResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_audit_logs

> models::AuditLogsListResponseSchema list_audit_logs(list_audit_logs_request)
List audit logs

Retrieve a paginated list of audit logs with optional filtering by date range, action type, or actor. Used for tracking administrative operations and compliance auditing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_audit_logs_request** | [**ListAuditLogsRequest**](ListAuditLogsRequest.md) |  | [required] |

### Return type

[**models::AuditLogsListResponseSchema**](AuditLogsListResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_discovery_applications

> Vec<models::DiscoveryApplicationResponse> list_discovery_applications(status, limit, cursor)
List discovery applications

List discovery applications filtered by status. Requires DISCOVERY_REVIEW permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**cursor** | Option<**String**> |  |  |

### Return type

[**Vec<models::DiscoveryApplicationResponse>**](DiscoveryApplicationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_email_bans

> models::ListEmailBansResponseSchema list_email_bans(list_bans_request)
List email bans

List currently banned email addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_bans_request** | [**ListBansRequest**](ListBansRequest.md) |  | [required] |

### Return type

[**models::ListEmailBansResponseSchema**](ListEmailBansResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_emojis

> models::ListGuildEmojisResponse list_guild_emojis(guild_id)
List guild emojis

Lists all custom emojis in a guild. Returns ID, name, and creation date. Used for asset inventory and purge operations. Requires ASSET_PURGE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**models::ListGuildEmojisResponse**](ListGuildEmojisResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_members

> models::ListGuildMembersResponse list_guild_members(list_guild_members_request)
List guild members

Lists all guild members with pagination. Returns member IDs, join dates, and roles. Requires GUILD_LIST_MEMBERS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_guild_members_request** | [**ListGuildMembersRequest**](ListGuildMembersRequest.md) |  | [required] |

### Return type

[**models::ListGuildMembersResponse**](ListGuildMembersResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_stickers

> models::ListGuildStickersResponse list_guild_stickers(guild_id)
List guild stickers

Lists all stickers in a guild. Returns ID, name, and asset information. Used for asset inventory and purge operations. Requires ASSET_PURGE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**models::ListGuildStickersResponse**](ListGuildStickersResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ip_bans

> models::ListIpBansResponseSchema list_ip_bans(list_bans_request)
List IP bans

List currently banned IPs/CIDR ranges. Includes reverse DNS where available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_bans_request** | [**ListBansRequest**](ListBansRequest.md) |  | [required] |

### Return type

[**models::ListIpBansResponseSchema**](ListIpBansResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_phone_bans

> models::ListPhoneBansResponseSchema list_phone_bans(list_bans_request)
List phone bans

List currently banned phone numbers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_bans_request** | [**ListBansRequest**](ListBansRequest.md) |  | [required] |

### Return type

[**models::ListPhoneBansResponseSchema**](ListPhoneBansResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_reports

> models::ListReportsResponse list_reports(list_reports_request)
List reports

Lists user and content reports with optional status filtering and pagination. Requires REPORT_VIEW permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_reports_request** | [**ListReportsRequest**](ListReportsRequest.md) |  | [required] |

### Return type

[**models::ListReportsResponse**](ListReportsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_snowflake_reservations

> models::ListSnowflakeReservationsResponse list_snowflake_reservations()
List snowflake reservations

Lists all reserved snowflake ID ranges. Shows ranges reserved for future entities and their allocation status. Requires INSTANCE_SNOWFLAKE_RESERVATION_VIEW permission.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListSnowflakeReservationsResponse**](ListSnowflakeReservationsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_system_dm_jobs

> models::ListSystemDmJobsResponse list_system_dm_jobs(limit, before_job_id)
List system DM jobs

Lists system DM broadcast jobs with pagination. Shows job status, creation time, and content preview. Requires SYSTEM_DM_SEND permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**String**> |  |  |
**before_job_id** | Option<**String**> |  |  |

### Return type

[**models::ListSystemDmJobsResponse**](ListSystemDmJobsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_dm_channels

> models::ListUserDmChannelsResponse list_user_dm_channels(list_user_dm_channels_request)
List user DM channels

List historical one-to-one DM channels for a user with cursor pagination. Requires USER_LIST_DM_CHANNELS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_user_dm_channels_request** | [**ListUserDmChannelsRequest**](ListUserDmChannelsRequest.md) |  | [required] |

### Return type

[**models::ListUserDmChannelsResponse**](ListUserDmChannelsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_guilds

> models::ListUserGuildsResponse list_user_guilds(list_user_guilds_request)
List user guilds

List all guilds a user is a member of. Shows roles and join dates. Requires USER_LIST_GUILDS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_user_guilds_request** | [**ListUserGuildsRequest**](ListUserGuildsRequest.md) |  | [required] |

### Return type

[**models::ListUserGuildsResponse**](ListUserGuildsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_sessions

> models::ListUserSessionsResponse list_user_sessions(list_user_sessions_request)
List user sessions

List all active user sessions across devices. Shows device info, IP, last activity, and creation time. Requires USER_LIST_SESSIONS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_user_sessions_request** | [**ListUserSessionsRequest**](ListUserSessionsRequest.md) |  | [required] |

### Return type

[**models::ListUserSessionsResponse**](ListUserSessionsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_webauthn_credentials

> Vec<models::WebAuthnCredentialResponse> list_user_webauthn_credentials(list_web_authn_credentials_request)
List user WebAuthn credentials

List all WebAuthn credentials (passkeys/security keys) registered for a user. Returns credential names, creation dates, and last usage. Creates audit log entry. Requires USER_UPDATE_MFA permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_web_authn_credentials_request** | [**ListWebAuthnCredentialsRequest**](ListWebAuthnCredentialsRequest.md) |  | [required] |

### Return type

[**Vec<models::WebAuthnCredentialResponse>**](WebAuthnCredentialResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_visionary_slots

> models::ListVisionarySlotsResponse list_visionary_slots()
List all visionary slots

Retrieve the complete list of visionary slots with their reservation status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListVisionarySlotsResponse**](ListVisionarySlotsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_voice_regions

> models::ListVoiceRegionsResponse list_voice_regions(list_voice_regions_request)
List voice regions

Lists all configured voice server regions with status and server count. Shows region names, latency info, and availability. Requires VOICE_REGION_LIST permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_voice_regions_request** | [**ListVoiceRegionsRequest**](ListVoiceRegionsRequest.md) |  | [required] |

### Return type

[**models::ListVoiceRegionsResponse**](ListVoiceRegionsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_voice_servers

> models::ListVoiceServersResponse list_voice_servers(list_voice_servers_request)
List voice servers

Lists all voice servers with connection counts and capacity. Shows server status, region assignment, and load metrics. Supports filtering and pagination. Requires VOICE_SERVER_LIST permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_voice_servers_request** | [**ListVoiceServersRequest**](ListVoiceServersRequest.md) |  | [required] |

### Return type

[**models::ListVoiceServersResponse**](ListVoiceServersResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_guild

> models::LookupGuildResponse lookup_guild(lookup_guild_request)
Look up guild

Retrieves complete guild details including metadata, settings, and statistics. Look up by guild ID or vanity slug. Requires GUILD_LOOKUP permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookup_guild_request** | [**LookupGuildRequest**](LookupGuildRequest.md) |  | [required] |

### Return type

[**models::LookupGuildResponse**](LookupGuildResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_message

> models::LookupMessageResponse lookup_message(lookup_message_request)
Look up message details

Retrieves complete message details including content, attachments, edits, and metadata. Look up by message ID and channel. Requires MESSAGE_LOOKUP permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookup_message_request** | [**LookupMessageRequest**](LookupMessageRequest.md) |  | [required] |

### Return type

[**models::LookupMessageResponse**](LookupMessageResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_message_by_attachment

> models::LookupMessageResponse lookup_message_by_attachment(lookup_message_by_attachment_request)
Look up message by attachment

Finds and retrieves message containing a specific attachment by ID. Used to locate messages with sensitive or illegal content. Requires MESSAGE_LOOKUP permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookup_message_by_attachment_request** | [**LookupMessageByAttachmentRequest**](LookupMessageByAttachmentRequest.md) |  | [required] |

### Return type

[**models::LookupMessageResponse**](LookupMessageResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_user

> models::LookupUserResponse lookup_user(lookup_user_request)
Lookup user

Look up detailed user profile by ID, username, email, or phone. Returns account status, permissions, and metadata. Requires USER_LOOKUP permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookup_user_request** | [**LookupUserRequest**](LookupUserRequest.md) |  | [required] |

### Return type

[**models::LookupUserResponse**](LookupUserResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purge_guild_assets

> models::PurgeGuildAssetsResponseSchema purge_guild_assets(purge_guild_assets_request)
Purge guild assets

Delete and clean up all assets belonging to a guild, including icons, banners, and other media. This is a destructive operation used for cleanup during guild management or compliance actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purge_guild_assets_request** | [**PurgeGuildAssetsRequest**](PurgeGuildAssetsRequest.md) |  | [required] |

### Return type

[**models::PurgeGuildAssetsResponseSchema**](PurgeGuildAssetsResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## queue_message_shred

> models::MessageShredResponse queue_message_shred(message_shred_request)
Queue message shred operation

Queues bulk message shredding with attachment deletion. Returns job ID to track progress asynchronously. Used for large-scale content removal. Requires MESSAGE_SHRED permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_shred_request** | [**MessageShredRequest**](MessageShredRequest.md) |  | [required] |

### Return type

[**models::MessageShredResponse**](MessageShredResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_search_index

> models::RefreshSearchIndexResponse refresh_search_index(refresh_search_index_request)
Refresh search index

Trigger full or partial search index rebuild. Creates background job to reindex guilds and users. Returns job ID for status tracking. Requires GUILD_LOOKUP permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_search_index_request** | [**RefreshSearchIndexRequest**](RefreshSearchIndexRequest.md) |  | [required] |

### Return type

[**models::RefreshSearchIndexResponse**](RefreshSearchIndexResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reject_discovery_application

> models::DiscoveryApplicationResponse reject_discovery_application(guild_id, discovery_admin_reject_request)
Reject discovery application

Reject a pending discovery application. Requires DISCOVERY_REVIEW permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**discovery_admin_reject_request** | [**DiscoveryAdminRejectRequest**](DiscoveryAdminRejectRequest.md) |  | [required] |

### Return type

[**models::DiscoveryApplicationResponse**](DiscoveryApplicationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## release_legal_hold_on_evidence

> models::LegalHoldResponse release_legal_hold_on_evidence(report_id)
Release legal hold on evidence

Remove a legal hold on a report. Evidence becomes eligible for automatic deletion per the retention policy. Used after legal matters are resolved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | The report id | [required] |

### Return type

[**models::LegalHoldResponse**](LegalHoldResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reload_all_specified_guilds

> models::ReloadAllGuildsResponse reload_all_specified_guilds(reload_guilds_request)
Reload specified guilds

Reconnects to the database and re-syncs guild state. Used for recovery after data inconsistencies. Requires GATEWAY_RELOAD_ALL permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reload_guilds_request** | [**ReloadGuildsRequest**](ReloadGuildsRequest.md) |  | [required] |

### Return type

[**models::ReloadAllGuildsResponse**](ReloadAllGuildsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reload_guild

> models::SuccessResponse reload_guild(reload_guild_request)
Reload guild

Reloads a single guild state from database. Used to recover from corruption or sync issues. Logged to audit log. Requires GUILD_RELOAD permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reload_guild_request** | [**ReloadGuildRequest**](ReloadGuildRequest.md) |  | [required] |

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_email_ban

> remove_email_ban(ban_email_request)
Remove email ban

Lift a previously applied email ban, allowing the address to be used for new registrations. Used for appeals or error correction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_email_request** | [**BanEmailRequest**](BanEmailRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_from_discovery

> models::DiscoveryApplicationResponse remove_from_discovery(guild_id, discovery_admin_remove_request)
Remove guild from discovery

Remove an approved guild from discovery. Requires DISCOVERY_REMOVE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**discovery_admin_remove_request** | [**DiscoveryAdminRemoveRequest**](DiscoveryAdminRemoveRequest.md) |  | [required] |

### Return type

[**models::DiscoveryApplicationResponse**](DiscoveryApplicationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_ip_ban

> remove_ip_ban(ban_ip_request)
Remove IP ban

Lift a previously applied IP ban, allowing traffic from those addresses again. Used for appeals or when bans were applied in error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_ip_request** | [**BanIpRequest**](BanIpRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_phone_ban

> remove_phone_ban(ban_phone_request)
Remove phone ban

Lift a previously applied phone ban, allowing the number to be used for verification again. Used for appeals or error correction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_phone_request** | [**BanPhoneRequest**](BanPhoneRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reserve_visionary_slot

> models::VisionarySlotOperationResponse reserve_visionary_slot(reserve_visionary_slot_request)
Reserve or unreserve a visionary slot

Reserve a specific slot index for a user ID, or unreserve it by setting user_id to null. Special value -1 is also valid for user_id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserve_visionary_slot_request** | [**ReserveVisionarySlotRequest**](ReserveVisionarySlotRequest.md) |  | [required] |

### Return type

[**models::VisionarySlotOperationResponse**](VisionarySlotOperationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resolve_report

> models::ResolveReportResponse resolve_report(resolve_report_request)
Resolve report

Closes and resolves a report with optional public comment. Marks report as handled and creates audit log entry. Requires REPORT_RESOLVE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resolve_report_request** | [**ResolveReportRequest**](ResolveReportRequest.md) |  | [required] |

### Return type

[**models::ResolveReportResponse**](ResolveReportResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_account_deletion

> models::UserMutationResponse schedule_account_deletion(schedule_account_deletion_request)
Schedule account deletion

Schedule user account for deletion after grace period. Account will be fully deleted with all content unless cancellation is executed. Creates audit log entry. Requires USER_DELETE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_account_deletion_request** | [**ScheduleAccountDeletionRequest**](ScheduleAccountDeletionRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_bulk_user_deletion

> models::BulkOperationResponse schedule_bulk_user_deletion(bulk_schedule_user_deletion_request)
Schedule bulk user deletion

Queue multiple users for deactivation/deletion with an optional grace period. Deletions are processed asynchronously according to retention policies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_schedule_user_deletion_request** | [**BulkScheduleUserDeletionRequest**](BulkScheduleUserDeletionRequest.md) |  | [required] |

### Return type

[**models::BulkOperationResponse**](BulkOperationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_audit_logs

> models::AuditLogsListResponseSchema search_audit_logs(search_audit_logs_request)
Search audit logs

Perform a full-text search across audit logs for specific events or changes. Allows targeted queries for compliance investigations or incident response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_audit_logs_request** | [**SearchAuditLogsRequest**](SearchAuditLogsRequest.md) |  | [required] |

### Return type

[**models::AuditLogsListResponseSchema**](AuditLogsListResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_guilds

> models::SearchGuildsResponse search_guilds(search_guilds_request)
Search guilds

Searches guilds by name, ID, and other criteria. Supports full-text search and filtering. Requires GUILD_LOOKUP permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_guilds_request** | [**SearchGuildsRequest**](SearchGuildsRequest.md) |  | [required] |

### Return type

[**models::SearchGuildsResponse**](SearchGuildsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_reports

> models::SearchReportsResponse search_reports(search_reports_request)
Search reports

Searches and filters reports by user, content, reason, and status criteria. Supports full-text search and advanced filtering. Requires REPORT_VIEW permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_reports_request** | [**SearchReportsRequest**](SearchReportsRequest.md) |  | [required] |

### Return type

[**models::SearchReportsResponse**](SearchReportsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users

> models::SearchUsersResponse search_users(search_users_request)
Search users

Searches users by username, email, ID, and other criteria. Supports full-text search and filtering by account status. Requires USER_LOOKUP permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_users_request** | [**SearchUsersRequest**](SearchUsersRequest.md) |  | [required] |

### Return type

[**models::SearchUsersResponse**](SearchUsersResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_password_reset

> send_password_reset(send_password_reset_request)
Send password reset

Send password reset email to user with reset link. User must use link within expiry window. Creates audit log entry. Requires USER_UPDATE_EMAIL permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_password_reset_request** | [**SendPasswordResetRequest**](SendPasswordResetRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_legal_hold_on_evidence

> models::LegalHoldResponse set_legal_hold_on_evidence(report_id, legal_hold_request)
Set legal hold on evidence

Place a legal hold on report evidence to prevent automatic deletion. Used for compliance with legal investigations or regulatory requirements. Optionally specify an expiration date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | The report id | [required] |
**legal_hold_request** | [**LegalHoldRequest**](LegalHoldRequest.md) |  | [required] |

### Return type

[**models::LegalHoldResponse**](LegalHoldResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_acls

> models::UserMutationResponse set_user_acls(set_user_acls_request)
Set user ACLs

Grant or revoke admin ACL permissions to user. Controls admin capabilities and panel access. Creates audit log entry. Requires ACL_SET_USER permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_user_acls_request** | [**SetUserAclsRequest**](SetUserAclsRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_bot_status

> models::UserMutationResponse set_user_bot_status(set_user_bot_status_request)
Set user bot status

Mark or unmark a user account as a bot. Controls bot badge visibility and API permissions. Creates audit log entry. Requires USER_UPDATE_BOT_STATUS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_user_bot_status_request** | [**SetUserBotStatusRequest**](SetUserBotStatusRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_system_status

> models::UserMutationResponse set_user_system_status(set_user_system_status_request)
Set user system status

Mark or unmark a user as a system account. System accounts have special permissions for automated operations. Creates audit log entry. Requires USER_UPDATE_BOT_STATUS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_user_system_status_request** | [**SetUserSystemStatusRequest**](SetUserSystemStatusRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_traits

> models::UserMutationResponse set_user_traits(set_user_traits_request)
Set user traits

Set or update user trait attributes and profile metadata. Traits customize user display and features. Creates audit log entry. Requires USER_UPDATE_TRAITS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_user_traits_request** | [**SetUserTraitsRequest**](SetUserTraitsRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shrink_visionary_slots

> models::VisionarySlotOperationResponse shrink_visionary_slots(shrink_visionary_slots_request)
Shrink visionary slots

Reduce the total number of visionary slots. Only unreserved slots from the highest indices can be removed. Fails if reserved slots would be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shrink_visionary_slots_request** | [**ShrinkVisionarySlotsRequest**](ShrinkVisionarySlotsRequest.md) |  | [required] |

### Return type

[**models::VisionarySlotOperationResponse**](VisionarySlotOperationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shutdown_guild

> models::SuccessResponse shutdown_guild(shutdown_guild_request)
Shutdown guild

Shuts down and unloads a guild from the gateway. Guild data remains in database. Used for emergency resource cleanup. Logged to audit log. Requires GUILD_SHUTDOWN permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shutdown_guild_request** | [**ShutdownGuildRequest**](ShutdownGuildRequest.md) |  | [required] |

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_report_to_ncmec

> models::NcmecSubmitResultResponse submit_report_to_ncmec(report_id)
Submit report to NCMEC

Manually submit a child safety report to the National Center for Missing & Exploited Children. Requires explicit authorization and includes evidence packaging. Can only be done once per report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | The report id | [required] |

### Return type

[**models::NcmecSubmitResultResponse**](NcmecSubmitResultResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap_visionary_slots

> models::VisionarySlotOperationResponse swap_visionary_slots(swap_visionary_slots_request)
Swap visionary slot reservations

Swap the reserved user IDs between two slot indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**swap_visionary_slots_request** | [**SwapVisionarySlotsRequest**](SwapVisionarySlotsRequest.md) |  | [required] |

### Return type

[**models::VisionarySlotOperationResponse**](VisionarySlotOperationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## temp_ban_user

> models::UserMutationResponse temp_ban_user(temp_ban_user_request)
Temp ban user

Apply temporary ban to user account for specified duration. Prevents login and guild operations. Automatically lifts after expiry. Creates audit log entry. Requires USER_TEMP_BAN permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**temp_ban_user_request** | [**TempBanUserRequest**](TempBanUserRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## terminate_user_sessions

> models::TerminateSessionsResponse terminate_user_sessions(terminate_sessions_request)
Terminate user sessions

Terminate all active user sessions across devices. Forces user to re-authenticate on next connection. Creates audit log entry. Requires USER_UPDATE_FLAGS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terminate_sessions_request** | [**TerminateSessionsRequest**](TerminateSessionsRequest.md) |  | [required] |

### Return type

[**models::TerminateSessionsResponse**](TerminateSessionsResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_guild_ownership

> models::GuildUpdateResponse transfer_guild_ownership(transfer_guild_ownership_request)
Transfer guild ownership

Transfers guild ownership to another user. Used when owner is inactive or for administrative recovery. Logged to audit log. Requires GUILD_TRANSFER_OWNERSHIP permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_guild_ownership_request** | [**TransferGuildOwnershipRequest**](TransferGuildOwnershipRequest.md) |  | [required] |

### Return type

[**models::GuildUpdateResponse**](GuildUpdateResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_guild_archive

> models::AdminArchiveResponseSchema trigger_guild_archive(trigger_guild_archive_request)
Trigger guild archive

Initiates a data export for a guild (server). Creates an archive containing all guild data including channels, messages, members, roles, and settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trigger_guild_archive_request** | [**TriggerGuildArchiveRequest**](TriggerGuildArchiveRequest.md) |  | [required] |

### Return type

[**models::AdminArchiveResponseSchema**](AdminArchiveResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_user_archive

> models::AdminArchiveResponseSchema trigger_user_archive(trigger_user_archive_request)
Trigger user archive

Initiates a data export for a user. Creates an archive containing all the user's data (messages, server memberships, preferences, etc.) for export or compliance purposes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trigger_user_archive_request** | [**TriggerUserArchiveRequest**](TriggerUserArchiveRequest.md) |  | [required] |

### Return type

[**models::AdminArchiveResponseSchema**](AdminArchiveResponseSchema.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unban_user

> models::UserMutationResponse unban_user(disable_mfa_request)
Unban user

Immediately remove temporary ban from user account. User can log in and access guilds again. Creates audit log entry. Requires USER_TEMP_BAN permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**disable_mfa_request** | [**DisableMfaRequest**](DisableMfaRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_user_phone

> models::UserMutationResponse unlink_user_phone(unlink_phone_request)
Unlink user phone

Remove phone number from user account. Unlinks any two-factor authentication that depends on phone. Creates audit log entry. Requires USER_UPDATE_PHONE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_phone_request** | [**UnlinkPhoneRequest**](UnlinkPhoneRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_features

> models::GuildUpdateResponse update_guild_features(update_guild_features_request)
Update guild features

Enables or disables guild feature flags. Modifies verification levels and community settings. Changes are logged to audit log. Requires GUILD_UPDATE_FEATURES permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_guild_features_request** | [**UpdateGuildFeaturesRequest**](UpdateGuildFeaturesRequest.md) |  | [required] |

### Return type

[**models::GuildUpdateResponse**](GuildUpdateResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_name

> models::GuildUpdateResponse update_guild_name(update_guild_name_request)
Update guild name

Changes a guild name. Used for removing inappropriate names or correcting display issues. Logged to audit log. Requires GUILD_UPDATE_NAME permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_guild_name_request** | [**UpdateGuildNameRequest**](UpdateGuildNameRequest.md) |  | [required] |

### Return type

[**models::GuildUpdateResponse**](GuildUpdateResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_settings

> models::GuildUpdateResponse update_guild_settings(update_guild_settings_request)
Update guild settings

Modifies guild configuration including description, region, language and other settings. Logged to audit log. Requires GUILD_UPDATE_SETTINGS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_guild_settings_request** | [**UpdateGuildSettingsRequest**](UpdateGuildSettingsRequest.md) |  | [required] |

### Return type

[**models::GuildUpdateResponse**](GuildUpdateResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_vanity

> models::GuildUpdateResponse update_guild_vanity(update_guild_vanity_request)
Update guild vanity

Updates a guild vanity URL slug. Sets custom short URL and prevents duplicate slugs. Logged to audit log. Requires GUILD_UPDATE_VANITY permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_guild_vanity_request** | [**UpdateGuildVanityRequest**](UpdateGuildVanityRequest.md) |  | [required] |

### Return type

[**models::GuildUpdateResponse**](GuildUpdateResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance_config

> models::InstanceConfigResponse update_instance_config(instance_config_update_request)
Update instance configuration

Updates instance configuration settings including manual review mode, webhook URLs, and SSO parameters. Changes apply immediately. Requires INSTANCE_CONFIG_UPDATE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_config_update_request** | [**InstanceConfigUpdateRequest**](InstanceConfigUpdateRequest.md) |  | [required] |

### Return type

[**models::InstanceConfigResponse**](InstanceConfigResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_limit_config

> models::LimitConfigGetResponse update_limit_config(limit_config_update_request)
Update limit configuration

Updates rate limit configuration including message throughput, upload sizes, and request throttles. Changes apply immediately to all new operations. Requires INSTANCE_LIMIT_CONFIG_UPDATE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit_config_update_request** | [**LimitConfigUpdateRequest**](LimitConfigUpdateRequest.md) |  | [required] |

### Return type

[**models::LimitConfigGetResponse**](LimitConfigGetResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_suspicious_activity_flags

> models::UserMutationResponse update_suspicious_activity_flags(update_suspicious_activity_flags_request)
Update suspicious activity flags

Flag user as suspicious for account abuse, fraud, or policy violations. Enables enforcement actions and rate limiting. Creates audit log entry. Requires USER_UPDATE_SUSPICIOUS_ACTIVITY permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_suspicious_activity_flags_request** | [**UpdateSuspiciousActivityFlagsRequest**](UpdateSuspiciousActivityFlagsRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_flags

> models::UserMutationResponse update_user_flags(update_user_flags_request)
Update user flags

Add or remove user flags to control account features and restrictions. Flags determine verification status and special properties. Creates audit log entry. Requires USER_UPDATE_FLAGS permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_flags_request** | [**UpdateUserFlagsRequest**](UpdateUserFlagsRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_voice_region

> models::UpdateVoiceRegionResponse update_voice_region(update_voice_region_request)
Update voice region

Updates voice region settings such as latency thresholds or priority. Changes affect voice routing for new sessions. Creates audit log entry. Requires VOICE_REGION_UPDATE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_voice_region_request** | [**UpdateVoiceRegionRequest**](UpdateVoiceRegionRequest.md) |  | [required] |

### Return type

[**models::UpdateVoiceRegionResponse**](UpdateVoiceRegionResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_voice_server

> models::UpdateVoiceServerResponse update_voice_server(update_voice_server_request)
Update voice server

Updates voice server configuration including capacity, region assignment, and quality settings. Changes apply to new connections. Creates audit log entry. Requires VOICE_SERVER_UPDATE permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_voice_server_request** | [**UpdateVoiceServerRequest**](UpdateVoiceServerRequest.md) |  | [required] |

### Return type

[**models::UpdateVoiceServerResponse**](UpdateVoiceServerResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_user_email

> models::UserMutationResponse verify_user_email(verify_user_email_request)
Verify user email

Manually verify user email address without requiring confirmation link. Bypasses email verification requirement. Creates audit log entry. Requires USER_UPDATE_EMAIL permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_user_email_request** | [**VerifyUserEmailRequest**](VerifyUserEmailRequest.md) |  | [required] |

### Return type

[**models::UserMutationResponse**](UserMutationResponse.md)

### Authorization

[adminApiKey](../README.md#adminApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

