use crate::RayTracing::Ray::Ray;
use crate::RayTracing::HitRecord::HitRecord;

pub trait Shape {
    fn intersection(&self, ray: Ray) -> f32;
}