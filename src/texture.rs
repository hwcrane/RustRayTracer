use crate::Vec3;

#[derive(Copy, Clone)]
pub enum Texture{
    SolidColour {colour_value: Vec3},
    Checker {odd: Vec3, even:Vec3},
}



impl Texture{
    pub fn value(self, mut _u: f64, mut _v: f64, p: Vec3) -> Vec3{
        match self {
            Texture::SolidColour {colour_value} => {
                return colour_value
            }
            Texture::Checker {odd, even} => {
                let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
                if sines < 0. {odd} else {even}
            }
        }
    }
}
