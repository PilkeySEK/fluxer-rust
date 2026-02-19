# UserAdminResponseSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**username** | **String** |  | 
**discriminator** | **i32** |  | 
**global_name** | Option<**String**> |  | 
**bot** | **bool** |  | 
**system** | **bool** |  | 
**flags** | **String** | A single user flag value to add or remove | 
**avatar** | Option<**String**> |  | 
**banner** | Option<**String**> |  | 
**bio** | Option<**String**> |  | 
**pronouns** | Option<**String**> |  | 
**accent_color** | Option<**i32**> |  | 
**email** | Option<**String**> |  | 
**email_verified** | **bool** |  | 
**email_bounced** | **bool** |  | 
**phone** | Option<**String**> |  | 
**date_of_birth** | Option<**String**> |  | 
**locale** | Option<**String**> |  | 
**premium_type** | Option<**i32**> |  | 
**premium_since** | Option<**String**> |  | 
**premium_until** | Option<**String**> |  | 
**suspicious_activity_flags** | **i32** | Bitmask of suspicious activity flags that triggered the disable | 
**temp_banned_until** | Option<**String**> |  | 
**pending_deletion_at** | Option<**String**> |  | 
**pending_bulk_message_deletion_at** | Option<**String**> |  | 
**deletion_reason_code** | Option<**i32**> |  | 
**deletion_public_reason** | Option<**String**> |  | 
**acls** | **Vec<String>** |  | 
**traits** | **Vec<String>** |  | 
**has_totp** | **bool** |  | 
**authenticator_types** | **Vec<i32>** |  | 
**last_active_at** | Option<**String**> |  | 
**last_active_ip** | Option<**String**> |  | 
**last_active_ip_reverse** | Option<**String**> |  | 
**last_active_location** | Option<**String**> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


