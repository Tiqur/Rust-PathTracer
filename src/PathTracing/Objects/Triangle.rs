use crate::Classes::Vec3::Vec3;
use crate::PathTracing::Classes::Material::Material;
use crate::PathTracing::Traits::Shape::Shape;
use crate::PathTracing::Classes::Ray::Ray;
use crate::Classes::Rgb::Rgb;
use crate::PathTracing::Classes::HitRecord::HitRecord;
use crate::Classes::Point2D::Point2D;
use crate::PathTracing::Enums::ObjectEnum::ObjectEnum;

const EPSILON: f32 = 0.0000001;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub vec1: Vec3,
    pub vec2: Vec3,
    pub vec3: Vec3,
    pub material: Material
}

impl Shape for Triangle {
    fn intersection(&self, ray: Ray) -> HitRecord { // https://en.wikipedia.org/wiki/Möller–Trumbore_intersection_algorithm
        let mut record = HitRecord::default();
        let vertex0 = self.vec1;
        let vertex1 = self.vec2;
        let vertex2 = self.vec3;
        let edge1 = vertex1 - vertex0;
        let edge2 = vertex2 - vertex0;
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);
        if a > -EPSILON && a < EPSILON {
            record.hit = false;  // ray is parallel to triangle
            return record;
        }
        let f = 1.0 / a;
        let s = ray.origin - vertex0;
        let u = f * s.dot(h);
        if u < 0.0 || u > 1.0 {
            record.hit = false;
            return record;
        }
        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);
        if v < 0.0 || u + v > 1.0 {
            record.hit = false;
            return record;
        }
        let t = f * edge2.dot(q);
        if t > EPSILON {
            record.hit = true;
            record.closest_point = ray.get_point(t);
            record.color = self.get_uv_at(record.closest_point);
            record.object = ObjectEnum::Triangle(self.clone());
            record.distance = t;
            record.normal = edge1.cross(edge2).to_unit_vector();
        } else {
            record.hit = false;
        }
        return record;
    }

    fn get_uv_at(&self, p: Vec3) -> Rgb {
        return self.material.uv_pattern_at(Point2D {x: p.x, y: p.z})
    }
}