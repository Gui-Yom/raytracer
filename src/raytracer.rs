use std::f32::MAX;
use std::ops::Div;

use rand::Rng;
use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

use crate::camera::Camera;
use crate::world::Object;

/// The software raytracer
pub struct Raytracer {
    pub camera: Camera,
    pub ss: Box<dyn SamplingStrategy>,
    pub max_bounces: u32,
}

impl Raytracer {
    /// Streams the rendered pixels to the closure f
    pub fn render_stream<F>(&mut self, world: &dyn Object, mut f: F) where F: FnMut(u32, u32, (f32, f32, f32)) {
        // Cast rays for each pixels
        for j in 0..self.camera.height {
            for i in 0..self.camera.width {
                let mut color: Vec3 = self.ss.sample(&self.camera, i, j, world, self.max_bounces);
                // Gamma correction
                color.apply(|v: f32| v.powf(0.4545));
                // TODO move this to a post treatment step

                // Update image and progress bar
                f(i, j, (color.x, color.y, color.z))
            }
        }
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

/// Compute the color for a given ray
fn compute_color(ray: &Ray, world: &dyn Object, depth: u32) -> Vec3 {
    let hit = world.hit(ray, 0.001, MAX);
    if hit.is_some() {
        let hit = hit.unwrap();
        let (scatter, attenuation, scattered) = hit.material.scatter(ray, &hit);
        if depth > 0 && scatter {
            attenuation * compute_color(&scattered, world, depth - 1)
        } else {
            Vec3::zero()
        }
    } else {
        let t: f32 = 0.5 * (ray.direction.y + 1.0);
        Vec3::new(0.3, 0.5, 1.0) * t + Vec3::one() * (1.0 - t)
    }
}

pub trait SamplingStrategy {
    fn sample(&mut self, camera: &Camera, x: u32, y: u32, world: &dyn Object, max_bounces: u32) -> Vec3;
}

pub struct BasicSS;

impl SamplingStrategy for BasicSS {
    fn sample(&mut self, camera: &Camera, x: u32, y: u32, world: &dyn Object, max_bounces: u32) -> Vec3 {
        compute_color(&camera.cast_ray(x as f32, y as f32), world, max_bounces)
    }
}

pub struct MontecarloSS {
    pub quant: u32,
    rng: rand::rngs::ThreadRng,
}

impl MontecarloSS {
    pub fn new(quant: u32) -> Self {
        MontecarloSS {
            quant,
            rng: rand::thread_rng(),
        }
    }
}

impl SamplingStrategy for MontecarloSS {
    fn sample(&mut self, camera: &Camera, x: u32, y: u32, world: &dyn Object, max_bounces: u32) -> Vec3 {
        let mut color = Vec3::zero();
        for _ in 0..self.quant {
            color += compute_color(
                &camera.cast_ray(x as f32 + self.rng.gen::<f32>(), y as f32 + self.rng.gen::<f32>()),
                world,
                max_bounces);
        }
        // avg division
        color.apply(|v: f32| v.div(self.quant as f32));
        color
    }
}

pub struct OversamplingSS {
    pub sqrt_quant: u32
}

impl OversamplingSS {
    pub fn new(sqrt_quant: u32) -> Self {
        OversamplingSS {
            sqrt_quant
        }
    }
}

impl SamplingStrategy for OversamplingSS {
    fn sample(&mut self, camera: &Camera, x: u32, y: u32, world: &dyn Object, max_bounces: u32) -> Vec3 {
        let mut color = Vec3::zero();
        for k in 0..self.sqrt_quant {
            for l in 0..self.sqrt_quant {
                color += compute_color(
                    &camera.cast_ray(x as f32 + k as f32 / self.sqrt_quant as f32,
                                     y as f32 + l as f32 / self.sqrt_quant as f32),
                    world,
                    max_bounces);
            }
        }
        // avg division
        color.apply(|v: f32| v.div((self.sqrt_quant * self.sqrt_quant) as f32));
        color
    }
}
