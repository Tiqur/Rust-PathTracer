use crate::Classes::Vec3::Vec3;
use crate::Classes::Rgb::Rgb;
use crate::PathTracing::Enums::ObjectEnum::ObjectEnum;
use crate::PathTracing::Objects::Sphere::Sphere;
use crate::PathTracing::Classes::Material::Material;

pub struct HitRecord {
    pub closest_point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub hit: bool,
    pub color: Rgb,
    pub object: ObjectEnum
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord {
            closest_point: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            distance: -1.0,
            hit: false,
            color: Rgb {..Default::default()},
            object: ObjectEnum::Sphere(Sphere{..Default::default()})
        }
    }
}
