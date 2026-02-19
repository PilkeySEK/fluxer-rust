# MessageBaseResponseSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier (snowflake) for this message | 
**channel_id** | **String** | The ID of the channel this message was sent in | 
**author** | [**models::UserPartialResponse**](UserPartialResponse.md) |  | 
**webhook_id** | Option<**String**> |  | [optional]
**r#type** | **Type** | The type of message (enum: 0, 1, 2, 3, 4, 5, 6, 7, 19) | 
**flags** | **i32** | Message flags bitfield | 
**content** | **String** | The text content of the message | 
**timestamp** | **String** | The ISO 8601 timestamp of when the message was created | 
**edited_timestamp** | Option<**String**> |  | [optional]
**pinned** | **bool** | Whether the message is pinned | 
**mention_everyone** | **bool** | Whether the message mentions @everyone | 
**tts** | Option<**bool**> | Whether the message was sent as text-to-speech | [optional]
**mentions** | Option<[**Vec<models::UserPartialResponse>**](UserPartialResponse.md)> |  | [optional]
**mention_roles** | Option<**Vec<String>**> |  | [optional]
**embeds** | Option<[**Vec<models::MessageEmbedResponse>**](MessageEmbedResponse.md)> |  | [optional]
**attachments** | Option<[**Vec<models::MessageAttachmentResponse>**](MessageAttachmentResponse.md)> |  | [optional]
**stickers** | Option<[**Vec<models::MessageStickerResponse>**](MessageStickerResponse.md)> |  | [optional]
**reactions** | Option<[**Vec<models::MessageReactionResponse>**](MessageReactionResponse.md)> |  | [optional]
**message_reference** | Option<[**models::MessageReferenceResponse**](MessageReferenceResponse.md)> |  | [optional]
**message_snapshots** | Option<[**Vec<models::MessageSnapshotResponse>**](MessageSnapshotResponse.md)> |  | [optional]
**nonce** | Option<**String**> |  | [optional]
**call** | Option<[**models::MessageCallResponse**](MessageCallResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


