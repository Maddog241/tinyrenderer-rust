
use cgmath::{ortho, InnerSpace, Matrix4, Point2, Point3, Vector3};
use image::DynamicImage;

use crate::{
    framebuffer::FrameBuffer,
    shader::Shader,
    vertex::Vertex,
    WIDTH,
};

pub fn barycentric(p: Point3<Float>, a: Point3<Float>, b: Point3<Float>, c: Point3<Float>) -> Vector3<Float> {
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

pub fn texture_2d(texture: &DynamicImage, tex_coord: Point2<Float>) -> Vector3<Float> {
    let (tex_width, tex_height) = (texture.width() as Float, texture.height() as Float);
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

    Vector3::new(r as Float / 255.0, g as Float / 255.0, b as Float / 255.0)
}

pub fn triangle(
    framebuffer: &mut FrameBuffer,
    zbuffer: &mut Vec<Float>,
    shader: &impl Shader,
    v: Vec<Vertex>,
) {
    assert!(v.len() == 3);

    let mut bbox_min: Point2<Float> = Point2::new(
        framebuffer.width as Float - 1.0,
        framebuffer.height as Float - 1.0,
    );
    let mut bbox_max: Point2<Float> = Point2::new(0.0, 0.0);

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
                Point3::new(x as Float, y as Float, 0.0),
                v[0].gl_position,
                v[1].gl_position,
                v[2].gl_position,
            );
            // interpolate z value
            let z = bar.x * v[0].gl_position.z
                + bar.y * v[1].gl_position.z
                + bar.z * v[2].gl_position.z;
            let depth = &mut zbuffer[x as usize + y as usize * WIDTH as usize];

            // check if fragment inside triangle
            // and depth test
            if bar.x >= 0.0 && bar.y >= 0.0 && bar.z >= 0.0 && z > *depth {
                // interpolate z value
                let gl_fragcolor = shader.fragment(v.clone(), bar);

                if let Some(color) = gl_fragcolor {
                    // do not discard this fragment
                    // update color buffer and depth buffer
                    framebuffer.set(x, y, color);
                    *depth = z;
                }
            }
        }
    }
}

pub fn ortho_mat(
    left: Float,
    right: Float,
    bottom: Float,
    top: Float,
    near: Float,
    far: Float,
    image_width: u32,
    image_height: u32,
) -> Matrix4<Float> {
    Matrix4::new(
        image_width as Float / 2.0,
        0.0,
        0.0,
        0.0,
        0.0,
        image_height as Float / 2.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.5,
        0.0,
        image_width as Float / 2.0,
        image_height as Float / 2.0,
        0.5,
        1.0,
    ) * ortho(left, right, bottom, top, near, far)
}

pub fn perspective_mat(
    fov: Float,
    near: Float,
    far: Float,
    image_width: u32,
    image_height: u32,
) -> Matrix4<Float> {
    let height = (fov / 2.0).tan() * near.abs() * 2.0;
    let aspect_ratio = image_width as Float / image_height as Float;
    let width = height * aspect_ratio;

    Matrix4::new(
        WIDTH as Float / width,
        0.0,
        0.0,
        0.0,
        0.0,
        image_height as Float / height,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        WIDTH as Float / 2.0,
        image_width as Float / 2.0,
        0.0,
        1.0,
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

pub fn look_at(camera_pos: Point3<Float>, focal_pos: Point3<Float>, up: Vector3<Float>) -> Matrix4<Float> {
    let w = (camera_pos - focal_pos).normalize();
    let u = up.cross(w).normalize();
    let v = w.cross(u);

    Matrix4::new(
        u.x,
        u.y,
        u.z,
        0.0,
        v.x,
        v.y,
        v.z,
        0.0,
        w.x,
        w.y,
        w.z,
        0.0,
        camera_pos.x,
        camera_pos.y,
        camera_pos.z,
        1.0,
    )
}

pub fn model_mat(translate: Vector3<Float>, scale: Vector3<Float>) -> Matrix4<Float> {
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

pub type Float = f64;