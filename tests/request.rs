extern crate TransparentChannel;

use TransparentChannel::batch;
use image::{ImageError};

#[actix_rt::test]
async fn test_batch() -> Result<(), ImageError> {
    let image_list = vec![
        String::from("https://gw.alicdn.com/imgextra/i3/O1CN01tqFLVn1oddFYIRjUr_!!6000000005248-2-tps-1200-1200.png"),
        String::from("https://gw.alicdn.com/imgextra/i4/O1CN01xsBzZy1wPuJ9GW5fC_!!6000000006301-2-tps-1200-1200.png")
    ];

    let content = batch(&image_list).await?;
    assert_eq!(image_list.len(), content.len());
    Ok(())
}
