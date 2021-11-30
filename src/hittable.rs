use crate::Vec3;
use crate::Ray;
use crate::material::Material;
use crate::aabb::Aabb;

#[derive(Copy, Clone)]
pub struct HitRecord{
    pub point: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub font_face: bool
}

impl HitRecord{
    pub fn new(point: Vec3, mat_ptr: Material, t: f64, u: f64, v: f64, outward_normal: Vec3, r: &Ray) -> HitRecord{
        let mut h = HitRecord{
            point, mat_ptr, t, u, v, normal: Vec3::new(0., 0., 0.), font_face: true
        };
        h.set_font_face(r, outward_normal);
        h
    }

    pub fn set_font_face(&mut self, r: &Ray, outward_normal: Vec3){
        self.font_face = Vec3::dot(&r.direction, &outward_normal) < 0.;
        self.normal = match self.font_face {
            true => outward_normal,
            false =>-outward_normal
        }
    }
}

pub trait Hittable: Sync + Send{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box (&self) -> Option<Aabb>;
}

