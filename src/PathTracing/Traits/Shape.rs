use crate::PathTracing::Classes::Ray::Ray;
use crate::PathTracing::Classes::HitRecord::HitRecord;

pub trait Shape {
    fn intersection(&self, ray: Ray) -> HitRecord;
}