use crate::PathTracing::Materials::Diffuse::Diffuse;
use crate::PathTracing::Materials::Mirror::Mirror;

#[derive(Copy, Clone)]
pub enum MaterialEnum {
    Diffuse(Diffuse),
    Mirror(Mirror)
}