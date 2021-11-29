use crate::Ray;
use crate::hittable::HitRecord;
use crate::Vec3;
use crate::random::random_f64;
#[derive(Copy, Clone)]
pub enum Material{
    Lambertian {albedo: Vec3},
    Metal {albedo: Vec3, fuz: f64},
    Dielectric {index_of_refraction: f64}
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
            Material::Metal { albedo, fuz } => {
                let reflected = Vec3::reflect(r_in.direction.unit(), rec.normal);
                let scattered = Ray::new(rec.point, reflected + fuz * Vec3::random_in_unit_sphere());
                let attenuation = albedo;
                if Vec3::dot(&scattered.direction, &rec.normal) > 0.{
                    return Some((scattered, attenuation))
                }
                None
            }
            Material::Dielectric {index_of_refraction} => {
                let attenuation = Vec3::new(1., 1., 1.);
                let refreaction_ratio = match rec.font_face {
                    true => 1. / index_of_refraction,
                    false => index_of_refraction
                };
                let unit_direction = r_in.direction.unit();

                let cos_theta = f64::min(Vec3::dot(&-unit_direction, &rec.normal), 1.);
                let sin_theta = (1. - cos_theta*cos_theta).sqrt();

                let cannot_reflect = refreaction_ratio * sin_theta > 1.;
                let direction: Vec3;

                if cannot_reflect || reflectance(cos_theta, refreaction_ratio) > random_f64() {
                    direction = Vec3::reflect(unit_direction, rec.normal);
                } else {
                    direction = Vec3::refract(unit_direction, rec.normal, refreaction_ratio);
                }

                let scattered = Ray::new(rec.point, direction);
                
                Some((scattered, attenuation))
            }
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64{
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0*r0;
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}