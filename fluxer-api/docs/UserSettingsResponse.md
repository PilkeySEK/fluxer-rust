# UserSettingsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **String** | The current online status of the user | 
**status_resets_at** | Option<**String**> |  | [optional]
**status_resets_to** | Option<**String**> |  | [optional]
**theme** | **String** | The UI theme preference | 
**locale** | [**models::Locale**](Locale.md) |  | 
**restricted_guilds** | **Vec<String>** | Guild IDs where direct messages are restricted | 
**bot_restricted_guilds** | **Vec<String>** | Guild IDs where bot direct messages are restricted | 
**default_guilds_restricted** | **bool** | Whether new guilds have DM restrictions by default | 
**bot_default_guilds_restricted** | **bool** | Whether new guilds have bot DM restrictions by default | 
**inline_attachment_media** | **bool** | Whether to display attachments inline in chat | 
**inline_embed_media** | **bool** | Whether to display embed media inline in chat | 
**gif_auto_play** | **bool** | Whether GIFs auto-play in chat | 
**render_embeds** | **bool** | Whether to render message embeds | 
**render_reactions** | **bool** | Whether to display reactions on messages | 
**animate_emoji** | **bool** | Whether to animate custom emoji | 
**animate_stickers** | [**models::StickerAnimationOptions**](StickerAnimationOptions.md) | Sticker animation preference setting | 
**render_spoilers** | [**models::RenderSpoilers**](RenderSpoilers.md) | Spoiler rendering preference setting | 
**message_display_compact** | **bool** | Whether to use compact message display mode | 
**friend_source_flags** | **i32** | Friend source flags | 
**incoming_call_flags** | **i32** | Incoming call settings | 
**group_dm_add_permission_flags** | **i32** | Group DM add permissions | 
**guild_folders** | [**Vec<models::UserSettingsResponseGuildFoldersInner>**](UserSettingsResponseGuildFoldersInner.md) | The folder structure for organizing guilds in the sidebar | 
**custom_status** | Option<[**models::CustomStatusResponse**](CustomStatusResponse.md)> |  | 
**afk_timeout** | **i32** | The idle timeout in seconds before going AFK | 
**time_format** | [**models::TimeFormatTypes**](TimeFormatTypes.md) | The preferred time format setting | 
**developer_mode** | **bool** | Whether developer mode is enabled | 
**trusted_domains** | **Vec<String>** | List of trusted external link domains | 
**default_hide_muted_channels** | **bool** | Whether muted channels are hidden by default in new guilds | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


