# \AttachmentsApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**complete_chunked_upload**](AttachmentsApi.md#complete_chunked_upload) | **POST** /channels/{channel_id}/chunked-uploads/{upload_id}/complete | Complete a chunked upload
[**create_chunked_upload**](AttachmentsApi.md#create_chunked_upload) | **POST** /channels/{channel_id}/chunked-uploads | Initiate a chunked upload session
[**upload_chunk**](AttachmentsApi.md#upload_chunk) | **PUT** /channels/{channel_id}/chunked-uploads/{upload_id}/chunks/{chunk_index} | Upload a file chunk



## complete_chunked_upload

> models::CompleteChunkedUploadResponse complete_chunked_upload(channel_id, upload_id, complete_chunked_upload_request)
Complete a chunked upload

Completes a chunked upload session by assembling all uploaded chunks. Requires ETags for all chunks. Returns the upload filename that can be referenced when sending a message with the uploaded file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**upload_id** | **String** | The upload id | [required] |
**complete_chunked_upload_request** | [**CompleteChunkedUploadRequest**](CompleteChunkedUploadRequest.md) |  | [required] |

### Return type

[**models::CompleteChunkedUploadResponse**](CompleteChunkedUploadResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_chunked_upload

> models::CreateChunkedUploadResponse create_chunked_upload(channel_id, create_chunked_upload_request)
Initiate a chunked upload session

Creates a new chunked upload session for uploading large files. Returns the upload ID, expected chunk size, and total chunk count. The client should then upload each chunk individually and complete the upload when all chunks are uploaded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**create_chunked_upload_request** | [**CreateChunkedUploadRequest**](CreateChunkedUploadRequest.md) |  | [required] |

### Return type

[**models::CreateChunkedUploadResponse**](CreateChunkedUploadResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_chunk

> models::UploadChunkResponse upload_chunk(channel_id, upload_id, chunk_index)
Upload a file chunk

Uploads a single chunk of a file as part of a chunked upload session. The chunk index is zero-based. Returns an ETag that must be provided when completing the upload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**upload_id** | **String** | The upload id | [required] |
**chunk_index** | **String** | The chunk index | [required] |

### Return type

[**models::UploadChunkResponse**](UploadChunkResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

