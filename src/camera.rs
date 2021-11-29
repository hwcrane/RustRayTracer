use crate::Vec3;
use crate::Ray;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3
}

impl Camera{
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera{
        let theta = vfov * std::f64::consts::PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = ( look_from - look_at ).unit();
        let u = Vec3::cross(&vup, &w).unit();
        let v = Vec3::cross(&w, &u);

        let origin = look_from;

        let horizontal = focus_dist * viewport_width * u;
        let vertical =  focus_dist * viewport_height * v;
        let lower_left_corner =
            origin - horizontal/2. - vertical/2. - focus_dist * w;
        
        let lens_radius = aperture / 2.;
        Camera{
            origin, horizontal, vertical, lower_left_corner, lens_radius, u, v
        }
    }

    pub fn get_ray(&self, s:f64, t:f64) -> Ray{
        let rd = self.lens_radius * Vec3::random_in_unit_disk();

        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}