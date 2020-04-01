use std::f32::consts::PI;

use rand::distributions::Uniform;
use rand::prelude::ThreadRng;
use rand::Rng;
use ultraviolet::geometry::Ray;
use ultraviolet::vec::Vec3;

use crate::world::Hit;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> (bool, Vec3, Ray);
}

pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> (bool, Vec3, Ray) {
        let mut target = hit.normal + random_unit_vector();
        let scattered = Ray {
            origin: hit.p,
            direction: target,
        };
        let attenuation = self.albedo;
        (true, attenuation, scattered)
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzzyness: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> (bool, Vec3, Ray) {
        let scattered = Ray {
            origin: hit.p,
            direction: ray.direction.reflected(hit.normal) + random_unit_vector() * self.fuzzyness,
        };
        let attenuation = self.albedo;
        (scattered.direction.dot(hit.normal) > 0.0, attenuation, scattered)
    }
}

pub struct Dielectric {
    pub refract_idx: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> (bool, Vec3, Ray) {
        let coeff: f32 = if hit.front { 1. / self.refract_idx } else { self.refract_idx };
        let cos_theta: f32 = 1.0f32.min(hit.normal.dot(-ray.direction));
        let sin_theta: f32 = (1. - cos_theta * cos_theta).sqrt();
        if coeff * sin_theta > 1. {
            return (true, Vec3::one(), Ray { origin: hit.p, direction: hit.normal.reflected(ray.direction) });
        }
        let mut r0 = (1. - self.refract_idx) / (1. + self.refract_idx);
        r0 = r0 * r0;
        // Schlick's approximation
        let reflect_prob = r0 + (1. - r0) * (1. - cos_theta).powi(5);
        unsafe {
            if RNG.is_none() {
                RNG = Some(rand::thread_rng());
            }
            if RNG.unwrap().gen::<f32>() < reflect_prob {
                return (true, Vec3::one(), Ray { origin: hit.p, direction: hit.normal.reflected(ray.direction) });
            }
        }
        return (true, Vec3::one(), Ray { origin: hit.p, direction: ray.direction.refracted(hit.normal, coeff) });
    }
}

static mut RNG: Option<ThreadRng> = None;

/*
fn random_unit_vector() -> Vec3 {
    unsafe {
        if RNG.is_none() {
            RNG = Some(rand::thread_rng());
        }
        let phi: f32 = RNG.unwrap().sample(Uniform::new(0., 2. * PI));
        let theta: f32 = RNG.unwrap().sample(Uniform::new(0., PI));
        Vec3::new(theta.sin() * phi.cos(), theta.sin() * phi.sin(), theta.cos())
    }
}
*/

fn random_unit_vector() -> Vec3 {
    unsafe {
        if RNG.is_none() {
            RNG = Some(rand::thread_rng());
        }
        let a: f32 = RNG.unwrap().sample(Uniform::new(0., 2. * PI));
        let z: f32 = RNG.unwrap().sample(Uniform::new(-1., 1.));
        let r = (1. - z * z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }
}
