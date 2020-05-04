use crate::PathTracing::Textures::Base::Base;
use crate::PathTracing::Textures::Checkerboard::Checkerboard;

pub enum TextureEnum {
    Base(Base),
    Checkerboard(Checkerboard)
}