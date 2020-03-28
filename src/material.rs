use rand::Rng;
use ultraviolet::geometry::Ray;
use ultraviolet::vec::Vec3;

use crate::geom::Hit;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> (bool, Vec3, Ray);
}

pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> (bool, Vec3, Ray) {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray {
            origin: hit.p,
            direction: target - hit.p,
        };
        let attenuation = self.albedo;
        (true, attenuation, scattered)
    }
}

pub struct Metal {
    pub albedo: Vec3
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> (bool, Vec3, Ray) {
        let scattered = Ray {
            origin: hit.p,
            direction: ray.direction.reflected(hit.normal),
        };
        let attenuation = self.albedo;
        (scattered.direction.dot(hit.normal) > 0.0, attenuation, scattered)
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();
    while {
        p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        p.dot(p) >= 1.0
    } {}
    p
}
