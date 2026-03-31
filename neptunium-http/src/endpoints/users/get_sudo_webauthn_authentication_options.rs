use reqwest::Method;
use serde::Deserialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct GetSudoWebauthnAuthenticationOptions;

#[derive(Deserialize, Clone, Debug)]
pub struct GetSudoWebauthnAuthenticationOptionsResponse {
    /// The WebAuthn challenge.
    pub challenge: String,
}

impl Endpoint for GetSudoWebauthnAuthenticationOptions {
    type Response = GetSudoWebauthnAuthenticationOptionsResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path("/users/@me/sudo/webauthn/authentication-options".to_owned())
            .build()
    }
}
