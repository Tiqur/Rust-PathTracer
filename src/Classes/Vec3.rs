use std::ops::*;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// returns the square of two numbers
fn sqr(n: f32) -> f32 {
    return n * n;
}

impl Add for Vec3 {  /* + */
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl AddAssign for Vec3 {  /* += */
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {  /* - */
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl SubAssign for Vec3 {  /* -= */
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Div for Vec3 {  /* / */
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl DivAssign for Vec3 {  /* /= */
    fn div_assign(&mut self, other: Vec3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Mul for Vec3 {  /* * */
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl MulAssign for Vec3 {  /* *= */
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl Vec3 {
    // mixes two points using linear interpolation
    pub fn mix(&self, other: Vec3, t: f32) -> Vec3 {
        return Vec3 {
            x: self.x + ( other.x - self.x ) * t,
            y: self.y + ( other.y - self.y ) * t,
            z: self.y + ( other.y - self.y ) * t
        }
    }

    // returns the distance between two vec3s
    pub fn dist(&self, other: Vec3) -> f32 {
        return (sqr(other.x - self.x) + sqr(other.y - self.y) + sqr(other.z - self.z)).sqrt()
    }

    // returns the dot product of two vec3s
    pub fn dot(&self, other: Vec3) -> f32 {
        return (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    // gets the magnitude ( length ) of a vec squared
    pub fn get_magnitude_squared(&self) -> f32 {
        return sqr(self.x) + sqr(self.y) + sqr(self.z);
    }

    // gets the magnitude ( length ) of a vec
    pub fn get_magnitude(&self) -> f32 {
        return (self.get_magnitude_squared()).sqrt();
    }

    // normalizes vec3
    pub fn normalized(&self) -> Vec3 {
        let mag = self.get_magnitude();
        return Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
} 