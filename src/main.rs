extern crate image;
extern crate nalgebra_glm as glm;
extern crate pbr;

use std::env;

use glm::Vec3;
use image::{ImageBuffer, Rgb};
use pbr::ProgressBar;

struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    fn point(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}

fn main() {
    println!("GuiYom's raytracer v0.1.0");
    let args: Vec<String> = env::args().collect();
    let width: u32 = args[1].parse().unwrap();
    let height: u32 = args[2].parse().unwrap();
    println!("Writing to {}", args[3]);
    let count = width * height;
    let mut pb = ProgressBar::new(count as u64);
    pb.format("[=>-]");
    pb.message("Casting rays : ");

    let mut buf = ImageBuffer::new(width, height);
    let lower_left_corner = glm::vec3(-2.0, -1.0, -1.0);
    let horizontal = glm::vec3(4.0, 0.0, 0.0);
    let vertical = glm::vec3(0.0, 2.0, 0.0);
    let origin = glm::vec3(0.0, 0.0, 0.0);
    for (i, j, pixel) in buf.enumerate_pixels_mut() {
        let r = Ray {
            origin,
            dir: lower_left_corner + horizontal * (i as f32 / width as f32) + vertical * (j as f32 / height as f32),
        };
        let color = color(&r);
        *pixel = Rgb([(color.data[0] * 255.0) as u8, (color.data[1] * 255.0) as u8, (color.data[2] * 255.0) as u8]);
        pb.inc();
    }
    buf.save(&args[3]).unwrap();
    pb.finish_println("OK !")
}

fn color(r: &Ray) -> Vec3 {
    let t: f32 = 0.5 * (r.dir.data[1] + 1.0);
    glm::vec3(0.5, 0.7, 1.0) * t + glm::vec3(1.0, 1.0, 1.0) * (1.0 - t)
}
