use crate::PathTracing::Textures::Base::Base;
use crate::PathTracing::Textures::Checkerboard::Checkerboard;

#[derive(Copy, Clone)]
pub enum TextureEnum {
    Base(Base),
    Checkerboard(Checkerboard)
}