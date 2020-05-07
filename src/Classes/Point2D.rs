use crate::Classes::Window::Window;

pub struct Point2D {
    pub x: f32,
    pub y: f32
}

impl Point2D {
    pub fn as_buffer(&self, window: Window) -> u32 {
        return (window.width as f32 * self.y + self.x) as u32;
    }
}