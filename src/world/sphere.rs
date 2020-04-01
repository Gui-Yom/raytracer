use std::any::Any;
use std::rc::Rc;

use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

use crate::material::Material;
use crate::world::{Hit, Object};

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
        if discriminant > 0. {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                return self.compute_hit(ray, temp);
            } else {
                temp = (-b + discriminant.sqrt()) / a;
                if temp > t_min && temp < t_max {
                    return self.compute_hit(ray, temp);
                }
            }
        }
        None
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Sphere {
    fn compute_hit(&self, ray: &Ray, temp: f32) -> Option<Hit> {
        let p = ray.at_distance(temp);
        let normal = (p - self.center) / self.radius;
        let front = ray.direction.dot(normal) < 0.;
        Some(Hit {
            t: temp,
            p,
            normal: if front { normal } else { -normal },
            material: self.material.clone(),
            front,
        })
    }
}
