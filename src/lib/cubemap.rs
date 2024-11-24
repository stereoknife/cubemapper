use std::io;
use std::io::Write;
use std::path::Path;
use image::RgbImage;
use crate::lib::cubeuv::CubeUV;
use crate::lib::face::Face;
use crate::lib::{random_vector_pointing_to, Vec3};

pub struct CubeMap {
    pub up: RgbImage,
    pub down: RgbImage,
    pub left: RgbImage,
    pub right: RgbImage,
    pub front: RgbImage,
    pub back: RgbImage
}

impl CubeMap {
    pub fn new(size: u32) -> CubeMap {
        let up = RgbImage::new(size, size);
        let down = RgbImage::new(size, size);
        let left = RgbImage::new(size, size);
        let right = RgbImage::new(size, size);
        let front = RgbImage::new(size, size);
        let back = RgbImage::new(size, size);

        CubeMap { up, down, left, right, front, back }
    }

    pub fn open<P>(up: P, down: P, left: P, right: P, front: P, back: P) -> CubeMap
        where P: AsRef<Path>{
        let up = image::open(up).unwrap().to_rgb8();
        let down = image::open(down).unwrap().to_rgb8();
        let left = image::open(left).unwrap().to_rgb8();
        let right = image::open(right).unwrap().to_rgb8();
        let front = image::open(front).unwrap().to_rgb8();
        let back = image::open(back).unwrap().to_rgb8();

        CubeMap { up, down, left, right, front, back }
    }

    pub fn sample(&self, uv: CubeUV) -> &image::Rgb<u8> {
        let (x, y) = uv.to_px(self.up.height());
        match uv.face {
            Face::Right => self.right.get_pixel(x, y),
            Face::Left => self.left.get_pixel(x, y),
            Face::Up => self.up.get_pixel(x, y),
            Face::Down => self.down.get_pixel(x, y),
            Face::Front => self.front.get_pixel(x, y),
            Face::Back => self.back.get_pixel(x, y),
        }
    }

    pub fn paint(&mut self, uv: CubeUV, r: u8, g: u8, b: u8) {
        let (x, y) = uv.to_px(self.up.height());
        match uv.face {
            Face::Right => self.right.put_pixel(x, y, [r, g, b].into()),
            Face::Left => self.left.put_pixel(x, y, [r, g, b].into()),
            Face::Up => self.up.put_pixel(x, y, [r, g, b].into()),
            Face::Down => self.down.put_pixel(x, y, [r, g, b].into()),
            Face::Front => self.front.put_pixel(x, y, [r, g, b].into()),
            Face::Back => self.back.put_pixel(x, y, [r, g, b].into()),
        }
    }

    pub fn diffuse_to(&self, out_map: &mut CubeMap, samples: u32) {
        println!("Difusing top map...");
        self.diffuse_face_to(&mut out_map.up, Face::Up, samples);
        println!("Difusing bottom map...");
        self.diffuse_face_to(&mut out_map.down, Face::Down, samples);
        println!("Difusing left map...");
        self.diffuse_face_to(&mut out_map.left, Face::Left, samples);
        println!("Difusing right map...");
        self.diffuse_face_to(&mut out_map.right, Face::Right, samples);
        println!("Difusing front map...");
        self.diffuse_face_to(&mut out_map.front, Face::Front, samples);
        println!("Difusing back map...");
        self.diffuse_face_to(&mut out_map.back, Face::Back, samples);
    }

    pub fn diffuse_face_to(&self, out_image: &mut RgbImage, face: Face, samples: u32) {
        // Get 5% of samples, for logging purposes only
        let fivepc = out_image.width() / 20;

        // Iterate through columns
        for x in 0..out_image.width() {
            if x % fivepc == 0 {
                print!(".");
                let _ = io::stdout().flush();
            };
            // Iterate through rows
            for y in 0..out_image.height() {

                // Get the uv coord of the pixel and translate into cubemap vector
                let uv = CubeUV::from_px(face, x, y, out_image.width());
                let vec: Vec3 = uv.into();

                // Initialize a colour variable, this is 32 bits because it
                // needs to be able to hold enough 8 bit samples
                let mut colour: [u32; 3] = [0,0,0];

                // For every sample get a random vector in the same hemisphere as this one
                // Sample the colour at the random vector and accumulate it
                for i in 0..samples {
                    let s_vec = random_vector_pointing_to(vec);
                    let s_uv = s_vec.into();
                    let c = self.sample(s_uv).0;
                    let dot = Vec3::dot(vec, s_vec);
                    colour[0] += (c[0] as f32 * dot) as u32;
                    colour[1] += (c[1] as f32 * dot) as u32;
                    colour[2] += (c[2] as f32 * dot) as u32;
                }

                // Divide the accumulated colour by the total samples to get the average
                colour[0] /= samples;
                colour[1] /= samples;
                colour[2] /= samples;

                // Store it in the current pixel
                out_image.put_pixel(x, y, [colour[0] as u8, colour[1] as u8, colour[2] as u8].into())
            }
        }
        println!("done")
    }

    pub fn paint_hemisphere_to(&self, out_map: &mut CubeMap, face: Face, samples: u32) {
        let uv = CubeUV::from_px(face, 128, 128, out_map.up.width());
        let vec: Vec3 = uv.into();

        for i in 0..samples {
            let s_vec = random_vector_pointing_to(vec);
            let s_uv = s_vec.into();
            out_map.paint(s_uv, 255, 0, 0);
        }
    }

    pub fn save(&self) {
        self.up.save("./top.png").unwrap();
        self.down.save("./bottom.png").unwrap();
        self.left.save("./left.png").unwrap();
        self.right.save("./right.png").unwrap();
        self.front.save("./front.png").unwrap();
        self.back.save("./back.png").unwrap();
    }
}