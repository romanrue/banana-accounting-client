mod networking;

use networking::server_handshake;
use std::error::Error;
use url::Url;

pub struct BananaClient {
    pub(crate) url: Url,
    pub(crate) title: String,
}

impl BananaClient {
    pub async fn new(url: Url, expected_title: &str) -> Result<Self, Box<dyn Error>> {
        if let Err(e) = server_handshake(url.clone(), expected_title).await {
            return Err(format!("Handshake failed for URL {}: {}", url, e).into());
        }

        Ok(BananaClient {
            url,
            title: expected_title.to_owned(),
        })
    }
}
