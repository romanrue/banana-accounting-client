use quick_xml::{events::Event, Reader};
use std::error::Error;
use url::Url;

pub(crate) async fn server_handshake(url: Url, expected_title: &str) -> Result<(), Box<dyn Error>> {
    let html = reqwest::get(url.clone()).await?.text().await?;
    let mut reader = Reader::from_str(&html);
    reader.config_mut().trim_text(true);

    let mut title = String::new();
    let mut match_state = 0;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"html" => match_state += 1,
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"title" && match_state == 1 => {
                title = reader.read_text(e.name())?.into_owned();
                break;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
    }

    if title == expected_title {
        Ok(())
    } else {
        Err(format!(
            "Unexpected Server Response, expected `{}` to be equal to `{}`",
            title, expected_title
        )
        .into())
    }
}
