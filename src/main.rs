mod model;
mod myimage;
mod vertex;
mod mygl;
mod shader;

use std::f32::consts::PI;

use cgmath::{Point2, Point3, Vector3, SquareMatrix};
use model::Model;
use mygl::{perspective_mat, look_at, model_mat, triangle, ortho_mat};
use myimage::MyImage;
use shader::{Shader, ShadowShader, MyShader};

#[allow(dead_code)]
const WHITE: Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);
#[allow(dead_code)]
const RED: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
#[allow(dead_code)]
const GREEN: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
#[allow(dead_code)]
const BLUE: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const SM_WIDTH: u32 = 2000;
const SM_HEIGHT: u32 = 2000;

const NEAR: f32 = -1.0;
const FAR: f32 = -50.0;

const FOV: f32 = PI / 4.0;

// const CAMERA_POS: Point3<f32> = Point3::new(30.0, 0.0, -30.0);
const CAMERA_POS: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
const FOCAL_POS: Point3<f32> = Point3::new(0.0, 0.0, -30.0);
const CAMERA_UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

const LIGHT_POS: Point3<f32> = Point3::new(-10.0, 10.0, -10.0);


fn main() {
    let mut image = MyImage::new(WIDTH, HEIGHT);

    let model = Model::new("./obj/diablo3/diablo3_pose.obj");
    let mut shadow_zbuffer = vec![f32::MIN; SM_WIDTH as usize*SM_HEIGHT as usize];

    // first pass
    // it's not a must for the resolution to be (WIDTH, HEIGHT)
    let mut shadowbuffer = MyImage::new(SM_WIDTH, SM_HEIGHT);

    let model_matrix =  model_mat(
            Vector3::new(0.0, 0.0, -30.0),
            Vector3::new(10.0, 10.0, 10.0),
        );

    let view_matrix = look_at(LIGHT_POS, FOCAL_POS, CAMERA_UP).invert().unwrap();
    let projection_matrix = ortho_mat(-15.0, 15.0, -15.0, 15.0, NEAR, FAR, SM_WIDTH, SM_HEIGHT);
    // let projection_matrix = perspective_mat(FOV, NEAR, FAR, WIDTH, HEIGHT);

    let world_to_sm = projection_matrix * view_matrix;
    
    let shadow_shader = ShadowShader {
        model_matrix,
        view_matrix, 
        projection_matrix,
    };

    for i in 0..model.faces.len() {
        let mut v = Vec::new();

        // vertex shader
        for j in 0..3 {
            let local_coord = model.verts[model.faces[i][j]];
            let tex_coord = Point2::new(model.texcoords[2 * model.texcoords_indices[i][j]], model.texcoords[2 * model.texcoords_indices[i][j] + 1]);
            let normal = model.normals[model.normal_indices[i][j]];

            v.push(shadow_shader.vertex(local_coord, tex_coord, normal));
        }

        // rasterizer
        triangle(&mut shadowbuffer, &mut shadow_zbuffer, &shadow_shader, v);
    }

    // second pass
    let mut zbuffer = vec![f32::MIN; WIDTH as usize*HEIGHT as usize];

    let tex =
        image::open("./obj/diablo3/diablo3_pose_diffuse.tga").expect("failed to open the texture file");
    // let normal_map = 
    //     image::open("./obj/diablo3/diablo3_pose_nm.tga").expect("failed to open the texture file");
    
    
    let view_matrix = look_at(CAMERA_POS, FOCAL_POS, CAMERA_UP).invert().unwrap();
    let projection_matrix = perspective_mat(FOV, NEAR, FAR, WIDTH, HEIGHT);


    let shader = MyShader {
        model_matrix,
        view_matrix,
        projection_matrix,
        texture: tex,
        normal_map: None,
        // normal_map: Some(normal_map),
        camera_pos: CAMERA_POS,
        light_pos: LIGHT_POS,
        light_color: WHITE,
        shadowbuffer,
        world_to_sm,
    };

    for i in 0..model.faces.len() {
        let mut v = Vec::new();

        // vertex shader
        for j in 0..3 {
            let local_coord = model.verts[model.faces[i][j]];
            let tex_coord = Point2::new(model.texcoords[2 * model.texcoords_indices[i][j]], model.texcoords[2 * model.texcoords_indices[i][j] + 1]);
            let normal = model.normals[model.normal_indices[i][j]];

            v.push(shader.vertex(local_coord, tex_coord, normal));
        }

        // rasterizer
        triangle(&mut image, &mut zbuffer, &shader, v);
    }

    image.write_img("output.png");
}
