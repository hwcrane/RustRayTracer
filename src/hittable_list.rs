use crate::hittable::{HitRecord, Hittable};
use crate::{Ray};

#[derive(Default)] 
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList{
    pub fn add(&mut self, hittable: Box<dyn Hittable>){
        self.list.push(hittable);
    }
}

impl Hittable for HittableList{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for hittable in &self.list{
            if let Some(rec) = hittable.hit(r, t_min, closest_so_far){
                closest_so_far = rec.t;
                hit_record = Some(rec)
            }
        }
        hit_record
    }
}