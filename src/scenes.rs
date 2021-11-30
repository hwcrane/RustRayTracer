use crate::Vec3;
use crate::HittableList;
use crate::Material;
use crate::sphere::Sphere;
use crate::random::*;
use crate::texture::{Texture};

pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();
    
    let ground_mat = Material::Lambertian {texture: Texture::Checker{odd: Vec3::new(1., 1., 1.), even: Vec3::new(0., 0., 0.)}};
    world.add(Box::new(Sphere::new(Vec3::new(0., -1000., 0.), 1000., ground_mat)));

    for a in -11..11 {
        for b in -11..11{
            let mat = random_f64();
            let center = Vec3::new(a as f64 + 0.9 * random_f64(), 0.2, b as f64+ 0.9 * random_f64());

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {

                if mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_mat = Material::Lambertian{texture: Texture::SolidColour{colour_value:albedo}};
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_mat)))
                } else if mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.);
                    let fuz = random_f64_range(0., 0.5);
                    let sphere_mat = Material::Metal {albedo, fuz};
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_mat)))
                } else {
                    let sphere_mat = Material::Dielectric {index_of_refraction: 1.5};
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_mat)))
                }
            }
        }
    }

    let mat1 = Material::Dielectric {index_of_refraction: 1.5};
    world.add(Box::new(Sphere::new(Vec3::new(0., 1., 0.), 1., mat1)));

    let mat2 = Material::Lambertian {texture: Texture::SolidColour{colour_value: Vec3::new(0.4, 0.2, 0.1)}};
    world.add(Box::new(Sphere::new(Vec3::new(-4., 1., 0.), 1., mat2)));

    let mat3 = Material::Metal {albedo: Vec3::new(0.7, 0.6, 0.5),fuz: 0.};
    world.add(Box::new(Sphere::new(Vec3::new(4., 1., 0.), 1., mat3)));

    world
}