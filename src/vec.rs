use std::{ops::DivAssign, process::Output};


pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3f
{
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self  {
            x, y, z
        }
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl DivAssign<f64> for Vec3f {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}