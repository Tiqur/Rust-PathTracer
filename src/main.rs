use minifb;
use minifb::WindowOptions;

mod Classes;
use Classes::Vec3::Vec3;
use Classes::Rgb::Rgb;
use Classes::Point2D::Point2D;

mod RayTracing;
use RayTracing::Ray::Ray;
use RayTracing::Camera::Camera;
use RayTracing::Traits::Shape::Shape;
use RayTracing::Scene::Scene;

use std::time::*;
use std::thread;

fn main() {
    let width = 1200;
    let height = 800;
    let threads = 2;
    let mut window = Classes::Window::Window {
        width,
        height,
        primary_buffer: vec![0; width * height],
        secondary_buffer: vec![0; width * height]
    };
    
    
    let scene = Scene {
        camera: Camera {
            pos: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        },
        objects: vec![]
    };

    scene.render(&mut window);

    let mut minifb_window = minifb::Window::new(
        "RayTracer",
        width,
        height,
        WindowOptions::default()
    )
        .unwrap_or_else(|e| {
            panic!("{}", e)
        });




    while minifb_window.is_open() && !minifb_window.is_key_down(minifb::Key::Escape) {
        window.swap_buffers();
        minifb_window.update_with_buffer(&window.primary_buffer, width, height)
            .unwrap();
    }


}
