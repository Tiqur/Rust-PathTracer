use crate::RayTracing::Enums::DistEnum::DistEnum;
use crate::RayTracing::Ray::Ray;

pub trait Shape {
    fn intersection(&self, ray: Ray) -> DistEnum;
}