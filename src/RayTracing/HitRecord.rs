use crate::Classes::Vec3::Vec3;
use crate::RayTracing::Ray::Ray;

pub struct HitRecord {
    pub closest_point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub front_face: bool,
    pub hit: bool
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
            front_face: false,
            hit: false
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray) {
        self.front_face = ray.direction.dot(self.normal) < 0.0;
        self.normal = if self.front_face {self.normal} else {-self.normal}
    }
}