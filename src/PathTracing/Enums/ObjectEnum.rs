use crate::PathTracing::Objects::Sphere::Sphere;
use crate::PathTracing::Objects::Triangle::Triangle;
use crate::PathTracing::Objects::Rectangle::Rect;

#[derive(Copy, Clone)]
pub enum ObjectEnum {
    Sphere(Sphere),
    Triangle(Triangle),
    Rectangle(Rect)
}