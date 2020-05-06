use crate::PathTracing::Enums::ObjectEnum::ObjectEnum;
use crate::Classes::Window::Window;
use crate::PathTracing::Classes::Ray::Ray;
use crate::Classes::Rgb::Rgb;
use crate::PathTracing::Classes::Camera::Camera;
use crate::PathTracing::Traits::Shape::Shape;
use crate::PathTracing::Classes::HitRecord::HitRecord;
extern crate rand;
use rand::Rng;
use crate::PathTracing::Functions::progress_bar::progress_bar;
use console::{Term, style};
use std::time::Instant;
use crate::Classes::Vec3::Vec3;
use crate::PathTracing::Classes::Light::Light;
use crate::PathTracing::Classes::Material::Material;
use crate::PathTracing::Enums::MaterialEnum::MaterialEnum;

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<ObjectEnum>,
    pub lights: Vec<Light> // again this is temp
}



impl Scene {



    fn get_background_gradient(&self, window: &mut Window) -> Vec<Rgb> {
        let mut gradient_rgb_buffer = Vec::new();

        let color1 = Rgb{
            r: 0.35,
            g: 0.35,
            b: 0.7
        };

        let color2 = Rgb {
            r: 1.0,
            g: 1.0,
            b: 1.0
        };

        for y in 0..window.height {
            for _x in 0..window.width {
                gradient_rgb_buffer.push(color1.mix(&color2, y as f32 / window.width as f32));
            }
        }

        return gradient_rgb_buffer;

    }

    fn ray_trace(&self, ray: Ray, recusion_depth: i32) -> HitRecord {
        let mut record = HitRecord {..Default::default()};


            for obj in &self.objects {

                match obj {
                    ObjectEnum::Sphere(sphere) => {


                        let object_hit_record =  sphere.intersection(ray);
                        // if intersection
                        if object_hit_record.hit {
                            // if no previous intersection or new intersection is closer
                            if !record.hit || object_hit_record.distance < record.distance {
                                record = object_hit_record;
                            }
                        }


                        match &sphere.material.material {
                            MaterialEnum::Matte(mat) => {

                            },
                            MaterialEnum::Mirror(mat) => {
                                if recusion_depth > 0 {
                                    let dn = 2.0 * ray.direction.dot(record.normal);
                                    let reflection_dir = ray.direction - Vec3 {
                                        x: record.normal.x * dn,
                                        y: record.normal.y * dn,
                                        z: record.normal.z * dn
                                    }.to_unit_vector();
                                    let reflection_ray = Ray { origin: record.closest_point, direction: reflection_dir };
                                    record.color = self.ray_trace(reflection_ray, recusion_depth - 1).color
                                }

                            }
                            _ => {}
                        }





                    }
                }
            }


        return record;
    }

    pub fn render(&self, window: &mut Window, samples_per_pixel: i32, show_statistics: bool) {


        // initialize rng generator
        let mut rng = rand::thread_rng();
        // caches background values
        let gradient_background_buffer = self.get_background_gradient(window);

        let mut ray_time_sum = 0.0;

        let mut index = 0;

        let term = Term::stdout();
        term.clear_screen();
        term.hide_cursor();
        let now = Instant::now();
        println!("{}", style("Starting render...").cyan());

        // sends out rays
        for y in 0..window.height {
                if show_statistics {
                    // this is just to clear the rest of the line when updating terminal ( Using this instead of just clearing it to prevent flickering )
                    let mut spaces_to_clear_line = "          ";
                    // I know this is unoptimized but the more calculations per pixel, the less this will matter
                    // Also note to self: remember to edit this when adding more computation per ray ( like lighting etc )
                    term.move_cursor_to(0, 1);
                    let avg_ray_time = (ray_time_sum / (index + 1) as f64 ) / 1000.0;
                    let remaining_rays = (window.width * window.height) - (index + window.width);
                    println!("{} (ms): {}", style("Average ray calculation time").cyan(), style(avg_ray_time).yellow());
                    println!("{}: {}{}", style("Rays remaining").cyan(), style(remaining_rays).yellow(), spaces_to_clear_line);
                    println!("{}: {:.2}{}", style("Estimated seconds remaining").cyan(), style((avg_ray_time * remaining_rays as f64) * 1000.0).yellow(), spaces_to_clear_line);
                    println!("{}", progress_bar(y as i32, window.height as i32, 30));
                }

            for x in 0..window.width {

                // actual pixel color after anti aliasing
                let mut pixel_color = Rgb {..Default::default()};

                // anti aliasing
                for i in 0..samples_per_pixel {
                    let ray_time_start = Instant::now();

                    let mut x_rand = x as f32;
                    let mut y_rand = y as f32;

                    if samples_per_pixel > 1 {
                        x_rand += rng.gen_range(0.0, 1.0);
                        y_rand += rng.gen_range(0.0, 1.0);
                    }

                    let ray = self.camera.get_ray(x_rand, y_rand, &window);

                    let mut sample_color = Rgb {..Default::default()};

                    // cast ray
                    let ray_trace_record = self.ray_trace(ray, 5);

                    // if ray trace hits object,
                    if ray_trace_record.hit {
                        // cast shadow ray
                        for light in &self.lights {
                            let light_source_obstructed = light.is_obstructed(&self.objects, ray_trace_record.closest_point);
                            sample_color = ray_trace_record.color;
                            if light_source_obstructed {
                                // light source is not obstructed
                               // sample_color = Rgb {..Default::default()};
                            }
                        }

                    } else {
                        // this will need to be fixed before reflections are implemented correctly
                        sample_color = gradient_background_buffer[index];
                    }


                    pixel_color += sample_color;
                    ray_time_sum += ray_time_start.elapsed().as_secs_f64();
                }
                window.secondary_buffer[index] = pixel_color.div(samples_per_pixel as f32).to_int();
                index += 1;
            }
        }
        println!("{}: {}", style("Render time").cyan(), style(format!("{}ms", now.elapsed().as_millis())).green());
    }
}