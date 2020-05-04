use crate::PathTracing::Enums::ObjectEnum::ObjectEnum;
use crate::Classes::Window::Window;
use crate::PathTracing::Classes::Ray::Ray;
use crate::Classes::Vec3::Vec3;
use crate::Classes::Rgb::Rgb;
use crate::PathTracing::Classes::Camera::Camera;
use crate::PathTracing::Traits::Shape::Shape;
use crate::PathTracing::Classes::HitRecord::HitRecord;
extern crate rand;
use rand::Rng;
use crate::PathTracing::Functions::progress_bar::progress_bar;

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<ObjectEnum>
}

enum ColorEnum {
    Color(Rgb),
    False(bool)
}

impl Scene {

    fn get_background_gradient(&self, window: &mut Window) -> Vec<Rgb> {
        let mut gradient_rgb_buffer = Vec::new();
        let mut index = 0;

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
            index += 1;
            }
        }

        return gradient_rgb_buffer;

    }

    fn ray_trace(&self, ray: Ray) -> ColorEnum {
        let mut ray_hit_record = HitRecord {..Default::default()};
        for obj in &self.objects {
            match obj {
                // sphere
                ObjectEnum::Sphere(sphere) => {
                    // passed in distance to check if closer before doing any unnecessary calculations.
                    let object_hit_record =  sphere.intersection(ray);
                    // if intersection
                    if object_hit_record.hit {
                        // if no previous intersection or new intersection is closer
                        if !ray_hit_record.hit || object_hit_record.distance < ray_hit_record.distance {
                            ray_hit_record = object_hit_record;
                        }
                    }
                }
            }
        }

        if ray_hit_record.hit  {
            return ColorEnum::Color(ray_hit_record.color);
            // return ColorEnum::Color( Rgb { // this is temp
            //     r: 0.5 * (ray_hit_record.normal.x + 1.0),
            //     g: 0.5 * (ray_hit_record.normal.y + 1.0),
            //     b: 0.5 * (ray_hit_record.normal.z + 1.0)
            // })
        }


        return ColorEnum::False(false);
    }

    pub fn render(&self, window: &mut Window, samples_per_pixel: i32) {


        // initialize rng generator
        let mut rng = rand::thread_rng();
        // caches background values
        let gradient_background_buffer = self.get_background_gradient(window);

        let mut index = 0;
        // sends out rays
        for y in 0..window.height {

            print!("{}[2J", 27 as char);  // this will only clear line in a normal terminal
            println!("{}\n{}", "Starting render...", progress_bar(y as i32, window.height as i32, 30));

            for x in 0..window.width {

                // actual pixel color after anti aliasing
                let mut pixel_color = Rgb {..Default::default()};

                // anti aliasing
                for a in 0..samples_per_pixel {

                    let mut x_rand = x as f32;
                    let mut y_rand = y as f32;

                    if samples_per_pixel > 1 {
                        x_rand += rng.gen_range(0.0, 1.0);
                        y_rand += rng.gen_range(0.0, 1.0);
                    }

                    let ray = self.camera.get_ray(x_rand, y_rand, &window);


                    let mut sample_color = Rgb {..Default::default()};

                    // ColorEnum
                    match self.ray_trace(ray) {
                        ColorEnum::Color(c) => {
                            sample_color = c;
                        },
                        ColorEnum::False(_f) => {
                            Rgb {..Default::default()};
                           sample_color = gradient_background_buffer[index];
                        }
                    };

                    pixel_color += sample_color;

                }
                window.secondary_buffer[index] = pixel_color.div(samples_per_pixel as f32).to_int();
                index += 1;
            }
        }
    }
}