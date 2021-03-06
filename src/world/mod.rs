use std::any::Any;
use std::rc::Rc;

use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

pub use plane::*;
pub use scene::*;
pub use sphere::*;
pub use triangle::*;

use crate::material::Material;

mod scene;
mod sphere;
mod plane;
mod triangle;

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
    fn as_any(&mut self) -> &mut dyn Any;
}

pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub front: bool,
}
