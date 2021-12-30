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

#[derive(Debug)]
pub enum ContentType {
    Gif,
    Jpeg,
    Png,
    Webp,
}

pub async fn get_image(uri: &str) -> Result<(Bytes, ContentType), GetImageError> {
    let request = get(uri).await?;
    let request_header = request.headers();
    let content_type = request_header.get("content-type");
    if let Some(t) = content_type {
        let mut current_content_type = ContentType::Gif;
        if t.eq("image/jpeg") {
            current_content_type = ContentType::Jpeg;
        } else if t.eq("image/png") {
            current_content_type = ContentType::Png;
        } else if t.eq("image/gif") {
            current_content_type = ContentType::Gif;
        }  else if t.eq("image/webp") {
            current_content_type = ContentType::Webp;
        } else {
            return Err(From::from(ContentTypeError::new()));
        }
        return Ok((request.bytes().await?, current_content_type));
    }
    return Err(From::from(ContentTypeError::new()));
}
