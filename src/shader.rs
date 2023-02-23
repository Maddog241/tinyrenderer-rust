use cgmath::{EuclideanSpace, InnerSpace, Matrix, Matrix4, Point2, Point3, SquareMatrix, Vector3};
use image::DynamicImage;

use crate::{mygl::{texture_2d, Float}, framebuffer::FrameBuffer, vertex::Vertex};

pub trait Shader {
    fn vertex(
        &self,
        local_coord: Point3<Float>,
        tex_coord: Point2<Float>,
        normal: Vector3<Float>,
    ) -> Vertex;
    fn fragment(&self, v: Vec<Vertex>, bar: Vector3<Float>) -> Option<Vector3<Float>>;
}

pub struct MyShader {
    pub model_matrix: Matrix4<Float>,
    pub view_matrix: Matrix4<Float>,
    pub projection_matrix: Matrix4<Float>,
    pub texture: DynamicImage,
    pub normal_map: Option<DynamicImage>,
    pub camera_pos: Point3<Float>,
    pub light_pos: Point3<Float>,
    pub light_color: Vector3<Float>,
    pub shadowbuffer: FrameBuffer,
    pub world_to_sm: Matrix4<Float>,
}

impl Shader for MyShader {
    fn vertex(
        &self,
        local_coord: Point3<Float>,
        tex_coord: Point2<Float>,
        normal: Vector3<Float>,
    ) -> Vertex {
        let raster_coord = self.projection_matrix
            * self.view_matrix
            * self.model_matrix
            * local_coord.to_vec().extend(1.0);
        Vertex {
            gl_position: Point3::from_homogeneous(raster_coord),
            tex_coord,
            normal,
            a_pos: local_coord,
        }
    }

    fn fragment(&self, v: Vec<Vertex>, bar: Vector3<Float>) -> Option<Vector3<Float>> {
        // interpolate tex coordinates
        let tex_coord = bar.x * v[0].tex_coord
            + (bar.y * v[1].tex_coord).to_vec()
            + (bar.z * v[2].tex_coord).to_vec();
        let color = texture_2d(&self.texture, tex_coord);
        let a_pos =
            bar.x * v[0].a_pos + (bar.y * v[1].a_pos).to_vec() + (bar.z * v[2].a_pos).to_vec();

        // lambert's law

        let normal = match &self.normal_map {
            None => bar.x * v[0].normal + bar.y * v[1].normal + bar.z * v[2].normal,
            Some(normal_map) => texture_2d(normal_map, tex_coord)*2.0 - Vector3::new(1.0, 1.0, 1.0),
        };

        // diffuse
        let world_normal = (self.model_matrix.invert().unwrap().transpose() * normal.extend(0.0))
            .truncate()
            .normalize();
        let frag_to_light = (self.light_pos.to_vec()
            - (self.model_matrix * a_pos.to_vec().extend(1.0)).truncate())
        .normalize();
        let diffuse = world_normal.dot(frag_to_light).max(0.0) * color;

        // ambient
        let ambient = Vector3::new(0.05, 0.05, 0.05);

        // specular
        let frag_to_camera = (self.camera_pos.to_vec()
            - (self.model_matrix * a_pos.to_vec().extend(1.0)).truncate())
        .normalize();
        let half_vec = ((frag_to_light + frag_to_camera) / 2.0).normalize();
        let specular = 0.1 * self.light_color * half_vec.dot(world_normal).powf(100.0);

        let intensity = ambient + diffuse + specular;

        // lookup shadow map
        let mut unblocked = false;
        // from world coordinate to shadow map coordinate
        let sm_coord = self.world_to_sm * self.model_matrix * a_pos.to_vec().extend(1.0);
        let (sm_x, sm_y) = (sm_coord.x as u32, sm_coord.y as u32);
        let depth = self.shadowbuffer.get(sm_x, sm_y)[0];
        if sm_coord.z.abs() < depth + 0.01 {
            unblocked = true;
        }
        // println!("{} vs {}", sm_coord.z, depth as Float/ 255.0);

        let k: Float= if unblocked { 1.0 } else { 0.3 };

        Some(Vector3::new(
            k * intensity.x.clamp(0.0, 1.0),
            k * intensity.y.clamp(0.0, 1.0),
            k * intensity.z.clamp(0.0, 1.0),
        ))
    }
}

pub struct ShadowShader {
    pub model_matrix: Matrix4<Float>,
    pub view_matrix: Matrix4<Float>,
    pub projection_matrix: Matrix4<Float>,
}

impl Shader for ShadowShader {
    fn vertex(
        &self,
        local_coord: Point3<Float>,
        tex_coord: Point2<Float>,
        normal: Vector3<Float>,
    ) -> Vertex {
        let raster_coord = self.projection_matrix
            * self.view_matrix
            * self.model_matrix
            * local_coord.to_vec().extend(1.0);
        Vertex {
            gl_position: Point3::from_homogeneous(raster_coord),
            tex_coord,
            normal,
            a_pos: local_coord,
        }
    }

    fn fragment(&self, v: Vec<Vertex>, bar: Vector3<Float>) -> Option<Vector3<Float>> {
        let z = bar.dot(Vector3::new(
            v[0].gl_position.z,
            v[1].gl_position.z,
            v[2].gl_position.z,
        ));

        Some(Vector3::new(z.abs(), z.abs(), z.abs()))
    }
}
