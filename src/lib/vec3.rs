use std::ops::{Add, AddAssign, Div, DivAssign};
use super::face::*;
use super::cubeuv::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 { Vec3{ x, y, z } }

    pub fn zero() -> Vec3 { Vec3::new(0.0,0.0,0.0) }

    pub fn dot(a: Self, b: Self) -> f32 { a.x * b.x + a.y * b.y + a.z * b.z }

    pub fn normalize(self) -> Self { self / Self::dot(self, self).sqrt() }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs
        )
    }
}


impl From<CubeUV> for Vec3 {
    // This code is based on the algorithm wikipedia page for cube mapping
    // and translated to Rust https://en.wikipedia.org/wiki/Cube_mapping
    // The reason for this is that translating between a vector and UV coordinates
    // in a texture is a solved problem and from what I understand the specific
    // implementation of this function is not the point of the assignment
    fn from(uv: CubeUV) -> Self {
        let uc = 2.0 * uv.u - 1.0;
        let vc = 2.0 * uv.v - 1.0;
        match uv.face {
            Face::Right => Vec3::new( 1.0,   vc,  -uc),
            Face::Left  => Vec3::new(-1.0,   vc,   uc),
            Face::Up    => Vec3::new(  uc,  1.0,  -vc),
            Face::Down  => Vec3::new(  uc, -1.0,   vc),
            Face::Front => Vec3::new(  uc,   vc,  1.0),
            Face::Back  => Vec3::new( -uc,   vc, -1.0),
        }
    }
}