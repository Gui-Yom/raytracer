use cgmath::InnerSpace;

use crate::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub width: u32,
    pub height: u32,
    pub fov: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            width: 200,
            height: 100,
            fov: 90.0,
        }
    }

    pub fn cast_ray(&self, x: f32, y: f32) -> Ray {
        let fov_adjusted = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = self.width as f32 / self.height as f32;
        let u = (x / self.width as f32 * 2.0 - 1.0) * aspect_ratio * fov_adjusted;
        let v = (1.0 - y / self.height as f32 * 2.0) * fov_adjusted;
        Ray {
            origin: self.origin,
            direction: Vec3::new(u, v, -1.0).normalize(),
        }
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
