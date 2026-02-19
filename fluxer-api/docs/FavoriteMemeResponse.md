# FavoriteMemeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the favorite meme | 
**user_id** | **String** | ID of the user who owns this favorite meme | 
**name** | **String** | Display name of the meme | 
**alt_text** | Option<**String**> |  | [optional]
**tags** | **Vec<String>** | Tags for categorizing and searching the meme | 
**attachment_id** | **String** | ID of the attachment storing the meme | 
**filename** | **String** | Original filename of the meme | 
**content_type** | **String** | MIME type of the meme file | 
**content_hash** | Option<**String**> |  | [optional]
**size** | **f64** | File size in bytes | 
**width** | Option<**i32**> |  | [optional]
**height** | Option<**i32**> |  | [optional]
**duration** | Option<**f64**> |  | [optional]
**url** | **String** | CDN URL to access the meme | 
**is_gifv** | Option<**bool**> | Whether the meme is a video converted from GIF | [optional]
**klipy_slug** | Option<**String**> |  | [optional]
**tenor_slug_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


