use bytes::Bytes;
use reqwest::get;
use std::stream::{Stream};

pub async fn get_image_stream(uri: &str) -> Stream<Item = Result<Bytes>> {
    let stream = get(uri).await?.bytes_stream();
    stream
}

pub async fn get_image_list_stream(uri_list: Vec<String>) -> Stream<Item = Result<Bytes>> {
}