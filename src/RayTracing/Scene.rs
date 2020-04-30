use crate::RayTracing::Enums::ObjectEnum::ObjectEnum;
use crate::Classes::Window::Window;
use crate::RayTracing::Ray::Ray;
use crate::Classes::Vec3::Vec3;
use crate::Classes::Rgb::Rgb;
use crate::RayTracing::Camera::Camera;
use crate::RayTracing::Traits::Shape::Shape;
use crate::RayTracing::Enums::DistEnum::DistEnum;

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<ObjectEnum>
}

enum ColorEnum {
    Color(Rgb),
    False(bool)
}

impl Scene {

    fn get_background_gradient(&self, window: &mut Window) -> Vec<u32> {
        let mut gradient_buffer = Vec::new();
        let mut index = 0;
            for x in 0..window.width {
                for y in 0..window.height {
                    let color1 = Rgb {
                        r: 255.0,
                        g: 255.0,
                        b: 255.0
                    };
                    let color2 = Rgb{
                        r: 40.0,
                        g: 40.0,
                        b: 170.0
                    };
                    gradient_buffer.push(color1.mix(color2, x as f32/ window.width as f32).to_int());
                    index += 1;
                }
            }
        return gradient_buffer;
    }

    fn ray_trace(&self, ray: Ray) -> ColorEnum {
        for obj in &self.objects {
            match obj {
                ObjectEnum::Sphere(sphere) => {
                    match sphere.intersection(ray) {
                        DistEnum::Distance(distance) => {
                            //println!("Intersection");
                            return ColorEnum::Color( Rgb{ // this is temp
                                r: 255.0,
                                g: 0.0,
                                b: 0.0
                            })
                        }
                        _ => {
                            
                        }
                    }
                }
            }
        }
        return ColorEnum::False(false);
    }

    pub fn render(&self, window: &mut Window) {
        // caches background values
        let gradient_background_buffer = self.get_background_gradient(window);

        let mut index = 0;
        for y in 0..window.height {
            for x in 0..window.width {
                let width = window.width as i32;
                let height = window.height as i32;
                let ray = Ray {
                    origin: self.camera.pos,
                    direction: Vec3 {
                        x: (x as f32 / width as f32 * 2.0 - (width / height) as f32) as f32,
                        y: (y as f32 / width as f32 * 2.0 - 1.0) as f32,
                        z: 1.0
                    }.normalized()
                };
               // println!("{} {} {}", ray.direction.x, ray.direction.y, ray.direction.z);


                // ColorEnum
                match self.ray_trace(ray) {
                    ColorEnum::Color(color) => {
                        window.secondary_buffer[index] = color.to_int();
                    },
                    ColorEnum::False(f) => {
                        window.secondary_buffer[index] = gradient_background_buffer[index];
                    }
                }

                index += 1;
            }
        }
    }
}