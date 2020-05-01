
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
}