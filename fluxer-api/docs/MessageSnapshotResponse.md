# MessageSnapshotResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | Option<**String**> |  | [optional]
**timestamp** | **String** | The ISO 8601 timestamp of when the original message was created | 
**edited_timestamp** | Option<**String**> |  | [optional]
**mentions** | Option<**Vec<String>**> |  | [optional]
**mention_roles** | Option<**Vec<String>**> |  | [optional]
**embeds** | Option<[**Vec<models::MessageEmbedResponse>**](MessageEmbedResponse.md)> |  | [optional]
**attachments** | Option<[**Vec<models::MessageAttachmentResponse>**](MessageAttachmentResponse.md)> |  | [optional]
**stickers** | Option<[**Vec<models::MessageStickerResponse>**](MessageStickerResponse.md)> |  | [optional]
**r#type** | **Type** | The type of message (enum: 0, 1, 2, 3, 4, 5, 6, 7, 19) | 
**flags** | **i32** | Message flags bitfield | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


