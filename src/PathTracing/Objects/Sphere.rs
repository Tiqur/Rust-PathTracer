use crate::Classes::Vec3::Vec3;
use crate::PathTracing::Traits::Shape::Shape;
use crate::Classes::Rgb::Rgb;
use crate::PathTracing::Ray::Ray;
use crate::PathTracing::HitRecord::HitRecord;

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32
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
            }
        }
        return hit_record;
    }
}