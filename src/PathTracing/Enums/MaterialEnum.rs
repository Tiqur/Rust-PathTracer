use crate::PathTracing::Materials::Matte::Matte;
use crate::PathTracing::Materials::Mirror::Mirror;

pub enum MaterialEnum {
    Matte(Matte),
    Mirror(Mirror)
}