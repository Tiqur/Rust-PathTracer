use std::ops::{AddAssign, Add, Sub};
#[derive(Copy, Clone)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Rgb {
    // converts to value able to be used with buffer
    pub fn to_int(&self) -> u32 {
        let r = if self.r < 0.0 { 0.0 } else if self.r > 1.0 { 1.0 } else { self.r };
        let g = if self.g < 0.0 { 0.0 } else if self.g > 1.0 { 1.0 } else { self.g };
        let b = if self.b < 0.0 { 0.0 } else if self.b > 1.0 { 1.0 } else { self.b };
        return ( 65536.0 * (r * 255.0).round() + 256.0 * (g * 255.0).round() + (b * 255.0).round() ) as u32;
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

    pub fn mul(&self, f: f32) -> Rgb {
        Rgb {
            r: self.r * f,
            g: self.g * f,
            b: self.b * f
        }
    }
}

impl Add for Rgb {
    type Output = Rgb;
    fn add(self, other: Rgb) -> Rgb {
        return Rgb {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b
        }
    }
}

impl Sub for Rgb {
    type Output = Rgb;
    fn sub(self, other: Rgb) -> Rgb {
        return Rgb {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b
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