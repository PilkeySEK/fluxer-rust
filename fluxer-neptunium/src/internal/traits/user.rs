use neptunium_model::{
    gateway::payload::incoming::UserPrivateResponse,
    guild::member::GuildMember,
    id::{Id, marker::UserMarker},
    user::PartialUser,
};

pub trait UserTrait: Sync {
    fn get_user_id(&self) -> Id<UserMarker>;
}

impl UserTrait for Id<UserMarker> {
    fn get_user_id(&self) -> Id<UserMarker> {
        *self
    }
}

impl UserTrait for PartialUser {
    fn get_user_id(&self) -> Id<UserMarker> {
        self.id
    }
}

impl UserTrait for UserPrivateResponse {
    fn get_user_id(&self) -> Id<UserMarker> {
        self.id
    }
}

impl UserTrait for GuildMember {
    fn get_user_id(&self) -> Id<UserMarker> {
        self.user.id
    }
}
