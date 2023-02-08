mod model;
mod myimage;
mod vertex;
mod mygl;

use std::f32::consts::PI;

use cgmath::{Point2, Point3, Vector3, Vector4, SquareMatrix};
use image::Rgb;
use model::Model;
use mygl::{perspective_mat, look_at, model_mat, triangle};
use myimage::MyImage;
use vertex::Vertex;

#[allow(dead_code)]
const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
#[allow(dead_code)]
const RED: Rgb<u8> = Rgb([255, 0, 0]);
#[allow(dead_code)]
const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
#[allow(dead_code)]
const BLUE: Rgb<u8> = Rgb([0, 0, 255]);

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const NEAR: f32 = -1.0;
const FAR: f32 = -50.0;

const FOV: f32 = PI / 4.0;
const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

const CAMERA_POS: Point3<f32> = Point3::new(10.0, 10.0, -10.0);
const FOCAL_POS: Point3<f32> = Point3::new(0.0, 0.0, -30.0);
const CAMERA_UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

/// convert the screen coordinate (x ranging [-1.0, 1.0]) to raster coordinate (x ranging [0, WIDTH-1])
fn screen_to_raster(fov: f32, near: f32, screen_coord: Point3<f32>) -> Point3<f32> {
    let half_height = (fov / 2.0).tan() * near.abs();
    let half_width = half_height * ASPECT_RATIO;

    let x = (screen_coord.x + half_width) / (2.0 * half_width) * WIDTH as f32;
    let y = (screen_coord.y + half_height) / (2.0 * half_height) * HEIGHT as f32;

    Point3::new(x, y, screen_coord.z)
}

fn main() {
    let mut image = MyImage::new(WIDTH, HEIGHT);

    let model = Model::new("./obj/african_head.obj");
    let mut zbuffer = vec![f32::MIN; WIDTH as usize*HEIGHT as usize];

    let tex =
        image::open("./obj/african_head_diffuse.tga").expect("failed to open the texture file");
    
    let mvp = perspective_mat(NEAR, FAR)
        * look_at(CAMERA_POS, FOCAL_POS, CAMERA_UP).invert().unwrap()
        * model_mat(
            Vector3::new(0.0, 0.0, -30.0),
            Vector3::new(10.0, 10.0, 10.0),
        );

    for i in 0..model.faces.len() {
        let mut v = Vec::new();

        // vertex shader
        for j in 0..3 {
            let local = model.verts[model.faces[i][j]];
            let screen_coord = Point3::from_homogeneous(mvp * Vector4::new(local.x, local.y, local.z, 1.0));
            let raster_coord = screen_to_raster(FOV, NEAR, screen_coord);

            let tex_coord = Point2::new(model.texcoords[2 * model.texcoords_indices[i][j]], model.texcoords[2 * model.texcoords_indices[i][j] + 1]);
            let normal = model.normals[model.normal_indices[i][j]];

            v.push(Vertex::new(raster_coord, tex_coord, normal));
        }

        // rasterizer and fragment shader
        triangle(&mut image, &mut zbuffer, &tex, v);
    }

    image.write_img("output.png");
}
