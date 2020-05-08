use crate::Classes::Vec3::Vec3;
use crate::PathTracing::Classes::Ray::Ray;
use crate::PathTracing::Enums::ObjectEnum::ObjectEnum;
use crate::PathTracing::Traits::Shape::Shape;
use std::f32::INFINITY;

#[derive(Copy, Clone)]
// this is most likely temporary, it can be made much better ( color, intensity, etc )
pub struct Light {
    pub pos: Vec3
}
