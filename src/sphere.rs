use crate::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl  Sphere{
    pub fn new(center: Vec3, radius: f64) -> Sphere{
        Sphere{
            center,
            radius
        }
    }
}