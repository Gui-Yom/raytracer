use std::env;
use std::fs::File;
use std::ops::{Deref, Div};
use std::rc::Rc;
use std::time::Duration;

use image::{Delay, Frame, ImageBuffer, Rgba};
use image::gif::Encoder;
use pbr::ProgressBar;
use ultraviolet::Vec3;

use softrays::{Camera, MontecarloSS, Raytracer};
use softrays::material::{Lambertian, Metal};
use softrays::world::{Scene, Sphere};

fn main() {
    // Parsing arguments
    let args: Vec<String> = env::args().collect();
    let width: u32 = args[1].parse().unwrap();
    let height: u32 = args[2].parse().unwrap();
    let frame_count: u32 = args[3].parse().unwrap();
    println!("Writing to {}", args[4]);

    // Progress bar stuff
    let mut pb = ProgressBar::new(frame_count as u64);
    pb.format("[=> ]");
    pb.message("Computing gif frames : ");
    pb.set_width(Some(20));
    pb.set_max_refresh_rate(Some(Duration::from_millis(500)));

    let mut scene = Scene {
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
                center: Vec3::new(0.0, -112.0, -1.0),
                radius: 100.0,
                material: Rc::new(Lambertian { albedo: Vec3::new(0.0, 0.5, 0.5) }),
            })
        ]
    };

    let mut raytracer = Raytracer {
        camera: Camera::new(Vec3::new(0.0, 0.0, 0.5), width, height, 90.0),
        ss: Box::new(MontecarloSS::new(8)),
        max_bounces: 4,
    };

    let mut file = File::create(&args[4]).unwrap();
    let mut encoder = Encoder::new(file);
    let mut time: f32 = 0.0;

    for _ in 0..frame_count {
        let mut buffer = ImageBuffer::new(width, height);
        raytracer.render_stream(&scene, |x, y, color| {
            buffer.put_pixel(x, y, Rgba([(color.0 * 255.0) as u8, (color.1 * 255.0) as u8, (color.2 * 255.0) as u8, 255]));
        });
        let frame = Frame::from_parts(buffer, 0, 0, Delay::from_numer_denom_ms(100, 1));
        encoder.encode_frame(frame);
        scene.objects[1].as_any().downcast_mut::<Sphere>().unwrap().center.y = 1.0 + 2.0 * time.div(500.0).sin();
        time += 100.0;
        pb.inc();
    }
    pb.finish_println("OK !");
}
