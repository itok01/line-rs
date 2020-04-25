use reqwest::header;
use serde::Serialize;
use std::error::Error;

pub async fn get<S: Into<String>, Q: Serialize + ?Sized>(
    channel_access_token: S,
    endpoint: S,
    query: &Q,
) -> Result<reqwest::Response, Box<dyn Error>> {
    let client = reqwest::Client::new();
    Ok(client
        .get(&endpoint.into())
        .query(query)
        .bearer_auth(channel_access_token.into())
        .send()
        .await?)
}

pub async fn post_json<S: Into<String>>(
    channel_access_token: S,
    endpoint: S,
    body: S,
) -> Result<reqwest::Response, Box<dyn Error>> {
    let client = reqwest::Client::new();
    Ok(client
        .post(&endpoint.into())
        .header(header::CONTENT_TYPE, "application/json")
        .bearer_auth(channel_access_token.into())
        .body(body.into())
        .send()
        .await?)
}
