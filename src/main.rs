mod model;
mod mygl;
mod framebuffer;
mod shader;
mod vertex;

use std::f64::consts::PI;
// use std::process::exit;

use cgmath::{Point2, Point3, SquareMatrix, Vector3};
use model::Model;
use mygl::{look_at, model_mat, ortho_mat, perspective_mat, triangle, Float};
use framebuffer::FrameBuffer;
use shader::{MyShader, Shader, ShadowShader};

#[allow(dead_code)]
const WHITE: Vector3<Float> = Vector3::new(1.0, 1.0, 1.0);
#[allow(dead_code)]
const RED: Vector3<Float> = Vector3::new(1.0, 0.0, 0.0);
#[allow(dead_code)]
const GREEN: Vector3<Float> = Vector3::new(0.0, 1.0, 0.0);
#[allow(dead_code)]
const BLUE: Vector3<Float> = Vector3::new(0.0, 0.0, 1.0);

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const SM_WIDTH: u32 = 2000;
const SM_HEIGHT: u32 = 2000;

const NEAR: Float= -1.0;
const FAR: Float= -60.0;

const FOV: Float= PI / 4.0;

// const CAMERA_POS: Point3<Float> = Point3::new(30.0, 0.0, -30.0);
const CAMERA_POS: Point3<Float> = Point3::new(0.0, 0.0, 0.0);
const FOCAL_POS: Point3<Float> = Point3::new(0.0, 0.0, -30.0);
const CAMERA_UP: Vector3<Float> = Vector3::new(0.0, 1.0, 0.0);

// const LIGHT_POS: Point3<Float> = Point3::new(-10.0, 5.0, -5.0);
const LIGHT_POS: Point3<Float> = Point3::new(10.0, 10.0, 0.0);

fn main() {
    let mut framebuffer = FrameBuffer::new(WIDTH, HEIGHT);

    let model = Model::new("./obj/diablo3/diablo3_pose.obj");

    //
    // first pass
    //
    let mut shadow_zbuffer = vec![Float::MIN; SM_WIDTH as usize * SM_HEIGHT as usize];
    let mut shadowbuffer = FrameBuffer::new(SM_WIDTH, SM_HEIGHT);

    let model_matrix = model_mat(
        Vector3::new(0.0, 0.0, -30.0),
        Vector3::new(10.0, 10.0, 10.0),
    );

    let view_matrix = look_at(LIGHT_POS, FOCAL_POS, CAMERA_UP).invert().unwrap();
    let projection_matrix = ortho_mat(-50.0, 50.0, -50.0, 50.0, NEAR, FAR, SM_WIDTH, SM_HEIGHT);

    // transformation matrix from world coordinate to shadow map coordinate
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
            let tex_coord = Point2::new(
                model.texcoords[2 * model.texcoords_indices[i][j]],
                model.texcoords[2 * model.texcoords_indices[i][j] + 1],
            );
            let normal = model.normals[model.normal_indices[i][j]];

            v.push(shadow_shader.vertex(local_coord.cast().unwrap(), tex_coord.cast().unwrap(), normal.cast().unwrap()));
        }

        // rasterizer
        triangle(&mut shadowbuffer, &mut shadow_zbuffer, &shadow_shader, v);
    }

    //
    // second pass
    //
    let mut zbuffer = vec![Float::MIN; WIDTH as usize * HEIGHT as usize];

    let tex = image::open("./obj/diablo3/diablo3_pose_diffuse.tga")
        .expect("failed to open the texture file");
    let normal_map =
        image::open("./obj/diablo3/diablo3_pose_nm.tga").expect("failed to open the texture file");

    let view_matrix = look_at(CAMERA_POS, FOCAL_POS, CAMERA_UP).invert().unwrap();
    let projection_matrix = perspective_mat(FOV, NEAR, FAR, WIDTH, HEIGHT);

    let shader = MyShader {
        model_matrix,
        view_matrix,
        projection_matrix,
        texture: tex,
        // normal_map: None,
        normal_map: Some(normal_map),
        camera_pos: CAMERA_POS,
        light_pos: LIGHT_POS,
        light_intensity: Vector3::new(5.0, 5.0, 5.0),
        shadowbuffer,
        world_to_sm,
    };

    for i in 0..model.faces.len() {
        let mut v = Vec::new();

        // vertex shader
        for j in 0..3 {
            let local_coord = model.verts[model.faces[i][j]];
            let tex_coord = Point2::new(
                model.texcoords[2 * model.texcoords_indices[i][j]],
                model.texcoords[2 * model.texcoords_indices[i][j] + 1],
            );
            let normal = model.normals[model.normal_indices[i][j]];

            v.push(shader.vertex(local_coord.cast().unwrap(), tex_coord.cast().unwrap(), normal.cast().unwrap()));
        }

        // rasterizer
        triangle(&mut framebuffer, &mut zbuffer, &shader, v);
    }

    framebuffer.write_img("output.png");
}
