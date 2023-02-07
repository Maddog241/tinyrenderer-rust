mod myimage;
mod model;

use std::mem::swap;

use cgmath::Vector3;
use image::Rgb;
use model::Model;
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
    let vec_x = Vector3::new(bx as f64 - ax as f64, cx as f64 - ax as f64, ax as f64 - x as f64);
    let vec_y = Vector3::new(by as f64 - ay as f64, cy as f64 - ay as f64, ay as f64 - y as f64);
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
    let (width, height) = (800, 800);
    let mut image = MyImage::new(width, height);

    let model = Model::new("./obj/african_head.obj");

    for abc in model.faces {
        for i in 0..3 {
            let v0 = model.verts[abc[i]];
            let v1 = model.verts[abc[(i+1)%3]];
            let x0 = ((v0.x+1.0)/2.0 * (width-1) as f32) as u32;
            let y0 = ((v0.y+1.0)/2.0 * (height-1) as f32) as u32;
            let x1 = ((v1.x+1.0)/2.0 * (width-1) as f32) as u32;
            let y1 = ((v1.y+1.0)/2.0 * (height-1) as f32) as u32;
            line(&mut image, x0, y0, x1, y1, WHITE);
        }
    }

    image.write_img("output.png");
}
