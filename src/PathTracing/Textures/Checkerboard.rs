use crate::Classes::Rgb::Rgb;

pub struct Checkerboard {
    pub color1: Rgb,
    pub color2: Rgb
}

impl Checkerboard {
    pub fn uv_pattern_at(&self, u: f32, v: f32) -> Rgb {
        return Rgb {
            r: 0.0,
            g: 1.0,
            b: 0.0
        }
    }
}