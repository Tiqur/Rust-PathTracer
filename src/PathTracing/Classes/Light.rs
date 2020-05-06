use crate::Classes::Vec3::Vec3;
use crate::PathTracing::Classes::Ray::Ray;
use crate::PathTracing::Enums::ObjectEnum::ObjectEnum;
use crate::PathTracing::Traits::Shape::Shape;

// this is most likely temporary, it can be made much better ( color, intensity, etc )
pub struct Light {
    pub pos: Vec3
}

impl Light {
    pub fn get_ray(&self, vec: Vec3) -> Ray {
        return Ray{ origin: vec, direction: self.pos.to_unit_vector()};
    }

    // another check for intersection because this time we dont care about the closest object
    pub fn is_obstructed(&self, objects: &Vec<ObjectEnum>, ray: Ray) -> bool {
        let mut obstructed = false;
        for obj in objects {
            match obj {
                ObjectEnum::Sphere(sphere) => {
                    obstructed = sphere.intersection(ray).hit;
                    break
                }
                _ => {}
            }
        }

        return obstructed;
    }
}