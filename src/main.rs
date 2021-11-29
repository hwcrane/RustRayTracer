mod camera;
mod hittable;
mod hittable_list;
mod material;
mod random;
mod ray;
mod sphere;
mod vec3;

extern crate image;
extern crate indicatif;
extern crate rayon;

use camera::Camera;
use hittable_list::HittableList;
use image::{ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use material::Material;
use random::{random_f64, random_f64_range};
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1000;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 1000;
const MAX_RAY_DEPTH: u32 = 50;

fn random_scene() -> HittableList {
    let mut world = HittableList::default();
    
    let ground_mat = Material::Lambertian {albedo: Vec3::new(0.5, 0.5, 0.5)};
    world.add(Box::new(Sphere::new(Vec3::new(0., -1000., 0.), 1000., ground_mat)));

    for a in -11..11 {
        for b in -11..11{
            let mat = random_f64();
            let center = Vec3::new(a as f64 + 0.9 * random_f64(), 0.2, b as f64+ 0.9 * random_f64());

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {

                if mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_mat = Material::Lambertian {albedo};
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

    let mat2 = Material::Lambertian {albedo: Vec3::new(0.4, 0.2, 0.1)};
    world.add(Box::new(Sphere::new(Vec3::new(-4., 1., 0.), 1., mat2)));

    let mat3 = Material::Metal {albedo: Vec3::new(0.7, 0.6, 0.5),fuz: 0.};
    world.add(Box::new(Sphere::new(Vec3::new(4., 1., 0.), 1., mat3)));

    world
}

fn main() {
    // Progress Bar
    let bar = ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH) as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] {wide_bar:.cyan/blue} {percent}% ({eta})",
            )
            .progress_chars("##-"),
    );

    // World

    let world = random_scene();
    // Camera

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = 10.0;
    let apeture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        apeture,
        dist_to_focus,
    );
    // Render

    let mut imgbuf = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    
    let mut pixels = vec![Rgb([0 as u8, 0 as u8, 0 as u8]); (IMAGE_WIDTH * IMAGE_HEIGHT) as usize];

    (pixels).par_iter_mut().enumerate().for_each(|(i, pixel)| {
        let x = i % IMAGE_WIDTH as usize;
        let y = IMAGE_HEIGHT as usize - (i / IMAGE_WIDTH as usize);

        let mut colour = Vec3::new(0., 0., 0.);

        for _i in 0..SAMPLES_PER_PIXEL {
            let u = (x as f64 + random_f64()) / (IMAGE_WIDTH) as f64;
            let v = (y as f64 + random_f64()) / (IMAGE_HEIGHT) as f64;
            let r = camera.get_ray(u, v);
            colour = colour + r.ray_colour(&world, MAX_RAY_DEPTH);
        }
        *pixel = make_colour(colour, SAMPLES_PER_PIXEL);
        if i % 10 == 0 {
            bar.inc(10)
        }
    });

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = pixels[(x + IMAGE_WIDTH * y) as usize];
    } 

    bar.finish();
    imgbuf.save("out.png").unwrap();
}

fn make_colour(colour: Vec3, samples_per_pixel: u32) -> Rgb<u8> {
    let mut r = colour.x;
    let mut g = colour.y;
    let mut b = colour.z;

    let scale = 1. / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    Rgb([
        (clamp(r, 0., 1.) * 255.) as u8,
        (clamp(g, 0., 1.) * 255.) as u8,
        (clamp(b, 0., 1.) * 255.) as u8,
    ])
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
