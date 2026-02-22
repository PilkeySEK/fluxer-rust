use fluxer_gateway::{
    fluxer_api::models::UserPrivateResponse,
    model::{
        event::dispatch::session::{GuildReadyResponse, ReadyDispatchData},
        snowflake::Snowflake,
    },
};

#[derive(Clone, Debug)]
pub struct ReadyEventData {
    pub dispatch_data: ReadyDispatchData,
}

impl ReadyEventData {
    #[must_use]
    pub fn user(&self) -> &UserPrivateResponse {
        &self.dispatch_data.user
    }

    #[must_use]
    pub fn guilds(&self) -> &Vec<GuildReadyResponse> {
        &self.dispatch_data.guilds
    }
}

#[derive(Clone, Debug)]
pub struct MessageCreateEventData {}

#[derive(Clone, Debug)]
pub struct GuildDeleteEventData {
    pub id: Snowflake,
    pub unavailable: bool,
}

#[derive(Clone, Debug)]
pub struct GuildCreateEventData {}
