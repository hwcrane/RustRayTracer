use rand::Rng;

pub fn random_f64_range(max: f64, min: f64) -> f64{
    let mut rng = rand::thread_rng();
    min + (max - min) * rng.gen::<f64>()
}

pub fn random_f64() -> f64{
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}