mod myimage;


use image::Rgb;
use myimage::MyImage;

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const RED: Rgb<u8> = Rgb([255, 0, 0]);

fn main() {
    let mut image = MyImage::new(100, 100);

    image.set(52, 40, RED);

    image.write_img("output.png");
}
