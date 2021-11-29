use std::{ops};
use crate::{random_f64_range, random_f64};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y:f64, z:f64) -> Vec3{
        Vec3{
            x, y, z
        }
    }

    pub fn length_squared(&self) -> f64 {
        &self.x * &self.x + &self.y * &self.y + &self.z * &self.z
    }

    pub fn length(self) -> f64{
        (self.length_squared()).sqrt()
    }

    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(vec1: &Vec3, vec2: &Vec3) -> f64 {
        vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
    }

    pub fn random_range(max: f64, min:f64) -> Vec3{
        Vec3::new(random_f64_range(max, min), random_f64_range(max, min), random_f64_range(max, min))
    }

    pub fn random() -> Vec3{
        Vec3::new(random_f64(), random_f64(), random_f64())
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1., 1.);
            if p.length_squared() >= 1. {
                continue
            }
            return p
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3{
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(&in_unit_sphere, &normal) > 0.{
            return in_unit_sphere
        }
        -in_unit_sphere
    }

    pub fn random_in_unit_disk() -> Vec3{
        loop {
            let p = Vec3::new(random_f64_range(-1., 1.), random_f64_range(-1., 1.), 0.);
            if p.length_squared() >= 1. {
                continue
            }
            return p;
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s  
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3{
        v - 2. * Vec3::dot(&v, &n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3{
        let cos_theta = f64::min(Vec3::dot(&-uv, &n), 1.);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -((1. - r_out_perp.length_squared()).abs()).sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub fn cross(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
        Vec3{
            x: vec1.y * vec2.z - vec1.z * vec2.y,
            y: vec1.z * vec2.x - vec1.x * vec2.z,
            z: vec1.x * vec2.y - vec1.y * vec2.x
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3{
            x: self.x + other.x, 
            y: self.y + other.y, 
            z: self.z + other.z
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3{
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    }

    
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, divider: f64) -> Vec3{
        Vec3{
            x: self.x / divider,
            y: self.y / divider,
            z: self.z / divider
        }
    }
}

impl ops::Neg for Vec3{
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3{
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3{
        self + -other
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3){
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3{
        Vec3{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}