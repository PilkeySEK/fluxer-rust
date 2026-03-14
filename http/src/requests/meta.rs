use reqwest::Method;

use crate::{
    endpoints::Endpoint, requests::Request, responses::meta::InstanceDiscoveryDocumentResponse,
};

pub struct InstanceDiscoveryDocument;

impl Endpoint for InstanceDiscoveryDocument {
    type Response = InstanceDiscoveryDocumentResponse;

    fn into_request(self) -> crate::requests::Request {
        Request::builder()
            .use_authorization_token(false)
            .method(Method::GET)
            .path(String::from("/.well-known/fluxer"))
            .build()
    }
}
