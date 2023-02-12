use std::mem::swap;

use cgmath::{Vector3, Point2, Point3, InnerSpace, Matrix4, ortho};
use image::{Rgb, DynamicImage};

use crate::{myimage::MyImage, vertex::Vertex, shader::Shader, WIDTH};

#[allow(dead_code)]
pub fn line(image: &mut MyImage, p0: Point2<u32>, p1: Point2<u32>, color: Rgb<u8>) {
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

pub fn barycentric(p: Point3<f32>, a: Point3<f32>, b: Point3<f32>, c: Point3<f32>) -> Vector3<f32> {
    let vec_x = Vector3::new(b.x - a.x, c.x - a.x, a.x - p.x);
    let vec_y = Vector3::new(b.y - a.y, c.y - a.y, a.y - p.y);
    let mut uv1 = vec_x.cross(vec_y);
    if uv1.z.abs() < 1e-2 {
        // in this case, the triangle is degenerate
        Vector3::new(-1.0, 1.0, 1.0)
    } else {
        uv1 /= uv1.z;
        let (u, v) = (uv1.x, uv1.y);

        Vector3::new(1.0 - u - v, u, v)
    }
}


pub fn texture_2d(texture: &DynamicImage, tex_coord: Point2<f32>) -> Vector3<f32> {
    let (tex_width, tex_height) = (texture.width() as f32, texture.height() as f32);
    let (u, v) = (tex_coord.x, tex_coord.y);
    let (i, j) = (
        (u * (tex_width - 0.001)) as usize,
        (v * (tex_height - 0.001)) as usize,
    );

    let bytes = texture.as_bytes();
    // if the image has no alpha channel, k = 3, else 4 ?
    let k = bytes.len() / tex_width as usize / tex_height as usize;

    let index = (tex_height as usize - 1 - j) * tex_width as usize + i;
    let (r, g, b) = (bytes[k * index], bytes[k * index + 1], bytes[k * index + 2]);

    Vector3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

pub fn triangle(
    image: &mut MyImage,
    zbuffer: &mut Vec<f32>,
    shader: &impl Shader,
    v: Vec<Vertex>
) {
    assert!(v.len() == 3);

    let (image_width, image_height) = image.dimensions();
    let mut bbox_min = Point2::new(image_width as f32 - 1.0, image_height as f32 - 1.0);
    let mut bbox_max = Point2::new(0.0f32, 0.0f32);

    for point in [v[0].gl_position, v[1].gl_position, v[2].gl_position] {
        bbox_min.x = bbox_min.x.min(point.x);
        bbox_min.y = bbox_min.y.min(point.y);
        bbox_max.x = bbox_max.x.max(point.x);
        bbox_max.y = bbox_max.y.max(point.y);
    }

    let (x_min, x_max) = (bbox_min.x as u32, bbox_max.x as u32);
    let (y_min, y_max) = (bbox_min.y as u32, bbox_max.y as u32);

    for x in x_min..(x_max + 1) {
        for y in y_min..(y_max + 1) {
            // compute baricentric coord
            let bar = barycentric(
                Point3::new(x as f32, y as f32, 0.0f32),
                v[0].gl_position,
                v[1].gl_position,
                v[2].gl_position,
            );
            // interpolate z value
            let z = bar.x * v[0].gl_position.z + bar.y * v[1].gl_position.z + bar.z * v[2].gl_position.z;
            let depth = &mut zbuffer[x as usize + y as usize * WIDTH as usize];

            // check if fragment inside triangle
            // and depth test
            if bar.x >= 0.0 && bar.y >= 0.0 && bar.z >= 0.0 && z > *depth {
                // interpolate z value
                let gl_fragcolor = shader.fragment(v.clone(), bar);

                if let Some(rgb) = gl_fragcolor {
                    // do not discard this fragment
                    // update color buffer and depth buffer
                    image.set(x, y, rgb);
                    *depth = z;
                }
           }
        }
    }
}



pub fn ortho_mat(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32, image_width: u32, image_height: u32) -> Matrix4<f32> {
    Matrix4::new(
        image_width as f32 / 2.0, 0.0, 0.0, 0.0,
        0.0, image_height as f32 / 2.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        image_width as f32 / 2.0, image_height as f32 /2.0, 1.0, 1.0
    )
    * ortho(left, right, bottom, top, near, far)
}

pub fn perspective_mat(fov: f32, near: f32, far: f32, image_width: u32, image_height: u32) -> Matrix4<f32> {
    let height = (fov / 2.0).tan() * near.abs() * 2.0;
    let aspect_ratio = image_width as f32 / image_height as f32;
    let width = height * aspect_ratio;

    Matrix4::new(
        WIDTH as f32 / width, 0.0, 0.0, 0.0,
        0.0, image_height as f32 / height, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        WIDTH as f32 / 2.0, image_width as f32 / 2.0, 0.0, 1.0
    ) * Matrix4::new(
        near,
        0.0,
        0.0,
        0.0,
        0.0,
        near,
        0.0,
        0.0,
        0.0,
        0.0,
        near + far,
        1.0,
        0.0,
        0.0,
        -near * far,
        0.0,
    )
}

pub fn look_at(camera_pos: Point3<f32>, focal_pos: Point3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    let w = (camera_pos - focal_pos).normalize();
    let u = up.cross(w).normalize();
    let v = w.cross(u);

    Matrix4::new(
        u.x, u.y, u.z, 0.0,
        v.x, v.y, v.z, 0.0,
        w.x, w.y, w.z, 0.0,
        camera_pos.x, camera_pos.y, camera_pos.z, 1.0,
    )
}

pub fn model_mat(translate: Vector3<f32>, scale: Vector3<f32>) -> Matrix4<f32> {
    Matrix4::new(
        scale[0],
        0.0,
        0.0,
        0.0,
        0.0,
        scale[1],
        0.0,
        0.0,
        0.0,
        0.0,
        scale[2],
        0.0,
        translate[0],
        translate[1],
        translate[2],
        1.0,
    )
}

