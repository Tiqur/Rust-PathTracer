use crate::Classes::Rgb::Rgb;
use crate::PathTracing::Enums::MaterialEnum::MaterialEnum;
use crate::PathTracing::Enums::TextureEnum::TextureEnum;

pub struct Material {
    pub material: MaterialEnum,
    pub texture: TextureEnum
}

impl Material {
    pub fn uv_pattern_at(&self, u: f32, v: f32) -> Rgb {
        match &self.texture {
            TextureEnum::Base(texture) => {
                return texture.color;
            }
            TextureEnum::Checkerboard(texture) => {
                return texture.uv_pattern_at(u, v);
            }
        }
    }
}