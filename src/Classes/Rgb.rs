
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Rgb {
    // converts to value able to be used with buffer
    pub fn to_int(&self) -> u32 {
        return ( 65536.0 * self.r + 256.0 * self.g + self.b) as u32;
    }

    // mixes two colors using linear interpolation
    pub fn mix(&self, c: Rgb, t: f32) -> Rgb {
        return Rgb {
            r: ( self.r + (c.r - self.r) * t ) as i32 as f32,
            g: ( self.g + (c.g - self.g) * t ) as i32 as f32,
            b: ( self.b + (c.b - self.b) * t ) as i32 as f32
        }
    }
}