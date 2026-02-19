# GlobalSearchMessagesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hits_per_page** | Option<**i32**> | Number of results per page (1-25) | [optional]
**page** | Option<**i64**> | Page number for pagination | [optional]
**max_id** | Option<**String**> |  | [optional]
**min_id** | Option<**String**> |  | [optional]
**content** | Option<**String**> | Text content to search for | [optional]
**contents** | Option<**Vec<String>**> | Multiple content queries to search for | [optional]
**exact_phrases** | Option<**Vec<String>**> | Exact phrases that must appear contiguously in message content | [optional]
**channel_id** | Option<**Vec<String>**> | Channel IDs to search in | [optional]
**exclude_channel_id** | Option<**Vec<String>**> | Channel IDs to exclude from search | [optional]
**author_type** | Option<[**Vec<models::MessageAuthorType>**](MessageAuthorType.md)> | Author types to filter by | [optional]
**exclude_author_type** | Option<[**Vec<models::MessageAuthorType>**](MessageAuthorType.md)> | Author types to exclude | [optional]
**author_id** | Option<**Vec<String>**> | Author user IDs to filter by | [optional]
**exclude_author_id** | Option<**Vec<String>**> | Author user IDs to exclude | [optional]
**mentions** | Option<**Vec<String>**> | User IDs that must be mentioned | [optional]
**exclude_mentions** | Option<**Vec<String>**> | User IDs that must not be mentioned | [optional]
**mention_everyone** | Option<**bool**> | Filter by whether message mentions everyone | [optional]
**pinned** | Option<**bool**> | Filter by pinned status | [optional]
**has** | Option<[**Vec<models::MessageContentType>**](MessageContentType.md)> | Content types the message must have | [optional]
**exclude_has** | Option<[**Vec<models::MessageContentType>**](MessageContentType.md)> | Content types the message must not have | [optional]
**embed_type** | Option<[**Vec<models::MessageEmbedType>**](MessageEmbedType.md)> | Embed types to filter by | [optional]
**exclude_embed_type** | Option<[**Vec<models::MessageEmbedType>**](MessageEmbedType.md)> | Embed types to exclude | [optional]
**embed_provider** | Option<**Vec<String>**> | Embed providers to filter by | [optional]
**exclude_embed_provider** | Option<**Vec<String>**> | Embed providers to exclude | [optional]
**link_hostname** | Option<**Vec<String>**> | Link hostnames to filter by | [optional]
**exclude_link_hostname** | Option<**Vec<String>**> | Link hostnames to exclude | [optional]
**attachment_filename** | Option<**Vec<String>**> | Attachment filenames to filter by | [optional]
**exclude_attachment_filename** | Option<**Vec<String>**> | Attachment filenames to exclude | [optional]
**attachment_extension** | Option<**Vec<String>**> | File extensions to filter by | [optional]
**exclude_attachment_extension** | Option<**Vec<String>**> | File extensions to exclude | [optional]
**sort_by** | Option<[**models::MessageSortField**](MessageSortField.md)> |  | [optional]
**sort_order** | Option<[**models::MessageSortOrder**](MessageSortOrder.md)> |  | [optional]
**include_nsfw** | Option<**bool**> | Whether to include NSFW channel results | [optional]
**scope** | Option<[**models::MessageSearchScope**](MessageSearchScope.md)> |  | [optional]
**context_channel_id** | Option<**String**> |  | [optional]
**context_guild_id** | Option<**String**> |  | [optional]
**channel_ids** | Option<**Vec<String>**> | Specific channel IDs to search in | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


