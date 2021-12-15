mod request;
mod transparent;

use crate::request::get_image;
use crate::transparent::create_gray_img;

use image::ImageError;
use image::{io::Reader, ImageBuffer, Rgba};
use std::{io::Cursor, panic};

pub async fn batch(list: &Vec<String>) -> Result<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, ImageError> {
    let mut result = vec![];
    let suffix = ".png".to_string();
    let mut index = 1;

    for url in list {
        let img = transparent_channel(url).await?;
        img.save(&(index.to_string() + &suffix));
        index += 1;
        result.push(img);
    }

    Ok(result)
}

pub async fn transparent_channel(uri: &str) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, ImageError> {
    let img_content = match get_image(uri).await {
        Ok(content) => content,
        _ => panic!("图片加载失败"),
    };

    let mut img = Reader::new(Cursor::new(img_content));

    img.set_format(image::ImageFormat::Png);

    let d_image = match img.decode() {
        Ok(img) => img,
        Err(err) => panic!("生成image buff失败 {:?}", err),
    };

    let out = match create_gray_img(&d_image) {
        Some(a) => a,
        None => panic!("灰度生成失败！"),
    };

    Ok(out)
}
