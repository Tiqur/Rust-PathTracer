use crate::RayTracing::Enums::ObjectEnum::ObjectEnum;
use crate::Classes::Window::Window;
use crate::RayTracing::Ray::Ray;
use crate::Classes::Vec3::Vec3;
use crate::Classes::Rgb::Rgb;
use crate::RayTracing::Camera::Camera;
use crate::RayTracing::Traits::Shape::Shape;
use crate::RayTracing::HitRecord::HitRecord;

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
        for y in 0..window.height {
            for _x in 0..window.width {
                let color1 = Rgb {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0
                };
                let color2 = Rgb{
                    r: 0.15,
                    g: 0.15,
                    b: 0.7
                };
            gradient_buffer.push(color1.mix(color2, y as f32 / window.width as f32).to_int());
            index += 1;
            }
        }
        return gradient_buffer;
    }

    fn ray_trace(&self, ray: Ray) -> ColorEnum {
        let mut hit = false;
        let mut hit_record = HitRecord {..Default::default()};
        for obj in &self.objects {
            match obj {
                // sphere
                ObjectEnum::Sphere(sphere) => {

                    let distance =  sphere.intersection(ray);

                    // if intersection
                    if distance > 0.0 {
                        let point = ray.get_point(distance);
                        let normal = point.to_unit_vector();

                        if !hit || distance < hit_record.distance {
                            hit = true;
                            hit_record.distance = distance;
                            hit_record.closest_point = point;
                            hit_record.normal = normal;
                        }

                    }
                }
            }
        }

        if hit {
            return ColorEnum::Color( Rgb { // this is temp
                r: 0.5 * (hit_record.normal.x + 1.0),
                g: 0.5 * (hit_record.normal.y + 1.0),
                b: 0.5 * (hit_record.normal.z + 1.0)
            })
        }


        return ColorEnum::False(false);
    }

    pub fn render(&self, window: &mut Window) {
        // caches background values
        let gradient_background_buffer = self.get_background_gradient(window);

        let mut index = 0;
        for y in 0..window.height {
            for x in 0..window.width {

               let ray = self.camera.get_ray(x as f32, y as f32, &window);
                // ColorEnum
                match self.ray_trace(ray) {
                    ColorEnum::Color(color) => {
                        window.secondary_buffer[index] = color.to_int();
                    },
                    ColorEnum::False(_f) => {
                        window.secondary_buffer[index] = gradient_background_buffer[index];
                    }
                }
                index += 1;
            }
        }
    }
}