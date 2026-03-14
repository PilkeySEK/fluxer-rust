use std::sync::Arc;

use neptunium_http::client::HttpClient;

// use crate::client::http::MessagesClient;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) http_client: Arc<HttpClient>,
}

impl Context {
    pub fn get_http_client(&self) -> &Arc<HttpClient> {
        &self.http_client
    }
}

// impl Context {
//     pub fn messages(&self, channel_id: Id<ChannelMarker>) -> MessagesClient {
//         MessagesClient {
//             http_client: Arc::clone(&self.http_client),
//             channel_id,
//         }
//     }
// }
