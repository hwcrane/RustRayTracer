use crate::hittable::{Hittable, HitRecord};
use crate::aabb::Aabb;
use crate::Ray;

pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>, 
    aabb: Aabb
}

impl Hittable for BvhNode{
    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.aabb)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(r, t_min, t_max){
            return None
        }

        let hit_left = self.left.hit(r, t_min, t_max);

        let hit_right = self.right.hit(r, t_min, if let Some(rec) = hit_left {rec.t} else {t_max});

        if let Some(rec) = hit_left {
            return Some(rec);
        } else if let Some(rec) = hit_right{
            return Some(rec)
        } else {
            return None;
        }
    }
}