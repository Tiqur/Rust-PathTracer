use crate::PathTracing::Traits::Shape::Shape;
use crate::Classes::Vec3::Vec3;
use crate::PathTracing::Classes::Material::Material;
use crate::PathTracing::Classes::Ray::Ray;
use crate::PathTracing::Classes::HitRecord::HitRecord;
use crate::Classes::Rgb::Rgb;
use crate::PathTracing::Objects::Triangle::Triangle;
use crate::PathTracing::Enums::ObjectEnum::ObjectEnum;
use crate::PathTracing::Enums::TextureEnum::TextureEnum;
use crate::PathTracing::Textures::Base::Base;

#[derive(Copy, Clone)]
pub struct Rect {
    pub vec1: Vec3,
    pub vec2: Vec3,
    pub vec3: Vec3,
    pub vec4: Vec3,
    pub material: Material
}

impl Shape for Rect {
    fn intersection(&self, ray: Ray) -> HitRecord {
        let mut record = HitRecord::default();
        let triangles = self.triangles();

        if triangles[0].trace(ray) {
            record = triangles[0].intersection(ray)
        } else if triangles[1].trace(ray) {
            record = triangles[1].intersection(ray)
        }

        return record;
    }

    fn get_uv_at(&self, vec: Vec3) -> Rgb {
        unimplemented!()
    }

    fn trace(&self, ray: Ray) -> bool {
        let triangles = self.triangles();
        return triangles[0].trace(ray) || triangles[1].trace(ray)
    }
}

impl Rect {
    fn triangles(&self) -> [Triangle; 2] {
        return [
            Triangle {
                vec1: self.vec1,
                vec2: self.vec2,
                vec3: self.vec3,
                material: self.material
            }, Triangle {
                vec1: self.vec1,
                vec2: self.vec3,
                vec3: self.vec4,
                material: self.material
            }
        ]
    }
}