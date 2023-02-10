use cgmath::{Point2, Point3, Vector3};

#[derive(Clone)]
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
