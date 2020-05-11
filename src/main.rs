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
use std::sync::{Arc, Mutex};
use console::{style, Term};
use crate::PathTracing::Classes::Statistics::Statistics;
use crate::PathTracing::Functions::progress_bar::progress_bar;

fn main() {
    let width = 1200;
    let height = 800;
    let samples_per_pixel = 20;
    let show_statistics = true;
    let recursion_depth = 50;
    let thread_count = 6;
    let now = Instant::now();
    let mut render_time= now.elapsed().as_millis();
    let term = Term::stdout();
    term.clear_screen();
    term.hide_cursor();
    
    let mut window = Arc::new(Mutex::new(Classes::Window::Window {
        width,
        height,
        buffer: vec![0; width * height],
        running_threads: 0
    }));


    let scene = Arc::new(Scene {
        camera: Camera {
            pos: Vec3 {
                x: 0.0,
                y: 4.0,
                z: -60.0
            },
            fov: 30.0,
            aspect_ratio: width as f32 / height as f32
        },
        objects: [
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
        lights: [
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
    });


        println!("{}", style("Starting render...").cyan());
        //how large the chunk is ( if thread count is 2 and height is 800, the chunk size will be 400 )
        let chunk_size = height / thread_count as usize;
        // initialize statistics struct
        let statistics = Arc::new(Mutex::new(Statistics {
            show_stats: show_statistics,
            running: true,
            average_ray_calc_time: 0.0,
            average_pixel_calc_time: 0.0,
            remaining_pixels: (height * width) as i32,
            running_threads: thread_count
        }));
        // spawn threads
        for chunk in 0..height / chunk_size {
            if chunk as i32 == thread_count - 1 {  }
            let mut window_clone = Arc::clone(&window);
            let mut scene_clone = Arc::clone(&scene);
            let mut stats_clone = Arc::clone(&statistics);
            std::thread::spawn(move || {
                let min = chunk * chunk_size;
                let max = if chunk as i32 == thread_count - 1 { height } else { chunk_size * (chunk + 1) };
                scene_clone.render(window_clone, (min, max), samples_per_pixel, recursion_depth, stats_clone);
            });
        }


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
            // I know this isn't good practice
            std::thread::sleep(Duration::from_millis(10));
            if statistics.lock().unwrap().running_threads != 0 {
                render_time = now.elapsed().as_millis();
            }
            if show_statistics {
                // this is just to clear the rest of the line when updating terminal ( Using this instead of just clearing it to prevent flickering )
                let mut spaces_to_clear_line = "          ";
                let average_pixel_calc_time = statistics.lock().unwrap().average_pixel_calc_time;
                let average_ray_calc_time = statistics.lock().unwrap().average_ray_calc_time;
                let pixels_remaining = statistics.lock().unwrap().remaining_pixels;
                let running_threads = statistics.lock().unwrap().running_threads;
                let total_pixels = (width * height) as i32;
                term.move_cursor_to(0, 1);
                println!("{}", progress_bar(total_pixels - pixels_remaining, total_pixels, 30));
                println!("{} (ms): {}{}", style("Average pixel calculation time").cyan(), style(average_pixel_calc_time).yellow(), spaces_to_clear_line);
                println!("{} (ms): {}{}", style("Average ray calculation time").cyan(), style(average_ray_calc_time).yellow(), spaces_to_clear_line);
                println!("{}: {:.2}{}", style("Estimated seconds remaining").cyan(), style((average_pixel_calc_time * pixels_remaining as f64)).yellow(), spaces_to_clear_line);
                println!("{}: {}{}", style("Pixels remaining").cyan(), style(statistics.lock().unwrap().remaining_pixels).yellow(), spaces_to_clear_line);
                println!("{}: {}{}", style("Threads working").cyan(), style(running_threads).yellow(), spaces_to_clear_line);
                println!("{}: {}", style("Render time").cyan(), style(format!("{}ms", render_time)).green());

            } else {
                statistics.lock().unwrap().running = false;
            }
            minifb_window.update_with_buffer(&window.lock().unwrap().buffer, width, height)
                .unwrap();
        }

}

