use cgmath::{Point2, Point3, Vector3};

#[derive(Clone)]
pub struct Vertex {
    pub gl_position: Point3<f32>,
    pub tex_coord: Point2<f32>,
    pub normal: Vector3<f32>,
    pub a_pos: Point3<f32>,
}