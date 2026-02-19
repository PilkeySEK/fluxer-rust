# UserPrivateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier (snowflake) for this user | 
**username** | **String** | The username of the user, not unique across the platform | 
**discriminator** | **String** | The four-digit discriminator tag of the user | 
**global_name** | Option<**String**> |  | 
**avatar** | Option<**String**> |  | 
**avatar_color** | Option<**i32**> |  | 
**bot** | Option<**bool**> | Whether the user is a bot account | [optional]
**system** | Option<**bool**> | Whether the user is an official system user | [optional]
**flags** | **i32** | The public flags on the user account | 
**is_staff** | **bool** | Whether the user has staff permissions | 
**acls** | **Vec<String>** | Access control list entries for the user | 
**traits** | **Vec<String>** | Special traits assigned to the user account | 
**email** | Option<**String**> |  | 
**email_bounced** | Option<**bool**> | Whether the current email address is marked as bounced by the mail provider | [optional]
**phone** | Option<**String**> |  | 
**bio** | Option<**String**> |  | 
**pronouns** | Option<**String**> |  | 
**accent_color** | Option<**i32**> |  | 
**banner** | Option<**String**> |  | 
**banner_color** | Option<**i32**> |  | 
**mfa_enabled** | **bool** | Whether multi-factor authentication is enabled | 
**authenticator_types** | Option<[**Vec<models::UserAuthenticatorTypes>**](UserAuthenticatorTypes.md)> | The types of authenticators configured for MFA | [optional]
**verified** | **bool** | Whether the email address has been verified | 
**premium_type** | Option<[**models::UserPremiumTypes**](UserPremiumTypes.md)> | The type of premium subscription | 
**premium_since** | Option<**String**> |  | 
**premium_until** | Option<**String**> |  | 
**premium_will_cancel** | **bool** | Whether premium is set to cancel at the end of the billing period | 
**premium_billing_cycle** | Option<**String**> |  | 
**premium_lifetime_sequence** | Option<**i32**> |  | 
**premium_badge_hidden** | **bool** | Whether the premium badge is hidden on the profile | 
**premium_badge_masked** | **bool** | Whether the premium badge shows a masked appearance | 
**premium_badge_timestamp_hidden** | **bool** | Whether the premium start timestamp is hidden | 
**premium_badge_sequence_hidden** | **bool** | Whether the lifetime sequence number is hidden | 
**premium_purchase_disabled** | **bool** | Whether premium purchases are disabled for this account | 
**premium_enabled_override** | **bool** | Whether premium features are enabled via override | 
**password_last_changed_at** | Option<**String**> |  | 
**required_actions** | Option<**Vec<String>**> |  | 
**nsfw_allowed** | **bool** | Whether the user is allowed to view NSFW content | 
**has_dismissed_premium_onboarding** | **bool** | Whether the user has dismissed the premium onboarding flow | 
**has_ever_purchased** | **bool** | Whether the user has ever made a purchase | 
**has_unread_gift_inventory** | **bool** | Whether there are unread items in the gift inventory | 
**unread_gift_inventory_count** | **i32** | The number of unread gift inventory items | 
**used_mobile_client** | **bool** | Whether the user has ever used the mobile client | 
**pending_bulk_message_deletion** | Option<[**models::UserPrivateResponsePendingBulkMessageDeletion**](UserPrivateResponsePendingBulkMessageDeletion.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


