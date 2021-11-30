use crate::Ray;
use crate::Vec3;

#[derive(Default, Copy, Clone)]
pub struct Aabb {
    pub maximum: Vec3,
    pub minimum: Vec3
}

impl Aabb {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Aabb{
        Aabb{ minimum, maximum}
    }
    pub fn hit (&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let invd = 1./ r.direction[a];
            let mut t0 = (self.minimum[a] - r.origin[a]) * invd;
            let mut t1 = (self.maximum[a] - r.origin[a]) * invd;
            if invd < 0. {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }
            let t_min = f64::max(t0, t_min);
            let t_max = f64::min(t1, t_max);
            if t_max <= t_min{
                return false;
            }
        }    
        true
    }
}