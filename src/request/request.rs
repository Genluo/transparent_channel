use bytes::Bytes;
use reqwest::{get, Error};

pub async fn get_image(uri: &str) -> Result<Bytes, Error> {
    let response = get(uri).await?.bytes().await?;
    Ok(response)
}
