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
use crate::PathTracing::Materials::Matte::Matte;
use crate::PathTracing::Textures::Checkerboard::Checkerboard;
use minifb::WindowOptions;

fn main() {
    let width = 1200;
    let height = 800;
    let samples_per_pixel = 1;
    // let threads = 2;  will multithread eventually

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
                z: -40.0
            },
            fov: 30.0,
            aspect_ratio: width as f32 / height as f32
        },
        objects: vec![
            ObjectEnum::Sphere(Sphere {
                pos: Vec3 {
                    x: 0.0,
                    y: 0.2,
                    z: 0.0
                },
                radius: 5.0,
                material: Material {
                    material: MaterialEnum::Matte(Matte {}),
                    texture: TextureEnum::Base(Base {
                        color: Rgb {
                            r: 0.0,
                            g: 0.0,
                            b: 1.0
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
                    material: MaterialEnum::Matte(Matte {}),
                    texture: TextureEnum::Checkerboard(Checkerboard {
                        color1: Rgb {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0
                        },
                        color2: Rgb {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0
                        },
                        size1: 2.0,
                        size2: 20.0
                    })
                }
            })
        ]
    };



        scene.render(&mut window, samples_per_pixel);




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

