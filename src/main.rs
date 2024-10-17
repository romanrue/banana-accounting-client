mod client;

use client::BananaClient;
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = Url::parse("http://localhost:8081")?;
    let title = "Banana Webserver | Banana Webserver";

    let conn = BananaClient::new(url, title).await?;

    println!(
        "Handshake successfull with BananaServer created with URL: `{}` and Title: `{}`",
        conn.url, conn.title
    );

    Ok(())
}
