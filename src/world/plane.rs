use std::any::Any;
use std::rc::Rc;

use ultraviolet::geometry::Plane as UPlane;
use ultraviolet::geometry::Ray;

use crate::material::Material;
use crate::world::{Hit, Object};

pub struct Plane {
    pub uplane: UPlane,
    pub material: Rc<dyn Material>,
}

impl Object for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let a = ray.direction.dot(self.uplane.normal);
        let d = -(ray.origin.dot(self.uplane.normal) + self.uplane.bias) / a;
        if a > 0.0 || d < t_min || d > t_max {
            None
        } else {
            Some(Hit {
                t: d,
                p: ray.at_distance(d),
                normal: self.uplane.normal,
                material: self.material.clone(),
                front: true
            })
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}