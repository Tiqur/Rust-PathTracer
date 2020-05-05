use crate::Classes::Vec3::Vec3;
use crate::PathTracing::Traits::Shape::Shape;
use crate::Classes::Rgb::Rgb;
use crate::PathTracing::Classes::Ray::Ray;
use crate::PathTracing::Classes::HitRecord::HitRecord;
use crate::PathTracing::Classes::Material::Material;
use std::f32::consts::PI;
use crate::Classes::Point2D::Point2D;

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32,
    pub material: Material
}

impl Sphere {

}


impl Shape for Sphere {
    fn intersection(&self, ray: Ray) -> HitRecord {  // thanks legend
        let mut hit_record = HitRecord{..Default::default()};
        let oc = ray.origin - self.pos;
        let b = ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let mut delta = b * b - c;
        hit_record.hit = delta > 0.0;
        if hit_record.hit {
            delta = delta.sqrt();
            let t0 = -b - delta;
            let t1 = -b + delta;
            if t0 < 0.0 || t1 < 0.0 {
                hit_record.hit = false;
            } else {
                hit_record.distance = if t0 < t1 {t0} else {t1};
                hit_record.closest_point = ray.get_point(hit_record.distance);
                hit_record.normal = (hit_record.closest_point - self.pos).to_unit_vector();
                hit_record.color = self.get_uv_at(hit_record.closest_point)
            }
        }
        return hit_record;
    }



    fn get_uv_at(&self, p: Vec3) -> Rgb {   // http://www.raytracerchallenge.com/bonus/texture-mapping.html
        let theta = (p.x).atan2(p.z);
        let radius = p.get_magnitude();
        let phi = (p.y / radius).acos();
        let raw_u = theta / (2.0 / PI);
        let u = 1.0 - (raw_u + 0.5);
        let v = 1.0 - (phi / PI);
        return self.material.uv_pattern_at(Point2D {x: u, y: v})
    }

}
