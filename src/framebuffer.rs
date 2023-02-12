use cgmath::Vector3;
use image::{ImageOutputFormat, Rgb};

use crate::mygl::Float;

pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    buf: Vec<Vector3<Float>>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buf: vec![Vector3::new(0.0, 0.0, 0.0); width as usize * height as usize],
        }
    }

    pub fn set(&mut self, x: u32, y: u32, val: Vector3<Float>) {
        self.buf[x as usize + y as usize * self.width as usize] = val
    }

    pub fn get(&self, x: u32, y: u32) -> Vector3<Float> {
        self.buf[x as usize + y as usize * self.width as usize]
    }

    pub fn write_img(&self, path: &str) {
        let mut writer = std::fs::File::create(path).unwrap();
        let mut img = image::RgbImage::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let v = self.buf[x as usize + y as usize * self.width as usize];
                let pixel = Rgb([(v.x.clamp(0.0, 1.0)*255.0) as u8, (v.y.clamp(0.0, 1.0)*255.0) as u8, (v.z.clamp(0.0, 1.0)*255.0) as u8]);
                img.put_pixel(x, self.height - 1 - y, pixel);
            }
        }

        img.write_to(&mut writer, ImageOutputFormat::Png).unwrap();
    }
}
