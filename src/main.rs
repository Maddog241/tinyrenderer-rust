mod myimage;
mod vec;

use std::mem::swap;

use image::Rgb;
use myimage::MyImage;
use vec::Vec3f;

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
}

fn barycentric(x: u32, y: u32, ax: u32, ay: u32, bx: u32, by: u32, cx: u32, cy: u32) -> (f64, f64, f64) {
    let vec_x = Vec3f::new(bx as f64 - ax as f64, cx as f64 - ax as f64, ax as f64 - x as f64);
    let vec_y = Vec3f::new(by as f64 - ay as f64, cy as f64 - ay as f64, ay as f64 - y as f64);
    let mut uv1 = vec_x.cross(vec_y);
    if uv1.z == 0.0 {
        // in this case, the triangle is degenerate
        (-1.0, 1.0, 1.0)
    } else {
        uv1 /= uv1.z;
        let (u, v) = (uv1.x, uv1.y);

        (1.0-u-v, u, v)
    }
}


fn triangle(image: &mut MyImage, ax: u32, ay: u32, bx: u32, by: u32, cx: u32, cy: u32, color: Rgb<u8>) {
    let (image_width, image_height) = image.dimensions();
    let (x_min, x_max) = (ax.min(bx.min(cx)), ax.max(bx.max(cx)).min(image_width-1));
    let (y_min, y_max) = (ay.min(by.min(cy)), ay.max(by.max(cy)).min(image_height-1));

    for x in x_min..(x_max+1) {
        for y in y_min..(y_max+1) {
            let (u_a, u_b, u_c) = barycentric(x, y, ax, ay, bx, by, cx, cy);
            if u_a >= 0.0 && u_b >= 0.0 && u_c >= 0.0 {
                image.set(x, y, color);
            }
        }
    }
}

fn main() {
    let mut image = MyImage::new(200, 200);

    triangle(&mut image, 10, 10, 100, 30, 280, 160, RED);

    image.write_img("output.png");
}
