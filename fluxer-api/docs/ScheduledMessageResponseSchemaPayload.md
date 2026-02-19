# ScheduledMessageResponseSchemaPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | Option<**String**> |  | [optional]
**tts** | Option<**bool**> | Whether this is a text-to-speech message | [optional]
**embeds** | Option<[**Vec<models::MessageEmbedResponse>**](MessageEmbedResponse.md)> | Array of embed objects attached to the message | [optional]
**attachments** | Option<[**Vec<models::MessageAttachmentResponse>**](MessageAttachmentResponse.md)> | Array of attachment objects for the message | [optional]
**stickers** | Option<[**Vec<models::MessageStickerResponse>**](MessageStickerResponse.md)> | Array of sticker objects attached to the message | [optional]
**sticker_ids** | Option<**Vec<String>**> | Array of sticker IDs to include in the message | [optional]
**allowed_mentions** | Option<[**models::ScheduledMessageAllowedMentionsSchema**](ScheduledMessageAllowedMentionsSchema.md)> |  | [optional]
**message_reference** | Option<[**models::ScheduledMessageReferenceSchema**](ScheduledMessageReferenceSchema.md)> |  | [optional]
**flags** | Option<**i32**> | Message flags bitfield | [optional]
**nonce** | Option<**String**> | Client-generated identifier for the message | [optional]
**favorite_meme_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


