use crate::PathTracing::Ray::Ray;
use crate::PathTracing::HitRecord::HitRecord;

pub trait Shape {
    fn intersection(&self, ray: Ray) -> HitRecord;
}