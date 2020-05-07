use crate::PathTracing::Objects::Sphere::Sphere;
use crate::PathTracing::Objects::Triangle::Triangle;

#[derive(Copy, Clone)]
pub enum ObjectEnum {
    Sphere(Sphere),
    Triangle(Triangle)
}