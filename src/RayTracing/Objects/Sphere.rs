use crate::Classes::Vec3::Vec3;
use crate::RayTracing::Traits::Shape::Shape;
use crate::Classes::Rgb::Rgb;
use crate::RayTracing::Ray::Ray;
use crate::RayTracing::HitRecord::HitRecord;

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32
}

impl Shape for Sphere {
    fn intersection(&self, ray: Ray, record_distance: f32) -> HitRecord {
        let mut hit_record = HitRecord{..Default::default()};
        let oc = ray.origin - self.pos;
        let a = ray.direction.get_magnitude_squared();
        let b = ray.direction.dot(oc);
        let c = oc.get_magnitude_squared() - (self.radius * self.radius);

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {  // if intersection

            let distance = (-b - discriminant.sqrt()) / (2.0 * a);

            if (distance < record_distance || !hit_record.hit) && distance > 0.0 {  // if no previous section or if current is closer and valid distance

                hit_record.hit = true;

                let point = ray.get_point(distance);
                let normal = point.to_unit_vector();

                hit_record.distance = distance;
                hit_record.closest_point = point;

                if ray.direction.dot(normal) > 0.0 {
                    hit_record.normal = -normal;
                    hit_record.front_face = false;
                } else {
                    hit_record.normal = normal;
                    hit_record.front_face = true;
                }

            }

        } else {
            hit_record.hit = false;  // ray doesn't intersect
        }
        return hit_record;
    }
}