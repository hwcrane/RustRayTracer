use crate::Ray;
use crate::hittable::HitRecord;
use crate::Vec3;

#[derive(Copy, Clone)]
pub enum Material{
    Lambertian {albedo: Vec3},
    Metal {albedo: Vec3}
}

impl Material{
    pub fn scatter (self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>{
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
                if scatter_direction.near_zero(){
                    scatter_direction = rec.normal
                }

                let scattered = Ray::new(rec.point, scatter_direction);
                let attenuation = albedo;
                Some((scattered, attenuation))
            }
            Material::Metal { albedo } => {
                let reflected = Vec3::reflect(r_in.direction.unit(), rec.normal);
                let scattered = Ray::new(rec.point, reflected);
                let attenuation = albedo;
                if Vec3::dot(&scattered.direction, &rec.normal) > 0.{
                    return Some((scattered, attenuation))
                }
                None
            }
        }
    }
}