use crate::__fluxer_gateway_bitflags_as_number;

const DEFAULT_GATEWAY_URL: &str = "wss://gateway.fluxer.app";
const DEFAULT_GATEWAY_VERSION: GatewayVersion = 1;

pub type GatewayVersion = i32;

__fluxer_gateway_bitflags_as_number! {
    GatewayIntentsDef =>
    /// See https://docs.discord.com/developers/events/gateway#gateway-intents
    #[derive(Copy, Clone, Debug)]
    pub struct GatewayIntents: u32 {
        const GUILDS = 1 << 0;
        const GUILD_MEMBERS = 1 << 1;
        const GUILD_MODERATION = 1 << 2;
        const GUILD_EXPRESSIONS = 1 << 3;
        const GUILD_INTEGRATIONS = 1 << 4;
        const GUILD_WEBHOOKS = 1 << 5;
        const GUILD_INVITES = 1 << 6;
        const GUILD_VOICE_STATES = 1 << 7;
        const GUILD_PRESENCES = 1 << 8;
        const GUILD_MESSAGES = 1 << 9;
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        const GUILD_MESSAGE_TYPING = 1 << 11;
        const DIRECT_MESSAGES = 1 << 12;
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;
        const DIRECT_MESSAGE_TYPING = 1 << 14;
        const MESSAGE_CONTENT = 1 << 15;
        const GUILD_SCHEDULED_EVENTS = 1 << 16;
        const AUTO_MODERATION_CONFIGURATION = 1 << 20;
        const AUTO_MODERATION_EXECUTION = 1 << 21;
        const GUILD_MESSAGE_POLLS = 1 << 24;
        const DIRECT_MESSAGE_POLLS = 1 << 25;
    }
}

#[expect(unused)]
pub struct GatewayClientConfiguration<'a> {
    pub(super) intents: GatewayIntents,
    pub(super) token: &'a str,
    pub(super) gateway_url: &'a str,
    pub(super) version: GatewayVersion,
}

impl<'a> GatewayClientConfiguration<'a> {
    #[must_use]
    pub fn new(token: &'a str, intents: GatewayIntents) -> Self {
        Self {
            intents,
            token,
            gateway_url: DEFAULT_GATEWAY_URL,
            version: DEFAULT_GATEWAY_VERSION,
        }
    }

    pub fn set_gateway_url(&mut self, url: &'a str) {
        self.gateway_url = url;
    }

    pub fn set_gateway_version(&mut self, version: GatewayVersion) {
        self.version = version;
    }
}
