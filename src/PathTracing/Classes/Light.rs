use crate::Classes::Vec3::Vec3;
use crate::PathTracing::Classes::Ray::Ray;
use crate::PathTracing::Enums::ObjectEnum::ObjectEnum;
use crate::PathTracing::Traits::Shape::Shape;
use std::f32::INFINITY;

// this is most likely temporary, it can be made much better ( color, intensity, etc )
pub struct Light {
    pub pos: Vec3
}

impl Light {
    // another check for intersection because this time we dont care about the closest object
    pub fn is_obstructed(&self, objects: &Vec<ObjectEnum>, vec: Vec3) -> bool {
        let light_direction = (self.pos - vec);
        let shadow_ray = Ray{ origin: vec, direction: light_direction.to_unit_vector()};

        let mut obstructed = false;
        for obj in objects {
            match obj {
                ObjectEnum::Sphere(sphere) => {
                    obstructed = sphere.intersection(shadow_ray).hit;
                    break
                }
                _ => {}
            }
        }

        return obstructed;
    }
}