use crate::Classes::Vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn get_point(&self, t: f32) -> Vec3 {
        return Vec3 {
            x: self.origin.x + t * self.direction.x,
            y: self.origin.y + t * self.direction.y,
            z: self.origin.z + t * self.direction.z
        };
    }
} 