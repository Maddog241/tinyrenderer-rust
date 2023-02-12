use cgmath::{Point3, Vector3, Point2, Matrix4, EuclideanSpace, InnerSpace, SquareMatrix, Matrix};
use image::{DynamicImage, Rgb};

use crate::{vertex::Vertex, mygl::texture_2d, myimage::MyImage};

pub trait Shader {
    fn vertex(&self, local_coord: Point3<f32>, tex_coord: Point2<f32>, normal: Vector3<f32>) -> Vertex;
    fn fragment(&self, v: Vec<Vertex>, bar: Vector3<f32>) -> Option<Rgb<u8>>;
}

pub struct MyShader {
    pub model_matrix: Matrix4<f32>,
    pub view_matrix: Matrix4<f32>,
    pub projection_matrix: Matrix4<f32>,
    pub texture: DynamicImage,
    pub normal_map: Option<DynamicImage>,
    pub camera_pos: Point3<f32>,
    pub light_pos: Point3<f32>,
    pub light_color: Vector3<f32>,
    pub shadowbuffer: MyImage,
    pub world_to_sm: Matrix4<f32>,
}

impl Shader for MyShader {
    fn vertex(&self, local_coord: Point3<f32>, tex_coord: Point2<f32>, normal: Vector3<f32>) -> Vertex {
        let raster_coord = self.projection_matrix * self.view_matrix * self.model_matrix * local_coord.to_vec().extend(1.0);
        Vertex{
            gl_position: Point3::from_homogeneous(raster_coord),
            tex_coord,
            normal,
            a_pos: local_coord,
        }
    }

    fn fragment(&self, v: Vec<Vertex>, bar: Vector3<f32>) -> Option<Rgb<u8>> {
        // interpolate tex coordinates
        let tex_coord = bar.x * v[0].tex_coord
            + (bar.y * v[1].tex_coord).to_vec()
            + (bar.z * v[2].tex_coord).to_vec();
        let color = texture_2d(&self.texture, tex_coord);
        let a_pos = bar.x * v[0].a_pos + (bar.y * v[1].a_pos).to_vec() + (bar.z * v[2].a_pos).to_vec();

        // lambert's law

        let normal = match &self.normal_map {
            None => { bar.x * v[0].normal + bar.y * v[1].normal + bar.z * v[2].normal }
            Some(normal_map) => {
                texture_2d(normal_map, tex_coord).normalize()
            }
        };

        // diffuse 
        let world_normal = (self.model_matrix.invert().unwrap().transpose() * normal.extend(0.0)).truncate().normalize();
        let frag_to_light = (self.light_pos.to_vec() - (self.model_matrix * a_pos.to_vec().extend(1.0)).truncate()).normalize();
        let diffuse = world_normal.dot(frag_to_light).max(0.0) * color;

        // ambient
        let ambient = Vector3::new(0.05, 0.05, 0.05);

        // specular
        let frag_to_camera = (self.camera_pos.to_vec() - (self.model_matrix * a_pos.to_vec().extend(1.0)).truncate()).normalize();
        let half_vec = ((frag_to_light + frag_to_camera) / 2.0).normalize();
        let specular = 0.1 * self.light_color * half_vec.dot(world_normal).powf(100.0f32);

        let intensity = ambient + diffuse + specular;

        // lookup shadow map
        let mut unblocked = false;
        // from world coordinate to shadow map coordinate
        let sm_coord = self.world_to_sm * self.model_matrix * a_pos.to_vec().extend(1.0);
        let (sm_x, sm_y) = (sm_coord.x as u32, sm_coord.y as u32);
        let depth = self.shadowbuffer.get(sm_x, sm_y).0[0];
        if sm_coord.z.abs()+0.01 > depth as f32 / 255.0 { unblocked = true; }

        let k: f32 = if unblocked { 1.0 } else { 0.3 };
        
        Some(Rgb([
            (k * intensity.x.clamp(0.0, 1.0) * 255.999) as u8,
            (k * intensity.y.clamp(0.0, 1.0) * 255.999) as u8,
            (k * intensity.z.clamp(0.0, 1.0) * 255.999) as u8,
        ]))
    }
}

pub struct ShadowShader {
    pub model_matrix: Matrix4<f32>,
    pub view_matrix: Matrix4<f32>,
    pub projection_matrix: Matrix4<f32>,
}

impl Shader for ShadowShader {
    fn vertex(&self, local_coord: Point3<f32>, tex_coord: Point2<f32>, normal: Vector3<f32>) -> Vertex {
        let raster_coord = self.projection_matrix * self.view_matrix * self.model_matrix * local_coord.to_vec().extend(1.0);
        Vertex{
            gl_position: Point3::from_homogeneous(raster_coord),
            tex_coord,
            normal,
            a_pos: local_coord,
        }
    }

    fn fragment(&self, v: Vec<Vertex>, bar: Vector3<f32>) -> Option<Rgb<u8>> {
        let z = bar.dot(Vector3::new(v[0].gl_position.z, v[1].gl_position.z, v[2].gl_position.z));
        let k = z.abs() * 255.0;

        Some(Rgb([k as u8, k as u8, k as u8]))
    }
}