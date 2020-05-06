use crate::Classes::Rgb::Rgb;

#[derive(Copy, Clone)]
pub struct Checkerboard {
    pub color1: Rgb,
    pub color2: Rgb,
    pub size1: f32,
    pub size2: f32
}

impl Checkerboard {
    pub fn uv_pattern_at(&self, u: f32, v: f32) -> Rgb {
        let u2 = (u * self.size1).round();
        let v2 = (v * self.size2).round();
        if (u2 + v2) % 2.0 == 0.0 {
            return self.color1;
        } else {
            return self.color2;
        }
    }
}