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
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                let p = ray.at_distance(temp);
                return Option::Some(Hit {
                    t: temp,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone(),
                });
            } else {
                temp = (-b + discriminant.sqrt()) / a;
                if temp > t_min && temp < t_max {
                    let p = ray.at_distance(temp);
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

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}