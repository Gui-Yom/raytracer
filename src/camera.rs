use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    fov_adjusted: f32,
    aspect_ratio: f32,
}

impl Camera {
    pub fn new(origin: Vec3, width: u32, height: u32, fov: f32) -> Camera {
        Camera {
            origin,
            width,
            height,
            fov,
            fov_adjusted: Camera::adjust_fov(fov),
            aspect_ratio: width as f32 / height as f32,
        }
    }

    pub fn fov(&mut self, fov: f32) {
        self.fov = fov;
        self.fov_adjusted = Camera::adjust_fov(fov);
    }

    fn adjust_fov(fov: f32) -> f32 {
        (fov.to_radians() / 2.0).tan()
    }

    pub fn cast_ray(&self, x: f32, y: f32) -> Ray {
        let u = (x / self.width as f32 * 2.0 - 1.0) * self.aspect_ratio * self.fov_adjusted;
        let v = (1.0 - y / self.height as f32 * 2.0) * self.fov_adjusted;
        let mut direction = Vec3::new(u, v, -1.0);
        direction.normalize();
        Ray {
            origin: self.origin,
            direction,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(Vec3::zero(), 800, 600, 90.0)
    }
}
