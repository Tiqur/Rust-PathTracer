use crate::Classes::Rgb::Rgb;

pub struct Material {
    pub color: Rgb
}

impl Default for Material {
    fn default() -> Material {
        Material {
            color: Rgb {..Default::default()}
        }
    }
}