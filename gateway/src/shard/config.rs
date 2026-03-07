use fluxer_model::gateway::{intents::Intents, shard::ShardInfo};
use zeroize::Zeroizing;

#[derive(Debug)]
pub struct ShardConfig {
    pub shard_info: ShardInfo,
    pub gateway_url: String,
    pub token: Zeroizing<String>,
    pub intents: Intents,
}

impl ShardConfig {
    pub const DEFAULT_GATEWAY_URL: &str = "wss://gateway.fluxer.app/?v=1&encoding=json";
}

pub struct ShardConfigBuilder {
    shard_info: Option<ShardInfo>,
    gateway_url: Option<String>,
    token: Zeroizing<String>,
    intents: Intents,
}

impl ShardConfigBuilder {
    #[must_use]
    pub fn new(token: String, intents: Intents) -> Self {
        Self {
            shard_info: None,
            gateway_url: None,
            token: Zeroizing::new(token),
            intents,
        }
    }

    #[must_use]
    pub fn gateway_url(mut self, url: impl Into<String>) -> Self {
        self.gateway_url = Some(url.into());
        self
    }

    #[must_use]
    pub fn shard_info(mut self, info: ShardInfo) -> Self {
        self.shard_info = Some(info);
        self
    }

    #[must_use]
    pub fn build(self) -> ShardConfig {
        ShardConfig {
            shard_info: self.shard_info.unwrap_or(ShardInfo::ONE),
            gateway_url: self
                .gateway_url
                .unwrap_or_else(|| ShardConfig::DEFAULT_GATEWAY_URL.to_owned()),
            token: self.token,
            intents: self.intents,
        }
    }
}
