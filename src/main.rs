mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod camera;
mod radom;
mod material;

extern crate image;
extern crate indicatif;
extern crate rayon;

use material::Material;
use radom::{random_f64, random_f64_range};
use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use ray::Ray;
use vec3::Vec3;
use indicatif::{ProgressBar, ProgressStyle};
use image::{ImageBuffer, Rgb};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 2000;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_RAY_DEPTH: u32 = 50;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

fn main() {
    // Progress Bar
    let bar = ProgressBar::new((IMAGE_HEIGHT* IMAGE_WIDTH) as u64);
    bar.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] {wide_bar:.cyan/blue} {percent}% ({eta})")
    .progress_chars("##-"));

    // World

    let mut world = HittableList::default();

    let material_ground = Material::Lambertian {albedo: Vec3::new(0.2, 0.2, 0.2)};
    let material_center = Material::Lambertian {albedo: Vec3::new(0.2, 0.9, 0.3)};
    let material_left = Material::Metal {albedo: Vec3::new(0.8, 0.8, 0.8)};
    let material_right = Material::Metal {albedo: Vec3::new(0.8, 0.6, 0.2)};

    world.add(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100., material_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(0., 0., -2.), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-1., 0., -1.), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Vec3::new(1., 0., -1.), 0.5, material_right)));



    // Camera

    let camera = Camera::new(ASPECT_RATIO, VIEWPORT_HEIGHT, VIEWPORT_WIDTH, FOCAL_LENGTH);
    
    // Render

    let mut imgbuf = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut colour = Vec3::new(0., 0., 0.);

        for _i in 0..SAMPLES_PER_PIXEL {
            let u = ((IMAGE_WIDTH - x) as f64 + random_f64()) / (IMAGE_WIDTH - 1) as f64;
            let v = ((IMAGE_HEIGHT - y) as f64 + random_f64()) / (IMAGE_HEIGHT - 1) as f64;
            let r = camera.get_ray(u, v);
            colour += r.ray_colour(&world, MAX_RAY_DEPTH);
        }
        *pixel = make_colour(colour, SAMPLES_PER_PIXEL);
        
        bar.inc(1)
        
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
        (clamp(b, 0., 1.) * 255.) as u8
        ])

}

fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x < min {return min}
    if x > max {return max}
    x
}