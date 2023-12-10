use reqwest::{header::USER_AGENT, Url};

use crate::traits::FromReader;

/// An adapter for automatically loading contents from an Http Get endpoint
#[derive(Debug, Clone)]
pub struct HttpGet<T> {
    /// The URL of the GET request to make
    pub url: Url,

    /// The data received from the request
    pub data: T,
}

impl<T: FromReader> std::str::FromStr for HttpGet<T> {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = s.parse::<Url>()?;
        // let res = reqwest::blocking::get(url.clone())?;
        let res = reqwest::blocking::Client::new()
            .get(url.clone())
            .header(USER_AGENT, env!("CARGO_CRATE_NAME"))
            .send()?;
        let text = res.text()?;
        let mut reader = std::io::Cursor::new(text);
        let data = T::from_reader(&mut reader)?;
        let item = HttpGet { url, data };
        Ok(item)
    }
}
