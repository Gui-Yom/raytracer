use cgmath::InnerSpace;
use rand::Rng;

use crate::camera::Ray;
use crate::geom::Hit;
use crate::Vec3;

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
        let reflected: Vec3 = reflection(&ray.direction, &hit.normal);
        let scattered = Ray {
            origin: hit.p,
            direction: reflected,
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

fn reflection(i: &Vec3, n: &Vec3) -> Vec3 {
    i - n * i.dot(*n) * 2.0
}
