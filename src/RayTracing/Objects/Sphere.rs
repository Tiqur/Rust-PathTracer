use crate::Classes::Vec3::Vec3;
use crate::RayTracing::Traits::Shape::Shape;
use crate::RayTracing::Enums::DistEnum::DistEnum;
use crate::Classes::Rgb::Rgb;
use crate::RayTracing::Ray::Ray;

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32
}

impl Shape for Sphere {
    fn intersection(&self, ray: Ray) -> DistEnum {
       // let mut record_hit;
        let oc = ray.origin - self.pos;
        let a = ray.direction.get_magnitude_squared();
        let b = ray.direction.dot(oc);
        let c = oc.get_magnitude_squared() - (self.radius * self.radius);

        let discriminant = b * b - a * c;
        return if discriminant > 0.0 {
            DistEnum::Distance((-b - discriminant.sqrt()) / (2.0 * a))
        } else {
            DistEnum::False(false)
        }



        // let mut delta = b * b - c;
        // record_hit = delta > 0.0;
        // if record_hit {
        //     delta = delta.sqrt();
        //     let t0 = -b - delta;
        //     let t1 = -b + delta;
        //     if t0 < 0.0 || t1 < 0.0 {
        //         record_hit = false
        //     } else {
        //         return DistEnum::Distance(1.0)
        //     }
        // }
    }
}