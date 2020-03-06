use std::rc::Rc;

use cgmath::InnerSpace;

use crate::camera::Ray;
use crate::material::{Lambertian, Material};
use crate::Vec3;

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>
}

impl Object for Scene {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest_hit = Option::None;
        let mut closest = t_max;
        for object in self.objects.iter() {
            let hit = object.hit(ray, t_min, closest);
            if hit.is_some() {
                let hit = hit.unwrap();
                closest = hit.t;
                closest_hit = Option::Some(hit);
            }
        }
        closest_hit
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a: f32 = ray.direction.dot(ray.direction);
        let b: f32 = oc.dot(ray.direction);
        let c: f32 = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                let p = ray.point(temp);
                return Option::Some(Hit {
                    p,
                    t: temp,
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone(),
                });
            } else {
                temp = (-b + discriminant.sqrt()) / a;
                if temp > t_min && temp < t_max {
                    let p = ray.point(temp);
                    return Option::Some(Hit {
                        p,
                        t: temp,
                        normal: (p - self.center) / self.radius,
                        material: self.material.clone(),
                    });
                }
            }
        }
        Option::None
    }
}