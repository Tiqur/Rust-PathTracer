use crate::Classes::Window::Window;

pub struct Point2D {
    pub x: i32,
    pub y: i32
}

impl Point2D {
    pub fn as_buffer(&self, window: Window) -> u32 {
        return (window.width as i32 * self.y + self.x) as u32;
    }
}