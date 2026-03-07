use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::UserMarker},
    misc::{HexColor32, ImageHash},
    user::flags::PublicUserFlags,
};

pub mod flags;
pub mod read_state;
pub mod settings;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPartial {
    avatar: Option<ImageHash>,
    avatar_color: Option<HexColor32>,
    #[serde(default)]
    bot: bool,
    // TODO: Maybe store as u16 instead
    discriminator: String,
    flags: PublicUserFlags,
    global_name: Option<String>,
    id: Id<UserMarker>,
    #[serde(default)]
    system: bool,
    /// Note that this is not unique (because Fluxer has discriminators too)
    username: String,
}
