use reqwest::Method;
use serde::Deserialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct ListSudoMfaAuthenticationMethods;

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct ListSudoMfaAuthenticationMethodsResponse {
    pub totp: bool,
    pub sms: bool,
    pub webauthn: bool,
    /// Whether any mfa method is enabled.
    pub has_mfa: bool,
}

impl Endpoint for ListSudoMfaAuthenticationMethods {
    type Response = ListSudoMfaAuthenticationMethodsResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/users/@me/sudo/mfa-methods".to_owned())
            .build()
    }
}
