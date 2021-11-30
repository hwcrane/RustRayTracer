use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::Ray;
use crate::Vec3;

#[derive(Default)]
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.list.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for hittable in &self.list {
            if let Some(rec) = hittable.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec)
            }
        }
        hit_record
    }

    fn bounding_box(&self) -> Option<Aabb> {
        if self.list.is_empty() {
            return None;
        }
        let mut first_box = true;
        let mut output_box= Aabb::default();
        for i in 0..self.list.len() {
            if let Some(aabb) = self.list[i].bounding_box() {
                output_box = if first_box {aabb} else {surrounding_box(output_box, aabb)};
                first_box = false;
            } else {
                return None;
            }
        }
        Some(output_box)
    }
}

fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb{
    let small = Vec3::new(
        f64::min(box0.minimum.x, box1.minimum.x),
        f64::min(box0.minimum.y, box1.minimum.y),
        f64::min(box0.minimum.z, box1.minimum.z),
    );

    let big = Vec3::new(
        f64::max(box0.maximum.x, box1.maximum.x),
        f64::max(box0.maximum.y, box1.maximum.y),
        f64::max(box0.maximum.z, box1.maximum.z),
    );

    Aabb::new(small, big)
}
