mod myimage;
mod model;

use std::mem::swap;

use cgmath::{Vector3, Point2};
use image::Rgb;
use model::Model;
use myimage::MyImage;
use rand::random;

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const RED: Rgb<u8> = Rgb([255, 0, 0]);
const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const BLUE: Rgb<u8> = Rgb([0, 0, 255]);

fn line(image: &mut MyImage, p0: Point2<u32>, p1: Point2<u32>, color: Rgb<u8>) {
    let (mut x0, mut x1) = (p0.x as f64, p1.x as f64);
    let (mut y0, mut y1) = (p0.y as f64, p1.y as f64);

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

fn barycentric(p: Point2<u32>, a: Point2<u32>, b: Point2<u32>, c: Point2<u32>) -> (f32, f32, f32) {
    let vec_x = Vector3::new(b.x as f32 - a.x as f32, c.x as f32 - a.x as f32, a.x as f32 - p.x as f32);
    let vec_y = Vector3::new(b.y as f32 - a.y as f32, c.y as f32 - a.y as f32, a.y as f32 - p.y as f32);
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


fn triangle(image: &mut MyImage, a: Point2<u32>, b: Point2<u32>, c: Point2<u32>, color: Rgb<u8>) {
    let (image_width, image_height) = image.dimensions();
    let (x_min, x_max) = (a.x.min(b.x.min(c.x)), a.x.max(b.x.max(c.x)).min(image_width-1));
    let (y_min, y_max) = (a.y.min(b.y.min(c.y)), a.y.max(b.y.max(c.y)).min(image_height-1));

    for x in x_min..(x_max+1) {
        for y in y_min..(y_max+1) {
            let (u_a, u_b, u_c) = barycentric(Point2::new(x,y), a, b, c);
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
        let v0 = model.verts[abc[0]];
        let v1 = model.verts[abc[1]];
        let v2 = model.verts[abc[2]];

        let x0 = ((v0.x+1.0)/2.0 * (width-1) as f32) as u32;
        let y0 = ((v0.y+1.0)/2.0 * (height-1) as f32) as u32;
        let x1 = ((v1.x+1.0)/2.0 * (width-1) as f32) as u32;
        let y1 = ((v1.y+1.0)/2.0 * (height-1) as f32) as u32;
        let x2 = ((v2.x+1.0)/2.0 * (width-1) as f32) as u32;
        let y2 = ((v2.y+1.0)/2.0 * (height-1) as f32) as u32;

        triangle(&mut image, Point2::new(x0, y0), Point2::new(x1, y1), Point2::new(x2, y2), Rgb([random::<u8>(), random::<u8>(), random::<u8>()]));
    }

    image.write_img("output.png");
}
