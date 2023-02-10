use cgmath::{Point3, Vector3, Point2, Matrix4, EuclideanSpace, InnerSpace, SquareMatrix, Matrix};
use image::{DynamicImage, Rgb};

use crate::{vertex::Vertex, mygl::texture_2d};

pub struct Shader {
    model_matrix: Matrix4<f32>,
    view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
    texture: DynamicImage,
}

impl Shader {
    pub fn new(model_matrix: Matrix4<f32>, view_matrix: Matrix4<f32>, projection_matrix: Matrix4<f32>, texture: DynamicImage) -> Self {
        Shader {
            model_matrix,
            view_matrix,
            projection_matrix,
            texture,
        }
    }

    pub fn vertex(&self, local_coord: Point3<f32>, tex_coord: Point2<f32>, normal: Vector3<f32>) -> Vertex {
        let raster_coord = self.projection_matrix * self.view_matrix * self.model_matrix * local_coord.to_vec().extend(1.0);
        Vertex::new(Point3::from_homogeneous(raster_coord), tex_coord, normal)
    }

    pub fn fragment(&self, v: Vec<Vertex>, bar: Vector3<f32>) -> Option<Rgb<u8>> {
        // interpolate tex coordinates
        let tex_coord = bar.x * v[0].tex_coord
            + (bar.y * v[1].tex_coord).to_vec()
            + (bar.z * v[2].tex_coord).to_vec();
        let color = texture_2d(&self.texture, tex_coord);

        // lambert's law

        let normal = bar.x * v[0].normal + bar.y * v[1].normal + bar.z * v[2].normal;
        let world_normal = (self.model_matrix.invert().unwrap().transpose() * normal.extend(0.0)).truncate().normalize();
        let frag_to_light = Vector3::new(0.0, 0.0, 1.0);

        let cosine = world_normal.dot(frag_to_light).max(0.0);

        let intensity = color * cosine;
        
        Some(Rgb([
            (intensity.x * 255.0) as u8,
            (intensity.y * 255.0) as u8,
            (intensity.z * 255.0) as u8,
        ]))
    }
}