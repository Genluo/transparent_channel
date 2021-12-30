use bytes::Bytes;
use reqwest::{get, Error as RequestError, header::HeaderValue};
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

fn get_img_type(head_value: &HeaderValue) -> Option<ContentType> {
    if head_value.eq("image/jpeg") {
        return Some(ContentType::Jpeg);
    } else if head_value.eq("image/png") {
        return Some(ContentType::Png);
    } else if head_value.eq("image/gif") {
        return Some(ContentType::Gif);
    }  else if head_value.eq("image/webp") {
        return Some(ContentType::Webp);
    }
    return None;
}

pub async fn get_image(uri: &str) -> Result<(Bytes, ContentType), GetImageError> {
    let request = get(uri).await?;
    let request_header = request.headers();
    let content_type = request_header.get("content-type");
    if let Some(t) = content_type {
        let current_content_type = match get_img_type(t) {
            Some(t) => t,
            None => {
                return Err(From::from(ContentTypeError::new()));
            }
        };
        return Ok((request.bytes().await?, current_content_type));
    }
    return Err(From::from(ContentTypeError::new()));
}
