use std::any::Any;

use ultraviolet::geometry::Ray;

use crate::world::{Hit, Object};

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

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}