use std::any::Any;
use std::rc::Rc;

use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

use crate::material::Material;
use crate::world::{Hit, Object};

pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub material: Rc<dyn Material>,
}

impl Object for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let v1v0 = self.v1 - self.v0;
        let v2v0 = self.v2 - self.v0;
        let rov0 = ray.origin - self.v0;

        let n = v1v0.cross(v2v0);
        let q = rov0.cross(ray.direction);
        let d: f32 = 1.0 / ray.direction.dot(n);
        let u = d * v2v0.dot(-q);
        let v = d * v1v0.dot(q);
        let t = d * rov0.dot(-n);

        if u < 0. || v < 0. || (u + v) > 1. || t < t_min || t > t_max {
            None
        } else {
            Some(Hit {
                t,
                p: ray.at_distance(t),
                normal: (-n).normalized(),
                material: self.material.clone(),
            })
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}