extern crate nalgebra_glm as glm;

use nalgebra_glm::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn point(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}

pub struct HitRecord {
    t: f32,
    p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: glm::vec3(0.0, 0.0, 0.0),
            normal: glm::vec3(0.0, 0.0, 0.0),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut has_hit = false;
        let mut temp_record = HitRecord::new();
        let mut closest = t_max;
        for hittable in self.list.iter() {
            if hittable.hit(r, t_min, closest, &mut temp_record) {
                has_hit = true;
                closest = temp_record.t;
                record.t = temp_record.t;
                record.p = temp_record.p;
                record.normal = temp_record.normal;
            }
        }
        has_hit
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a: f32 = glm::length2(&r.dir);
        let b: f32 = glm::dot(&oc, &r.dir);
        let c: f32 = glm::length2(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                record.t = temp;
                record.p = r.point(temp);
                record.normal = (record.p - self.center) / self.radius;
            } else {
                temp = (-b + discriminant.sqrt()) / a;
                if temp > t_min && temp < t_max {
                    record.t = temp;
                    record.p = r.point(temp);
                    record.normal = (record.p - self.center) / self.radius;
                }
            }
            true
        } else {
            false
        }
    }
}