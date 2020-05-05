use crate::Classes::Rgb::Rgb;
use crate::PathTracing::Enums::MaterialEnum::MaterialEnum;
use crate::PathTracing::Enums::TextureEnum::TextureEnum;
use crate::Classes::Point2D::Point2D;

pub struct Material {
    pub material: MaterialEnum,
    pub texture: TextureEnum
}

impl Material {
    pub fn uv_pattern_at(&self, uv_point: Point2D) -> Rgb {
        match &self.texture {
            TextureEnum::Base(texture) => {
                return texture.color;
            }
            TextureEnum::Checkerboard(texture) => {
                return texture.uv_pattern_at(uv_point.x, uv_point.y);
            }
        }
    }
}