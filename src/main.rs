use minifb;

mod Classes;
use Classes::Vec3::Vec3;

mod PathTracing;
use crate::PathTracing::Classes::Camera::Camera;
use PathTracing::Classes::Scene::Scene;

use std::time::*;
use crate::PathTracing::Objects::Sphere::Sphere;
use crate::PathTracing::Enums::ObjectEnum::ObjectEnum;
use crate::Classes::Rgb::Rgb;
use crate::PathTracing::Classes::Material::Material;
use crate::PathTracing::Enums::TextureEnum::TextureEnum;
use crate::PathTracing::Textures::Base::Base;
use crate::PathTracing::Enums::MaterialEnum::MaterialEnum;
use crate::PathTracing::Materials::Diffuse::Diffuse;
use crate::PathTracing::Textures::Checkerboard::Checkerboard;
use minifb::WindowOptions;
use crate::PathTracing::Classes::Light::Light;
use crate::PathTracing::Materials::Mirror::Mirror;
use crate::PathTracing::Objects::Triangle::Triangle;
use std::sync::{Arc, Mutex, MutexGuard};

fn main() {
    let width = 1200;
    let height = 800;
    let samples_per_pixel = 100;
    let show_statistics = true;
    let recursion_depth = 50;
    // let threads = 2;  will multithread eventually

    let mut window = Arc::new(Mutex::new(Classes::Window::Window {
        width,
        height,
        primary_buffer: vec![0; width * height],
        secondary_buffer: vec![0; width * height]
    }));


    let scene = Scene {
        camera: Camera {
            pos: Vec3 {
                x: 0.0,
                y: 4.0,
                z: -60.0
            },
            fov: 30.0,
            aspect_ratio: width as f32 / height as f32
        },
        objects: vec![
            ObjectEnum::Triangle(Triangle{
                vec1: Vec3 {
                    x: -2.0,
                    y: 20.0,
                    z: -30.0
                },
                vec2: Vec3 {
                    x: -30.0,
                    y: 4.0,
                    z: -10.0
                },
                vec3: Vec3 {
                    x: 10.0,
                    y: 3.0,
                    z: 50.0
                },
                material: Material {
                    material: MaterialEnum::Mirror(Mirror {}),
                    texture: TextureEnum::Base(Base {
                        color: Rgb {
                            r: 0.0,
                            g: 1.0,
                            b: 0.0
                        }
                    })
                }
            }),
            ObjectEnum::Sphere(Sphere {
                pos: Vec3 {
                    x: -7.0,
                    y: 1.0,
                    z: 0.0
                },
                radius: 5.0,
                material: Material {
                    material: MaterialEnum::Mirror(Mirror {}),
                    texture: TextureEnum::Base(Base {
                        color: Rgb {
                            r: 0.0,
                            g: 1.0,
                            b: 0.0
                        }
                    })
                }
            }),
            ObjectEnum::Sphere(Sphere {
                pos: Vec3 {
                    x: 7.0,
                    y: 1.0,
                    z: 0.0
                },
                radius: 5.0,
                material: Material {
                    material: MaterialEnum::Diffuse(Diffuse {}),
                    texture: TextureEnum::Base(Base {
                        color: Rgb {
                            r: 0.0,
                            g: 1.0,
                            b: 0.0
                        }
                    })
                }
            }),

            ObjectEnum::Sphere(Sphere {
                pos: Vec3 {
                    x: 0.0,
                    y: -505.0,
                    z: 0.0
                },
                radius: 500.0,
                material: Material {
                    material: MaterialEnum::Diffuse(Diffuse {}),
                    texture: TextureEnum::Checkerboard(Checkerboard {
                        color1: Rgb {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0
                        },
                        color2: Rgb {
                            r: 0.878,
                            g: 0.878,
                            b: 0.784
                        },
                        size1: 1.0,
                        size2: 1.0
                    })
                }
            })
        ],
        lights: vec![
            Light{ pos: Vec3 {
                x: -40.0,
                y: 30.0,
                z: 0.0
            }},
            Light{ pos: Vec3 {
                x: 40.0,
                y: 30.0,
                z: 0.0
            }}
        ]
    };

        let mut window_clone = Arc::clone(&window);
        let secondary_thread = std::thread::spawn(move || {
            scene.render(&window_clone, samples_per_pixel, show_statistics, recursion_depth);
        });




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
            window.lock().unwrap().swap_buffers();
            // I know this isn't good practice
            std::thread::sleep(Duration::from_secs_f32(0.01));
            minifb_window.update_with_buffer(&window.lock().unwrap().primary_buffer, width, height)
                .unwrap();
        }
    // with this commented out, the secondary process will stop if window is closed
    //secondary_thread.join().unwrap();

}

