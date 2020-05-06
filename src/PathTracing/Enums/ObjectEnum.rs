use crate::PathTracing::Objects::Sphere::Sphere;

#[derive(Copy, Clone)]
pub enum ObjectEnum {
    // will add more later on
    Sphere(Sphere)
}