use crate::Vec3;
use crate::hittable::{Hittable};
use crate::HittableList;

use std::f64::INFINITY;
pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray{
        Ray{
            origin, direction
        }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn ray_colour(&self, world: &HittableList, depth: u32) -> Vec3{
        
        if depth <= 0{
            return Vec3::new(0., 0., 0.);
        }

        if let Some(rec) = world.hit(self, 0.001, INFINITY){
            if let Some((scattered, attenuation)) = rec.mat_ptr.scatter(self, &rec){
                return attenuation * scattered.ray_colour(world, depth - 1);
            }
            return Vec3::new(0., 0., 0.)
        }
        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.);
        (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
    }

    
}