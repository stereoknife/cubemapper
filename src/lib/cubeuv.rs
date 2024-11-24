use super::face::*;
use super::vec3::*;

#[derive(Copy, Clone, Debug)]
pub struct CubeUV {
    pub face: Face,
    pub u: f32,
    pub v: f32
}

impl CubeUV {
    pub fn new(face: Face, u: f32, v: f32) -> CubeUV {
        CubeUV{ face, u, v }
    }

    pub fn to_px(self, size: u32) -> (u32, u32) {
        ((self.u * (size - 1) as f32).floor() as u32, 511 - ((self.v * (size - 1) as f32).floor() as u32))
    }

    pub fn from_px(face: Face, x: u32, y: u32, size: u32) -> Self {
        Self::new(face, x as f32 / size as f32, 1.0 - (y as f32 / size as f32))
    }
}

impl From<Vec3> for CubeUV {
    // This code is based on the algorithm wikipedia page for cube mapping
    // and translated to Rust https://en.wikipedia.org/wiki/Cube_mapping
    // The reason for this is that translating between a vector and UV coordinates
    // in a texture is a solved problem and from what I understand the specific
    // implementation of this function is not the point of the assignment
    fn from(v: Vec3) -> Self {
        let abs_x = v.x.abs();
        let abs_y = v.y.abs();
        let abs_z = v.z.abs();

        let x_pos = v.x.is_sign_positive();
        let y_pos = v.y.is_sign_positive();
        let z_pos = v.z.is_sign_positive();

        let mut max_axis: f32 = 0.0;
        let mut uc: f32 = 0.0;
        let mut vc: f32 = 0.0;
        let mut face = Face::Right;

        if abs_x >= abs_y && abs_x >= abs_z {
            max_axis = abs_x;
            if x_pos {
                uc = -v.z;
                vc = v.y;
                face = Face::Right;
            }
            else {
                uc = v.z;
                vc = v.y;
                face = Face::Left;
            }
        }

        if abs_y >= abs_x && abs_y >= abs_z {
            max_axis = abs_y;
            if y_pos {
                uc = v.x;
                vc = -v.z;
                face = Face::Up;
            }
            else {
                uc = v.x;
                vc = v.z;
                face = Face::Down;
            }
        }

        if abs_z >= abs_x && abs_z >= abs_y {
            max_axis = abs_z;
            if z_pos {
                uc = v.x;
                vc = v.y;
                face = Face::Front;
            }
            else {
                uc = -v.x;
                vc = v.y;
                face = Face::Back;
            }
        }

        CubeUV::new(
            face,
            0.5 * (uc / max_axis + 1.0),
            0.5 * (vc / max_axis + 1.0)
        )
    }
}