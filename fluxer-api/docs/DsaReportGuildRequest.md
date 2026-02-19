# DsaReportGuildRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticket** | **String** | Verification ticket obtained from email verification | 
**additional_info** | Option<**String**> | Additional context or details about the report | [optional]
**reporter_full_legal_name** | **String** | Full legal name of the person filing the report | 
**reporter_country_of_residence** | **ReporterCountryOfResidence** | EU country code of the reporter residence (enum: AT, BE, BG, HR, CY, CZ, DK, EE, FI, FR, DE, GR, HU, IE, IT, LV, LT, LU, MT, NL, PL, PT, RO, SK, SI, ES, SE) | 
**reporter_fluxer_tag** | Option<**String**> | Fluxer tag of the reporter if they have an account | [optional]
**report_type** | **ReportType** | Type of report (enum: guild) | 
**category** | [**models::GuildReportCategoryEnum**](GuildReportCategoryEnum.md) |  | 
**guild_id** | **String** |  | 
**invite_code** | Option<**String**> | Invite code used to access the guild | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


