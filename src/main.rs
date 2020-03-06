extern crate cgmath;
extern crate image;
extern crate pbr;
extern crate rand;

use std::env;
use std::f32::MAX;
use std::rc::Rc;

use cgmath::{ElementWise, InnerSpace, Vector3};
use image::{ImageBuffer, Rgb};
use pbr::ProgressBar;
use rand::Rng;

use crate::camera::{Camera, Ray};
use crate::geom::{Object, Scene, Sphere};
use crate::material::{Lambertian, Metal};

mod geom;
mod camera;
mod material;

type Vec3 = Vector3<f32>;

fn main() {
    println!("GuiYom's raytracer v0.1.0");
    // Parsing arguments
    let args: Vec<String> = env::args().collect();
    let width: u32 = args[1].parse().unwrap();
    let height: u32 = args[2].parse().unwrap();
    println!("Writing to {}", args[3]);

    // Progress bar stuff
    let count = width * height;
    let update_rate: u32 = 1000;
    let mut total: u32 = 0;
    let mut pb = ProgressBar::new(count as u64);
    pb.format("[=> ]");
    pb.message("Casting rays : ");

    let mut image = ImageBuffer::new(width, height);
    let camera = Camera {
        origin: Vec3::new(0.0, 0.0, 5.0),
        width,
        height,
        fov: 90.0,
    };
    // Number of anti-aliasing samples
    let aa: u32 = 16;

    // World objects
    let world = Scene {
        objects: vec![
            Box::new(Sphere {
                center: Vec3::new(-5.0, 0.0, -4.0),
                radius: 5.0,
                material: Rc::new(Lambertian { albedo: Vec3::new(0.8, 0.2, 0.3) }),
            }),
            Box::new(Sphere {
                center: Vec3::new(6.0, 1.0, -7.0),
                radius: 3.0,
                material: Rc::new(Metal { albedo: Vec3::new(0.0, 0.0, 0.8) }),
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, 112.0, -1.0),
                radius: 100.0,
                material: Rc::new(Lambertian { albedo: Vec3::new(0.0, 0.5, 0.5) }),
            })
        ]
    };

    let mut rng = rand::thread_rng();
    // Cast rays for each pixels
    for j in 0..height {
        for i in 0..width {
            let mut col: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            // Monte carlo multi sampling for anti-aliasing
            for _ in 0..aa {
                col += color(&camera.cast_ray(i as f32 + rng.gen::<f32>(), j as f32 + rng.gen::<f32>()), &world, 1, 0);
            }
            col /= aa as f32;
            // Gamma correction
            col = Vec3::new(col.x.powf(0.4545), col.y.powf(0.4545), col.z.powf(0.4545));
            // Update image and progress bar
            image.put_pixel(i, j, Rgb([(col.x * 255.0) as u8, (col.y * 255.0) as u8, (col.z * 255.0) as u8]));
            if total % update_rate == 0 {
                pb.set(total as u64);
            }
            total += 1;
        }
    }
    image = image::imageops::flip_vertical(&image);
    image.save(&args[3]).unwrap();
    pb.finish_println("OK !")
}

// Compute color for a given ray
fn color(ray: &Ray, world: &dyn Object, maxdepth: u32, depth: u32) -> Vec3 {
    let hit = world.hit(ray, 0.001, MAX);
    if hit.is_some() {
        let hit = hit.unwrap();
        let (scatter, attenuation, scattered) = hit.material.scatter(ray, &hit);
        if depth < maxdepth && scatter {
            attenuation.mul_element_wise(color(&scattered, world, maxdepth, depth + 1))
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let t: f32 = 0.5 * (ray.direction.y + 1.0);
        Vec3::new(0.3, 0.5, 1.0) * t + Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)
    }
}
