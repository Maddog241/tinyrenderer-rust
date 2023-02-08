use cgmath::{Point3, Point2, Vector3};

pub struct Vertex {
    pub position: Point3<f32>,
    pub tex_coord: Point2<f32>,
    pub normal: Vector3<f32>,
}

impl Vertex {
    pub fn new(position: Point3<f32>, tex_coord: Point2<f32>, normal: Vector3<f32>) -> Self {
        Self {
            position,
            tex_coord,
            normal,
        }
    }
}
