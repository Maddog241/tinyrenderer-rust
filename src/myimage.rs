use image::{RgbImage, Rgb, ImageOutputFormat};

pub struct MyImage {
    img: RgbImage,
}

impl MyImage {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            img: RgbImage::new(width, height),
        }
    }

    pub fn set(&mut self, x: u32, y: u32, rgb: Rgb<u8>) {
        self.img.put_pixel(x, self.img.dimensions().1-1-y, rgb)
    }

    pub fn write_img(&self, path: &str) {
        let mut writer = std::fs::File::create(path).unwrap();
        self.img.write_to(&mut writer, ImageOutputFormat::Png).unwrap(); 
    }
}