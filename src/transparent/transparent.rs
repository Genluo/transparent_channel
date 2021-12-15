use image::Pixel;
use image::{DynamicImage, ImageBuffer, Rgba};

pub fn create_gray_img(_img: &DynamicImage) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    if let DynamicImage::ImageRgba8(image) = _img {
        let (width, height) = image.dimensions();
        let mut out = ImageBuffer::new(width * 2, height);

        for y in 0..height {
            for x in 0..width {
                let p = image.get_pixel(x, y);
                let current_alpha_channel = p.channels()[3];
                if current_alpha_channel == 255 {
                    out.put_pixel(x + width, y, Pixel::from_channels(0, 0, 0, 255));
                    out.put_pixel(x, y, *p);
                } else {
                    out.put_pixel(x + width, y, Pixel::from_channels(255, 255, 255, 255));
                    out.put_pixel(x, y, Pixel::from_channels(255, 255, 255, 255));
                }
            }
        }
        return Some(out);
    }
    None
}
