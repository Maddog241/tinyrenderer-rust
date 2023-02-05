mod myimage;


use std::mem::swap;

use image::Rgb;
use myimage::MyImage;

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const RED: Rgb<u8> = Rgb([255, 0, 0]);
const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const BLUE: Rgb<u8> = Rgb([0, 0, 255]);

fn line(image: &mut MyImage, x0: u32, y0: u32, x1: u32, y1: u32, color: Rgb<u8>) {
    let (mut x0, mut x1) = (x0 as f64, x1 as f64);
    let (mut y0, mut y1) = (y0 as f64, y1 as f64);

    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        steep = true;
    }

    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = (x1 - x0) as i32;

    if dx != 0 {
        let derror = 2 * (y1 - y0).abs() as i32;
        let mut error = 0;

        let mut y = y0 as u32;
        for x in (x0 as u32)..(x1 as u32 + 1) {
            if !steep {
                image.set(x, y, color);
            } else {
                image.set(y, x, color);
            }
    
            error += derror;
            if error > dx {
                if y1 >= y0 {
                    y += 1;
                } else {
                    y -= 1;
                }
                error -= dx * 2;
            }
        }
    } else {
        image.set(x0 as u32, y0 as u32, color);
    }
}

fn main() {
    let mut image = MyImage::new(100, 100);

    // horizontal line 
    line(&mut image, 10, 10, 30, 10, GREEN);
    // vertical line 
    line(&mut image, 30, 10, 30, 40, RED);
    // upper-left to lower-right
    line(&mut image, 60, 50, 70, 20, BLUE);
    // upper-right to lower-left
    line(&mut image, 70, 50, 60, 20, RED);
    // lower-left to upper-right
    line(&mut image, 75, 10, 95, 90, WHITE);
    // lower-right to upper-left
    line(&mut image, 95, 10, 75, 90, WHITE);

    image.write_img("output.png");
}
