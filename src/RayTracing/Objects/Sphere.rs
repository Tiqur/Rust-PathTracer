use crate::Classes::Vec3::Vec3;
use crate::RayTracing::Traits::Shape::Shape;
use crate::Classes::Rgb::Rgb;
use crate::RayTracing::Ray::Ray;

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32
}

impl Shape for Sphere {
    fn intersection(&self, ray: Ray) -> f32 {
        let oc = ray.origin - self.pos;
        let a = ray.direction.get_magnitude_squared();
        let b = ray.direction.dot(oc);
        let c = oc.get_magnitude_squared() - (self.radius * self.radius);

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            return (-b - discriminant.sqrt()) / (2.0 * a)
        } else {
            return -1.0
        }
    }
}