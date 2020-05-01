use minifb;
use minifb::{WindowOptions, Key, KeyRepeat};

mod Classes;
use Classes::Vec3::Vec3;

mod RayTracing;
use RayTracing::Camera::Camera;
use RayTracing::Scene::Scene;

use std::time::*;
use std::thread;
use crate::RayTracing::Objects::Sphere::Sphere;
use crate::RayTracing::Enums::ObjectEnum::ObjectEnum;

fn main() {
    let width = 1200;
    let height = 800;
    let samples_per_pixel = 10;
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
                z: -20.0
            },
            fov: 30.0,
            aspect_ratio: width as f32 / height as f32
        },
        objects: vec![
            ObjectEnum::Sphere(Sphere {
                pos: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 20.0
                },
                radius: 5.0
            }),
            ObjectEnum::Sphere(Sphere {
                pos: Vec3 {
                    x: 0.0,
                    y: -505.0,
                    z: 20.0
                },
                radius: 500.0
            })
        ]
    };



        // for z in 0..10 {
        //     scene.objects.push(ObjectEnum::Sphere(Sphere {
        //         pos: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: ((z * 4) * 2 + 30) as f32
        //         },
        //         radius: 4.0 * z as f32
        //     }))
        // }




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
            let now = Instant::now();
            scene.render(&mut window, samples_per_pixel);
            minifb_window.set_title(&now.elapsed().as_secs_f64().to_string());
            window.swap_buffers();

            minifb_window.get_scroll_wheel().map(|scroll| {
                if scroll.1 > 0.0 {
                    scene.camera.pos.z += 4.0;
                } else {
                    scene.camera.pos.z -= 4.0;
                }
            });

            minifb_window.get_keys_pressed(KeyRepeat::Yes).map(|keys| {
                for t in keys {
                    match t {
                        Key::W => {  // i know this doesn't exactly work the way it should
                            scene.camera.pos.y += 1.0;
                        },
                        Key::A => {
                            scene.camera.pos.x -= 1.0;
                        },
                        Key::S => {
                            scene.camera.pos.y -= 1.0;
                        },
                        Key::D => {
                            scene.camera.pos.x += 1.0;
                        },
                        _ => (),
                    }
                }
            });
            minifb_window.update_with_buffer(&window.primary_buffer, width, height)
                .unwrap();
        }


}

