use minifb;
use minifb::{WindowOptions, Key, KeyRepeat};

mod Classes;
use Classes::Vec3::Vec3;

mod PathTracing;
use crate::PathTracing::Classes::Camera::Camera;
use PathTracing::Classes::Scene::Scene;

use std::time::*;
use std::thread;
use crate::PathTracing::Objects::Sphere::Sphere;
use crate::PathTracing::Enums::ObjectEnum::ObjectEnum;
use crate::PathTracing::Material::Material;
use crate::Classes::Rgb::Rgb;

fn main() {
    let width = 1200;
    let height = 800;
    let samples_per_pixel = 64;
    // let threads = 2;  will multithread eventually

    let mut window = Classes::Window::Window {
        width,
        height,
        primary_buffer: vec![0; width * height],
        secondary_buffer: vec![0; width * height]
    };


    let mut scene = Scene {
        camera: Camera {
            pos: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -40.0
            },
            fov: 30.0,
            aspect_ratio: width as f32 / height as f32
        },
        objects: vec![
            ObjectEnum::Sphere(Sphere {
                pos: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                radius: 5.0,
                material: Material {
                    color: Rgb {
                        r: 0.5,
                        g: 1.0,
                        b: 1.0
                    }
                }
            }),
            ObjectEnum::Sphere(Sphere {
                pos: Vec3 {
                    x: 0.0,
                    y: -505.0,
                    z: 0.0
                },
                radius: 500.0,
                material: Default::default()
            })
        ]
    };



        let now = Instant::now();

        scene.render(&mut window, samples_per_pixel);

        println!("Render time: {}ms", now.elapsed().as_millis());



        let mut minifb_window = minifb::Window::new(
            "Rust-PathTracer",
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

