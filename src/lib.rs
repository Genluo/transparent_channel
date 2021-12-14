use bytes::Bytes;
use image::io::Reader;
use image::ImageError;
use reqwest::{get, Error};
use std::{io::Cursor, panic};

pub async fn get_image(uri: &str) -> Result<Bytes, Error> {
    let response = get(uri).await?.bytes().await?;
    Ok(response)
}

pub async fn get_image_list(uri_list: &Vec<String>) -> Result<Vec<Bytes>, Error> {
    let mut result = vec![];
    for uri in uri_list {
        let next_image = get_image(uri).await?;
        result.push(next_image);
    }
    Ok(result)
}


pub async fn transparent_channel() -> Result<(), ImageError> {
    let img_content = match get_image("https://gw.alicdn.com/imgextra/i3/O1CN01tqFLVn1oddFYIRjUr_!!6000000005248-2-tps-1200-1200.png").await {
        Ok(content) => {
            content
        },
        _ => panic!("图片加载失败")
    };

    let mut img = Reader::new(Cursor::new(img_content));

    img.set_format(image::ImageFormat::Png);

    let d_image = match img.decode() {
        Ok(img) => img,
        Err(err) => panic!("生成image buff失败 {:?}", err),
    };
    Ok(())
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;
    use image::ImageError;
    use reqwest::Error;

    #[actix_rt::test]
    async fn test_get_image() -> Result<(), Error> {
        let content = get_image("https://gw.alicdn.com/imgextra/i3/O1CN01tqFLVn1oddFYIRjUr_!!6000000005248-2-tps-1200-1200.png")
        .await?;
        assert!(content.len() != 0);
        Ok(())
    }

    #[actix_rt::test]
    async fn test_get_image_list() -> Result<(), Error> {
        let image_list = vec![
            String::from("https://gw.alicdn.com/imgextra/i3/O1CN01tqFLVn1oddFYIRjUr_!!6000000005248-2-tps-1200-1200.png"),
            String::from("https://gw.alicdn.com/imgextra/i4/O1CN01xsBzZy1wPuJ9GW5fC_!!6000000006301-2-tps-1200-1200.png")
        ];
        let content = get_image_list(&image_list).await?;
        assert!(content.len() != 0);
        Ok(())
    }

    #[actix_rt::test]
    async fn test_translate_channel() -> Result<(), ImageError> {
        transparent_channel().await?;
        assert_eq!(1, 1);
        Ok(())
    }
}
