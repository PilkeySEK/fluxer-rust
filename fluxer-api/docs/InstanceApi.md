# \InstanceApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_well_known_fluxer**](InstanceApi.md#get_well_known_fluxer) | **GET** /.well-known/fluxer | Get instance discovery document



## get_well_known_fluxer

> models::WellKnownFluxerResponse get_well_known_fluxer()
Get instance discovery document

Returns the instance discovery document including API endpoints, feature flags, limits, and federation capabilities. This is the canonical discovery endpoint for all Fluxer clients.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WellKnownFluxerResponse**](WellKnownFluxerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

