# UserSettingsUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flags** | Option<**i32**> | Friend source flags | [optional]
**status** | Option<[**models::UserStatusType**](UserStatusType.md)> |  | [optional]
**status_resets_at** | Option<[**models::UserSettingsUpdateRequestStatusResetsAt**](UserSettingsUpdateRequestStatusResetsAt.md)> |  | [optional]
**status_resets_to** | Option<[**models::UserStatusType**](UserStatusType.md)> |  | [optional]
**theme** | Option<[**models::UserThemeType**](UserThemeType.md)> |  | [optional]
**locale** | Option<[**models::Locale**](Locale.md)> |  | [optional]
**restricted_guilds** | Option<**Vec<String>**> | Guilds with DM restrictions | [optional]
**bot_restricted_guilds** | Option<**Vec<String>**> | Guilds with bot DM restrictions | [optional]
**default_guilds_restricted** | Option<**bool**> | Default DM restriction for new guilds | [optional]
**bot_default_guilds_restricted** | Option<**bool**> | Default bot DM restriction for new guilds | [optional]
**inline_attachment_media** | Option<**bool**> | Display attachments inline | [optional]
**inline_embed_media** | Option<**bool**> | Display embed media inline | [optional]
**gif_auto_play** | Option<**bool**> | Auto-play GIFs | [optional]
**render_embeds** | Option<**bool**> | Render message embeds | [optional]
**render_reactions** | Option<**bool**> | Display reactions | [optional]
**animate_emoji** | Option<**bool**> | Animate custom emoji | [optional]
**animate_stickers** | Option<[**models::StickerAnimationOptions**](StickerAnimationOptions.md)> | Sticker animation preference | [optional]
**render_spoilers** | Option<[**models::RenderSpoilers**](RenderSpoilers.md)> | Spoiler rendering preference | [optional]
**message_display_compact** | Option<**bool**> | Compact message display | [optional]
**friend_source_flags** | Option<**i32**> | Friend source flags | [optional]
**incoming_call_flags** | Option<**i32**> | Incoming call settings | [optional]
**group_dm_add_permission_flags** | Option<**i32**> | Group DM add permissions | [optional]
**guild_folders** | Option<[**Vec<models::UserSettingsUpdateRequestGuildFoldersInner>**](UserSettingsUpdateRequestGuildFoldersInner.md)> | Guild folder organization | [optional]
**custom_status** | Option<[**models::CustomStatusPayload**](CustomStatusPayload.md)> |  | [optional]
**afk_timeout** | Option<**i32**> | AFK timeout in seconds | [optional]
**time_format** | Option<[**models::TimeFormatTypes**](TimeFormatTypes.md)> | Time format preference | [optional]
**developer_mode** | Option<**bool**> | Developer mode enabled | [optional]
**trusted_domains** | Option<**Vec<String>**> | Trusted external link domains. Use \"*\" to trust all domains. | [optional]
**default_hide_muted_channels** | Option<**bool**> | Hide muted channels by default in new guilds | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


