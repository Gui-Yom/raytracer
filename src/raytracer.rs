use std::f32::MAX;

use rand::Rng;
use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

use crate::camera::Camera;
use crate::geom::{Object, Scene};

/// The software raytracer
/// Currently uses Monte-Carlo anti-aliasing
pub struct Raytracer {
    pub camera: Camera,
    pub scene: Scene,
    pub mcaa: u32,
    pub max_bounces: u32,
}

impl Raytracer {
    pub fn render_stream<F>(&self, mut f: F) where F: FnMut(u32, u32, [f32; 3]) {
        let mut rng = rand::thread_rng();
        // Cast rays for each pixels
        for j in 0..self.camera.height {
            for i in 0..self.camera.width {
                let mut color: Vec3 = Vec3::zero();
                // Monte carlo multi sampling for anti-aliasing
                for _ in 0..self.mcaa {
                    color += Raytracer::compute_color(&self.camera.cast_ray(i as f32 + rng.gen::<f32>(), j as f32 + rng.gen::<f32>()), &self.scene, self.max_bounces, 0);
                }
                color /= self.mcaa as f32;
                // Gamma correction
                color = Vec3::new(color.x.powf(0.4545), color.y.powf(0.4545), color.z.powf(0.4545));
                // Update image and progress bar
                f(i, j, [color.x, color.y, color.z])
            }
        }
    }

    /// Compute the color for a given ray
    fn compute_color(ray: &Ray, world: &dyn Object, maxdepth: u32, depth: u32) -> Vec3 {
        let hit = world.hit(ray, 0.001, MAX);
        if hit.is_some() {
            let hit = hit.unwrap();
            let (scatter, attenuation, scattered) = hit.material.scatter(ray, &hit);
            if depth < maxdepth && scatter {
                attenuation * Raytracer::compute_color(&scattered, world, maxdepth, depth + 1)
            } else {
                Vec3::zero()
            }
        } else {
            let t: f32 = 0.5 * (ray.direction.y + 1.0);
            Vec3::new(0.3, 0.5, 1.0) * t + Vec3::one() * (1.0 - t)
        }
    }
}
