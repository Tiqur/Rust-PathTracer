use std::ops::{AddAssign};
#[derive(Copy, Clone)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Rgb {
    // converts to value able to be used with buffer
    pub fn to_int(&self) -> u32 {
        return ( 65536.0 * (self.r * 255.0).round() + 256.0 * (self.g * 255.0).round() + (self.b * 255.0).round() ) as u32;
    }

    // mixes two colors using linear interpolation
    pub fn mix(&self, c: &Rgb, t: f32) -> Rgb {
        return Rgb {
            r: ( self.r + (c.r - self.r) * t ),
            g: ( self.g + (c.g - self.g) * t ),
            b: ( self.b + (c.b - self.b) * t )
        }
    }

    pub fn div(&self, f: f32) -> Rgb {
        Rgb {
            r: self.r / f,
            g: self.g / f,
            b: self.b / f
        }
    }
}

impl AddAssign for Rgb {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}


impl Default for Rgb {
    fn default() -> Rgb {
        Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0
        }
    }
}