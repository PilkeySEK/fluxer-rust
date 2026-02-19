# WellKnownFluxerResponseLimits

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **Version** | Wire format version (enum: 2) | 
**trait_definitions** | **Vec<String>** | Available trait definitions (e.g., \"premium\") | 
**rules** | [**Vec<models::LimitRuleResponse>**](LimitRuleResponse.md) | Array of limit rules to evaluate | 
**defaults_hash** | **String** | Hash of the default limit values for cache invalidation | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


