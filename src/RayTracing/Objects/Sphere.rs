use crate::Classes::Vec3::Vec3;
use crate::RayTracing::Traits::Shape::Shape;
use crate::Classes::Rgb::Rgb;

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32
}

impl Shape for Sphere {
    fn intersection(&self) -> f32 {
        unimplemented!()
    }
}