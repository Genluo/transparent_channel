mod request;
mod transparent;

use crate::request::{get_image, GetImageError, ContentType};
use crate::transparent::create_gray_img;

use bytes::Bytes;
use image::DynamicImage;
use image::{io::Reader, ImageBuffer, Rgba, ImageError};
use std::{io::Cursor, panic};

#[derive(Debug)]
pub enum TransformError {
    /**
     * 图片处理报错
     */
    ImageError(ImageError),
    /**
     * 图片处理报错
     */
    RequestError(GetImageError),
}

impl From<ImageError> for TransformError {
    fn from(err: ImageError) -> TransformError {
        TransformError::ImageError(err)
    }
}

impl From<GetImageError> for TransformError {
    fn from(err: GetImageError) -> TransformError {
        TransformError::RequestError(err)
    }
}


const FILE_SUFFIX: &str = ".png";

pub async fn batch(list: &Vec<String>) -> Result<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, TransformError> {
    let mut result = vec![];

    for (index, url) in list.iter().enumerate() {
        println!("current index {}", url);
        let img = transparent_channel(url).await?;
        img.save(format!("{}{}", index, FILE_SUFFIX))?;
        result.push(img);
    }

    Ok(result)
}


async fn transparent_channel(uri: &str) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, TransformError> {
    let (img_content, content_type) = get_image(uri).await?;

    let d_image = format_image(img_content, content_type)?;

    let out = match create_gray_img(&d_image) {
        Some(a) => a,
        None => panic!("grayscale generation failed"),
    };

    Ok(out)
}


fn format_image(image_content: Bytes, content_type: ContentType) -> Result<DynamicImage, ImageError> {
    let mut img = Reader::new(Cursor::new(image_content));

    match content_type {
        ContentType::Gif => {
            img.set_format(image::ImageFormat::Gif);
        }
        ContentType::Jpeg => {
            img.set_format(image::ImageFormat::Jpeg);
        }
        ContentType::Png => {
            img.set_format(image::ImageFormat::Png);
        }
        ContentType::Webp => {
            img.set_format(image::ImageFormat::WebP);
        }
    }

    img.decode()
}
