use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    fov_adjusted: f32,
    aspect_ratio: f32,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, lookat: Vec3, width: u32, height: u32, fov: f32) -> Camera {
        let mut camera = Camera {
            origin,
            lookat,
            vup: Vec3::new(0.0, -1.0, 0.0),
            width,
            height,
            fov,
            fov_adjusted: 0.0,
            aspect_ratio: 0.0,
            horizontal: Default::default(),
            vertical: Default::default(),
            lower_left: Default::default(),
        };
        camera.recalc();
        camera
    }

    pub fn fov(&mut self, fov: f32) {
        self.fov = fov;
        self.recalc();
    }

    pub fn recalc(&mut self) {
        self.aspect_ratio = self.width as f32 / self.height as f32;
        self.fov_adjusted = (self.fov.to_radians() / 2.0).tan();
        let w = (self.origin - self.lookat).normalized();
        let u = (self.vup.cross(w)).normalized();
        let v = w.cross(u);
        self.lower_left = self.origin - self.aspect_ratio * self.fov_adjusted * u - self.fov_adjusted * v - w;
        self.horizontal = u * 2. * self.fov_adjusted * self.aspect_ratio;
        self.vertical = v * 2. * self.fov_adjusted;
    }

    pub fn cast_ray(&self, x: f32, y: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + self.horizontal * (x / self.width as f32) + self.vertical * (y / self.height as f32) - self.origin,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(Vec3::zero(), Vec3::new(0.0, 0.0, -1.0), 800, 600, 90.0)
    }
}
