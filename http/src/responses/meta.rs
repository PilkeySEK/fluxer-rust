use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentEndpoints {
    /// Base URL for authenticated API requests.
    pub api: String,
    /// Base URL for client API requests.
    pub api_client: String,
    /// Base URL for public API requests.
    pub api_public: String,
    /// WebSocket URL for the gateway.
    pub gateway: String,
    /// Base URL for the media proxy.
    pub media: String,
    /// Base URL for static assets (avatars, emojis, etc.).
    pub static_cdn: String,
    /// Base URL for the marketing website.
    pub marketing: String,
    /// Base URL for the admin panel.
    pub admin: String,
    /// Base URL for invite links.
    pub invite: String,
    /// Base URL for gift links.
    pub gift: String,
    /// Base URL for the web application.
    pub webapp: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase", tag = "provider")]
pub enum InstanceDiscoveryDocumentCaptchaConfig {
    HCaptcha { hcaptcha_site_key: String },
    Turnstile { turnstile_site_key: String },
    None,
}

#[expect(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct InstanceDiscoveryDocumentFeatures {
    pub sms_mfa_enabled: bool,
    pub voice_enabled: bool,
    pub stripe_enabled: bool,
    pub self_hosted: bool,
    pub manual_review_enabled: bool,
}

/// Single sign-on configuration.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentSSOConfiguration {
    pub enabled: bool,
    pub enforced: bool,
    pub display_name: Option<String>,
    pub redirect_uri: String,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InstanceDiscoveryDocumentGifProviders {
    Klipy,
    Tenor,
}

pub struct InstanceDiscoveryDocumentGifProvider {
    pub provider: InstanceDiscoveryDocumentGifProviders,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentLimitsRuleFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "guildFeatures")]
    pub guild_features: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentLimitsRule {
    /// Unique identifier for this limits rule.
    pub id: String,
    pub overrides: HashMap<String, i64>,
    pub filters: InstanceDiscoveryDocumentLimitsRuleFilters,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentLimits {
    /// Wire format version. Is always `2`.
    pub version: u32,
    /// Available trait definitions, e.g. "premium".
    #[serde(rename = "traitDefinitions")]
    pub trait_definitions: Vec<String>,
    pub rules: Vec<InstanceDiscoveryDocumentLimitsRule>,
    /// Hash of the default limit values for cache invalidation.
    #[serde(rename = "defaultHash")]
    pub default_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentPushNotificationConfig {
    pub public_vapid_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentAppPublic {
    /// Sentry DSN for client-side error reporting.
    pub sentry_dsn: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentFederationConfig {
    pub enabled: bool,
    /// Federation protocol version.
    pub version: u64,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InstanceDiscoveryDocumentPublicKeyAlgorithm {
    #[serde(rename = "x25519")]
    X25519,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentPublicKey {
    pub id: String,
    pub algorithm: InstanceDiscoveryDocumentPublicKeyAlgorithm,
    pub public_key_base64: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentOauth2 {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub scopes_supported: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentResponse {
    /// Version of the API server code.
    pub api_code_version: u64,
    pub endpoints: InstanceDiscoveryDocumentEndpoints,
    pub captcha: InstanceDiscoveryDocumentCaptchaConfig,
    pub features: InstanceDiscoveryDocumentFeatures,
    pub sso: InstanceDiscoveryDocumentSSOConfiguration,
    pub push: InstanceDiscoveryDocumentPushNotificationConfig,
    pub app_public: InstanceDiscoveryDocumentAppPublic,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation: Option<InstanceDiscoveryDocumentFederationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<InstanceDiscoveryDocumentPublicKey>,
    #[expect(clippy::doc_markdown)]
    /// OAuth2 endpoint for federation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<InstanceDiscoveryDocumentOauth2>,
}
