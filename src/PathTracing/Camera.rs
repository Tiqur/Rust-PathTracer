use crate::Classes::Vec3::Vec3;
use crate::PathTracing::Ray::Ray;
use crate::Classes::Window::Window;

pub struct Camera {
    pub pos: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32
}

impl Camera {

    pub fn get_ray(&self, x: f32, y: f32, window: &Window) -> Ray {
        let fov_adj = (self.fov.to_radians() / 2.0).tan();  // this can be optimized
        let sensor_x = ((((x as f32 + 0.5) / window.width as f32) * 2.0 - 1.0) * self.aspect_ratio) * fov_adj;
        let sensor_y = (1.0 - ((y as f32 + 0.5) / window.height as f32) * 2.0) * fov_adj;

        return Ray { origin: self.pos, direction: Vec3 {
            x: sensor_x,
            y: sensor_y,
            z: 1.0
        }.to_unit_vector() };
    }
}