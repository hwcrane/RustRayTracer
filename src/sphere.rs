use crate::Vec3;
use crate::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Sphere{
    pub fn new(center: Vec3, radius: f64) -> Sphere{
        Sphere{
            center,
            radius
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Finds Roots
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {return None}
        let sqrtd = discriminant.sqrt();

        // Finds the nearest root that lies in desired range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root{
                return None
            }
        }

        // Updates Hit record
        let mut rec = HitRecord::default();
        rec.t = root;
        rec.point = r.at(rec.t);

        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_font_face(&r, outward_normal);

        Some(rec)
    }
}
