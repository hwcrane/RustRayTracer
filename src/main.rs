mod vec3;
mod ray;
mod sphere;
extern crate image;
extern crate indicatif;

use sphere::Sphere;
use ray::Ray;
use vec3::Vec3;
use indicatif::{ProgressBar, ProgressStyle};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 2000;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

fn main() {
    let bar = ProgressBar::new((IMAGE_HEIGHT / 10) as u64);
    bar.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] {wide_bar:.cyan/blue} {percent}% ({eta})")
    .progress_chars("##-"));

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal/2.0  - vertical/2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
    
    let mut imgbuf = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = (IMAGE_WIDTH - x) as f64 / (IMAGE_WIDTH - 1) as f64;
        let v = (IMAGE_HEIGHT - y) as f64 / (IMAGE_HEIGHT - 1) as f64;
        let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
        let colour = r.ray_colour();
        *pixel = image::Rgb([(colour.x * 255.0) as u8, (colour.y * 255.) as u8, (colour.z * 255.)as u8]);
        if y % 10 == 0 && x == 0 {
            bar.inc(1)
        }
    } 
    bar.finish();
    imgbuf.save("out.png").unwrap();
}
