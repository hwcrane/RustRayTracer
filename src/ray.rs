use crate::Vec3;
use crate::Sphere;
pub struct Ray{
    origin: Vec3,
    direction: Vec3,
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

    pub fn ray_colour(&self) -> Vec3{
        let t = self.hit_sphere( Sphere::new(Vec3::new(0., 0., -1.), 0.5));
        if t > 0.{
            let n = (self.at(t) - Vec3::new(0., 0., -1.)).unit();
            return 0.5 * Vec3::new(n.x + 1., n.y + 1., n.z + 1.)
        }
        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3{x: 1.0, y: 1.0, z: 1.0} + t * Vec3{x: 0.5, y: 0.7, z: 1.0}
    }

    fn hit_sphere(&self, sphere: Sphere) -> f64{
        let oc = self.origin - sphere.center;
        let a = Vec3::dot(self.direction, self.direction);
        let b = 2. * Vec3::dot(oc, self.direction);
        let c = Vec3::dot(oc, oc) - sphere.radius * sphere.radius;
        let descriminant = b * b - 4. * a * c;
        if descriminant < 0. {
            return -1.;
        }
        (-b - descriminant.sqrt()) / (2. * a)
    }
}