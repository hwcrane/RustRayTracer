use crate::Vec3;
use crate::Ray;
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material
}

impl Sphere{
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere{
        Sphere{
            center,
            radius,
            material
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

        let point = r.at(root);
        let material = self.material;
        let t = root;
        let outward_normal = (point - self.center) / self.radius;
        
        let rec = HitRecord::new(point, material, t, outward_normal, &r);

        Some(rec)
    }
}
