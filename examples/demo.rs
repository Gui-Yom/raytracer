use std::env;
use std::rc::Rc;

use image::{ImageBuffer, Rgb};
use pbr::ProgressBar;
use ultraviolet::Vec3;

use softrays::Camera;
use softrays::geom::{Scene, Sphere};
use softrays::material::{Lambertian, Metal};
use softrays::Raytracer;

fn main() {
    println!("GuiYom's softraytracer v0.4.0");
    // Parsing arguments
    let args: Vec<String> = env::args().collect();
    let width: u32 = args[1].parse().unwrap();
    let height: u32 = args[2].parse().unwrap();
    println!("Writing to {}", args[3]);

    // Progress bar stuff
    let count = width * height;
    let update_rate: u32 = 1000;
    let mut total: u32 = 0;
    let mut pb = ProgressBar::new(count as u64);
    pb.format("[=> ]");
    pb.message("Casting rays : ");

    let mut image = ImageBuffer::new(width, height);
    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.5), width, height, 90.0);

    // World objects
    let scene = Scene {
        objects: vec![
            Box::new(Sphere {
                center: Vec3::new(-5.0, 0.0, -4.0),
                radius: 5.0,
                material: Rc::new(Lambertian { albedo: Vec3::new(0.8, 0.2, 0.3) }),
            }),
            Box::new(Sphere {
                center: Vec3::new(6.0, 1.0, -7.0),
                radius: 3.0,
                material: Rc::new(Metal { albedo: Vec3::new(0.2, 0.2, 0.2) }),
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, 112.0, -1.0),
                radius: 100.0,
                material: Rc::new(Lambertian { albedo: Vec3::new(0.0, 0.5, 0.5) }),
            })
        ]
    };

    let raytracer = Raytracer {
        camera,
        scene,
        mcaa: 16,
        max_bounces: 16,
    };

    raytracer.render_stream(|x, y, color| {
        image.put_pixel(x, y, Rgb([(color[0] * 255.0) as u8, (color[1] * 255.0) as u8, (color[2] * 255.0) as u8]));
        if total % update_rate == 0 {
            pb.set(total as u64);
        }
        total += 1;
    });

    image = image::imageops::flip_vertical(&image);
    image.save(&args[3]).unwrap();
    pb.finish_println("OK !")
}
