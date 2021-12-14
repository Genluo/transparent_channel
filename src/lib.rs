use reqwest::get;
use reqwest::Error;


pub async fn get_image(uri: &str) -> Result<String, Error> {
    let response = get(uri)
        .await?
        .text()
        .await?;
    Ok(response)
}

pub async fn get_image_list(uri_list: &Vec<String>) -> Result<String, Error> {
    let mut result  = String::from("");
    for uri in uri_list {
        let nextImage = get_image(uri).await?;
        result.push_str(&nextImage);
    }
    Ok(result)
}


#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;

    #[actix_rt::test]
    async fn test_get_image() -> Result<(), Error>{
        let content = get_image("https://gw.alicdn.com/imgextra/i3/O1CN01tqFLVn1oddFYIRjUr_!!6000000005248-2-tps-1200-1200.png")
        .await?;
        assert_ne!(content, String::from(""));
        Ok(())
    }

    #[actix_rt::test]
    async fn test_get_image_list() -> Result<(), Error> {
        let image_list = vec![
            String::from("https://gw.alicdn.com/imgextra/i3/O1CN01tqFLVn1oddFYIRjUr_!!6000000005248-2-tps-1200-1200.png"),
            String::from("https://gw.alicdn.com/imgextra/i4/O1CN01xsBzZy1wPuJ9GW5fC_!!6000000006301-2-tps-1200-1200.png")
        ];
        let content = get_image_list(&image_list).await?;
        assert_ne!(content, String::from(""));
        Ok(())
    }
}
