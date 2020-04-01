use std::env;
use std::rc::Rc;
use std::time::Duration;

use image::{ImageBuffer, Rgb};
use pbr::ProgressBar;
use ultraviolet::geometry::Plane as UPlane;
use ultraviolet::Vec3;

use softrays::{Camera, MontecarloSS, OversamplingSS};
use softrays::material::{Dielectric, Lambertian, Metal};
use softrays::Raytracer;
use softrays::world::{Plane, Scene, Sphere};

fn main() {
    // Parsing arguments
    let args: Vec<String> = env::args().collect();
    let width: u32 = args[1].parse().unwrap();
    let height: u32 = args[2].parse().unwrap();
    println!("Writing to {}", args[3]);

    // Progress bar stuff
    let count = width * height;
    let mut pb = ProgressBar::new(count as u64);
    pb.format("[=> ]");
    pb.message("Casting rays : ");
    pb.set_max_refresh_rate(Some(Duration::from_millis(500)));

    let mut image = ImageBuffer::new(width, height);

    let scene = Scene {
        objects: vec![
            Box::new(Sphere {
                center: Vec3::new(-5.0, 0.0, -4.0),
                radius: 3.0,
                material: Rc::new(Lambertian { albedo: Vec3::new(0.9, 0.1, 0.1) }),
            }),
            Box::new(Sphere {
                center: Vec3::new(6.0, 1.0, -7.0),
                radius: 3.0,
                material: Rc::new(Metal { albedo: Vec3::new(1., 1., 1.), fuzzyness: 0.05 }),
            }),
            Box::new(Sphere {
                center: Vec3::new(-1.0, 1.0, -7.5),
                radius: 2.0,
                material: Rc::new(Dielectric { refract_idx: 1.5 }),
            }),
            Box::new(Plane {
                uplane: UPlane {
                    normal: Vec3::new(0.0, 1.0, 0.0),
                    bias: 2.0,
                },
                material: Rc::new(Lambertian { albedo: Vec3::new(0.0, 0.5, 0.5) }),
            })
        ]
    };

    let mut raytracer = Raytracer {
        camera: Camera::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, 0.0, -3.0), width, height, 90.0),
        ss: Box::new(MontecarloSS::new(16)),
        max_bounces: 8,
    };

    raytracer.render_stream(&scene, |x, y, color| {
        image.put_pixel(x, y, Rgb([(color.0 * 255.0) as u8, (color.1 * 255.0) as u8, (color.2 * 255.0) as u8]));
        pb.inc();
    });
    image.save(&args[3]).unwrap();
    pb.finish_println("OK !")
}
