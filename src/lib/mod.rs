
mod face;
mod vec3;
mod cubeuv;
mod cubemap;

pub use face::*;
pub use vec3::*;
pub use cubeuv::*;
pub use cubemap::*;

use std::cmp::{Ordering, PartialOrd};
use std::f64::consts::PI;

use std::ops::Add;
use image::RgbImage;
use rand::{Rng, rngs::OsRng};
use rand::distributions::Uniform;

pub fn random_vector() -> Vec3 {
    let die = Uniform::new(-1.0, 1.0);
    Vec3::new(
        OsRng.sample(die) as f32,
        OsRng.sample(die) as f32,
        OsRng.sample(die) as f32
    )
}

// Uniform sampling algorithm from https://corysimon.github.io/articles/uniformdistn-on-sphere/
pub fn random_vector_sphere() -> Vec3 {
    let die = Uniform::new(0.0, 1.0);

    let theta: f64 = 2.0 * PI * OsRng.sample(die);
    let phi: f64 = (1.0 - 2.0 * OsRng.sample(die)).acos();
    let x: f64 = phi.sin() * theta.cos();
    let y: f64 = phi.sin() * theta.sin();
    let z: f64 = phi.cos();

    Vec3::new(x as f32, y as f32, z as f32)
}

pub fn random_vector_pointing_to(v: Vec3) -> Vec3 {
    let mut o = random_vector_sphere();
    while Vec3::dot(o, v) < 0.0 {
        o = random_vector_sphere()
    }
    o
}

pub fn vector_in_hemisphere(a: Vec3, b: Vec3) -> bool {
    Vec3::dot(a, b) >= 0f32
}