use crate::PathTracing::Classes::Ray::Ray;
use crate::PathTracing::Classes::HitRecord::HitRecord;
use crate::Classes::Vec3::Vec3;
use crate::Classes::Rgb::Rgb;

pub trait Shape {
    fn intersection(&self, ray: Ray) -> HitRecord;
    fn get_uv_at(&self, vec: Vec3) -> Rgb;
    fn trace(&self, ray: Ray) -> bool;  // this is for less intensive checks that dont need color, distance etc.
}