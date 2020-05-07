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
use std::ops::Mul;
use std::f32::consts::PI;
use std::sync::{Mutex, Arc};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<ObjectEnum>,
    pub lights: Vec<Light> // again this is temp
}



impl Scene {


    fn get_closest_hit_record(&self, objects: &Vec<ObjectEnum>, ray: Ray) -> HitRecord {
        let mut record = HitRecord {..Default::default()};
        for obj in objects {
            let mut object_hit_record;
            match obj {
                ObjectEnum::Sphere(sphere) => {
                    object_hit_record =  sphere.intersection(ray);
                },
                ObjectEnum::Triangle(triangle) => {
                    object_hit_record =  triangle.intersection(ray);
                }
            }
            // if intersection
            if object_hit_record.hit {
                // if no previous intersection or new intersection is closer
                if !record.hit || object_hit_record.distance < record.distance {
                    record = object_hit_record;
                }
            }
        }
        return record;
    }

    // I know that all the square roots here are inefficient
    fn random_in_unit_sphere(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0.0, 2.0 * PI);
        let z = rng.gen_range(-1.0, 1.0) as f32;
        let r = (1.0 - z * z).sqrt() as f32;
        return Vec3 {
            x: r * a.cos(),
            y: r * a.sin(),
            z: z
        }
    }

    fn random_in_hemisphere(&self, normal: Vec3) -> Vec3 {
        let in_unit_sphere = self.random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere
        } else {
            return -in_unit_sphere
        }
    }



    fn ray_trace(&self, ray: Ray, recursion_depth: i32) -> Rgb {
        // gets closest object and it's normal, distance, etc
        let mut record = self.get_closest_hit_record(&self.objects, ray);

        if recursion_depth < 0 {
            return Rgb::default()
        }

        if record.hit {
            match record.object {
                ObjectEnum::Sphere(sphere) => {
                    match sphere.material.material {
                        MaterialEnum::Diffuse(material) => {
                            // send diffuse ray
                            let target = (record.closest_point + record.normal + self.random_in_hemisphere(record.normal)) - record.closest_point;
                            let new_ray = Ray { origin: record.closest_point, direction: target};
                            return record.color * self.ray_trace(new_ray, recursion_depth - 1).mulF(0.5);
                        },
                        MaterialEnum::Mirror(material) => {
                            // send reflection ray
                            let dn = 2.0 * ray.direction.dot(record.normal);
                            let reflection_dir = ray.direction - record.normal.mulF(dn);
                            let reflection_ray = Ray { origin: record.closest_point, direction: reflection_dir.to_unit_vector() };
                            return self.ray_trace(reflection_ray, recursion_depth - 1)
                        }
                    }
                },
                ObjectEnum::Triangle(triangle) => {
                    match triangle.material.material {
                        MaterialEnum::Diffuse(material) => {
                            // send diffuse ray
                            let target = (record.closest_point + record.normal + self.random_in_hemisphere(record.normal)) - record.closest_point;
                            let new_ray = Ray { origin: record.closest_point, direction: target};
                            return record.color * self.ray_trace(new_ray, recursion_depth - 1).mulF(0.5);
                        },
                        MaterialEnum::Mirror(material) => {
                            // send reflection ray
                            let dn = 2.0 * ray.direction.dot(record.normal);
                            let reflection_dir = ray.direction - record.normal.mulF(dn);
                            let reflection_ray = Ray { origin: record.closest_point, direction: reflection_dir.to_unit_vector() };
                            return self.ray_trace(reflection_ray, recursion_depth - 1)
                        }
                    }
                }

            }

        }




        let t = 0.5 * ( ray.direction.to_unit_vector().y + 1.0);
        let sky_color = Rgb{ r: 1.0, g: 1.0, b: 1.0 }.mulF(1.0-t) + Rgb{ r: 0.5, g: 0.7, b: 1.0 }.mulF(t);
        return sky_color;
    }

    pub fn render(&self, window: &Arc<Mutex<Window>>, samples_per_pixel: i32, show_statistics: bool, recursion_depth: i32) {


        // initialize rng generator
        let mut rng = rand::thread_rng();

        let mut ray_time_sum = 0.0;

        let mut index = 0;

        let term = Term::stdout();
        term.clear_screen();
        term.hide_cursor();
        let now = Instant::now();
        println!("{}", style("Starting render...").cyan());

        // sends out rays
        let height = window.lock().unwrap().height;
        let width = window.lock().unwrap().width;
        for y in 0..height {
                if show_statistics {
                    // this is just to clear the rest of the line when updating terminal ( Using this instead of just clearing it to prevent flickering )
                    let mut spaces_to_clear_line = "          ";
                    // I know this is unoptimized but the more calculations per pixel, the less this will matter
                    // Also note to self: remember to edit this when adding more computation per ray ( like lighting etc )
                    term.move_cursor_to(0, 1);
                    let avg_ray_time = (ray_time_sum / (index + 1) as f64 ) / 1000.0;
                    let remaining_rays = (width * height) - (index + width);
                    println!("{} (ms): {}", style("Average ray calculation time").cyan(), style(avg_ray_time).yellow());
                    println!("{}: {}{}", style("Rays remaining").cyan(), style(remaining_rays).yellow(), spaces_to_clear_line);
                    println!("{}: {:.2}{}", style("Estimated seconds remaining").cyan(), style((avg_ray_time * remaining_rays as f64) * 1000.0).yellow(), spaces_to_clear_line);
                    println!("{}", progress_bar(y as i32, height as i32, 30));
                }

            for x in 0..width {

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

                    let ray = self.camera.get_ray(x_rand, y_rand, &window.lock().unwrap());

                    //cast ray
                    pixel_color += self.ray_trace(ray, recursion_depth);

                    ray_time_sum += ray_time_start.elapsed().as_secs_f64();
                }

                // add gamma correction
                let scale = 1.0 / samples_per_pixel as f32;
                let gamma_corrected_pixel_color = Rgb{
                    r: (pixel_color.r * scale).sqrt(),
                    g: (pixel_color.g * scale).sqrt(),
                    b: (pixel_color.b * scale).sqrt()
                };
                window.lock().unwrap().secondary_buffer[index] = gamma_corrected_pixel_color.to_int();
                index += 1;
            }
        }
        println!("{}: {}", style("Render time").cyan(), style(format!("{}ms", now.elapsed().as_millis())).green());
    }
}