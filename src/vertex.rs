use cgmath::{Point2, Point3, Vector3};

use crate::mygl::Float;

#[derive(Clone)]
pub struct Vertex {
    pub gl_position: Point3<Float>,
    pub tex_coord: Point2<Float>,
    pub normal: Vector3<Float>,
    pub a_pos: Point3<Float>,
}
