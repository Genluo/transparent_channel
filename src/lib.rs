mod request;
mod transparent;

use crate::request::get_image;
use crate::transparent::create_gray_img;

use image::ImageError;
use image::{io::Reader, ImageBuffer, Rgba};
use std::{io::Cursor, panic};
use request::GetImageError;

#[derive(Debug)]
pub enum TransformError {
    ImageError(ImageError),
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

pub async fn transparent_channel(uri: &str) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, TransformError> {
    let (img_content, content_type) = get_image(uri).await?;

    let mut img = Reader::new(Cursor::new(img_content));

    println!("content {:?}", content_type);
    img.set_format(image::ImageFormat::Png);

    let d_image = img.decode()?;

    let out = match create_gray_img(&d_image) {
        Some(a) => a,
        None => panic!("灰度生成失败！"),
    };

    Ok(out)
}
