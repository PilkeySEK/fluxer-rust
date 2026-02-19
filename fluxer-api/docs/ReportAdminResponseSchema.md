# ReportAdminResponseSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**report_id** | **String** |  | 
**reporter_id** | Option<**String**> |  | 
**reporter_tag** | Option<**String**> |  | 
**reporter_username** | Option<**String**> |  | 
**reporter_discriminator** | Option<**String**> |  | 
**reporter_email** | Option<**String**> |  | 
**reporter_full_legal_name** | Option<**String**> |  | 
**reporter_country_of_residence** | Option<**String**> |  | 
**reported_at** | **String** |  | 
**status** | [**models::ReportStatus**](ReportStatus.md) |  | 
**report_type** | [**models::ReportType**](ReportType.md) |  | 
**category** | Option<**String**> |  | 
**additional_info** | Option<**String**> |  | 
**reported_user_id** | Option<**String**> |  | 
**reported_user_tag** | Option<**String**> |  | 
**reported_user_username** | Option<**String**> |  | 
**reported_user_discriminator** | Option<**String**> |  | 
**reported_user_avatar_hash** | Option<**String**> |  | 
**reported_guild_id** | Option<**String**> |  | 
**reported_guild_name** | Option<**String**> |  | 
**reported_message_id** | Option<**String**> |  | 
**reported_channel_id** | Option<**String**> |  | 
**reported_channel_name** | Option<**String**> |  | 
**reported_guild_invite_code** | Option<**String**> |  | 
**resolved_at** | Option<**String**> |  | 
**resolved_by_admin_id** | Option<**String**> |  | 
**public_comment** | Option<**String**> |  | 
**mutual_dm_channel_id** | Option<**String**> |  | [optional]
**message_context** | Option<[**Vec<models::ReportAdminResponseSchemaMessageContextInner>**](ReportAdminResponseSchemaMessageContextInner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


