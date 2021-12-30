use bytes::Bytes;
use reqwest::{get, Error as RequestError};
use crate::request::ContentTypeError;


#[derive(Debug)]
pub enum GetImageError {
    RequestError(RequestError),
    ContentTypeError(ContentTypeError),
}

impl From<RequestError> for GetImageError {
    fn from(err: RequestError) -> Self {
        GetImageError::RequestError(err)
    }
}
impl From<ContentTypeError> for GetImageError {
    fn from(err: ContentTypeError) -> Self {
        GetImageError::ContentTypeError(err)
    }
}

pub enum ContentType {
    Gif,
    Jpeg,
    Png,
    Webp,
}

pub async fn get_image(uri: &str) -> Result<Bytes, GetImageError> {
    let request = get(uri).await?;
    let request_header = request.headers();
    let content_type = request_header.get("content-type");
    if let Some(t) = content_type {
        let result = request.bytes().await?;
        Ok(result)
    } else {
        Err(From::from(ContentTypeError::new()))
    }
}
