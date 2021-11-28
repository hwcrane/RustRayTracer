use crate::Vec3;
use crate::Ray;

#[derive(Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub font_face: bool
}

impl HitRecord {
    pub fn set_font_face(&mut self, r: &Ray, outward_normal: Vec3){
        self.font_face = Vec3::dot(&r.direction, &outward_normal) < 0.;
        self.normal = match self.font_face {
            true => outward_normal,
            false =>-outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}