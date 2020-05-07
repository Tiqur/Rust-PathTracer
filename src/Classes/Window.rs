use crate::Classes::Rgb::Rgb;
use crate::Classes::Point2D::Point2D;

pub struct Window {
    pub width: usize,
    pub height: usize,
    pub primary_buffer: Vec<u32>,
    pub secondary_buffer: Vec<u32>
}

impl Window {
    // copies secondary buffer to primary buffer
    pub fn swap_buffers(&mut self) {
        self.primary_buffer = self.secondary_buffer.clone()
    }

    // sets pixel color for specified position on window ( x, y )
    pub fn dot(&mut self, point: Point2D, color: &Rgb) {
        let c = color.to_int();
        let x = (point.x - 1.0) as i32;
        let y = (point.y - 1.0) as i32;

        let index = (y * self.width as i32 + x) as usize;
        if x < self.width as i32 && x > 0 && y < self.height as i32 && y > 0 {
            self.secondary_buffer[index] = c;
        }

    }

    // draws line from point A to point B
    pub fn line(&mut self, p1: Point2D, p2: Point2D, color: &Rgb) {

        let mut x0 = p1.x as i32;
        let mut y0 = p1.y as i32;
        let x1 = p2.x as i32;
        let y1 = p2.y as i32;

        let dx = i32::abs(x1 - x0);
        let sx = if x0 < x1 {1} else {-1};
        let dy = -i32::abs(y1 - y0);
        let sy = if y0 < y1 {1} else {-1};
        let mut err = dx + dy;

        loop {
            if x0 == x1 && y0 == y1 {break};

            self.dot(Point2D { x: x0 as f32, y: y0 as f32 }, color);

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }

    }

    // sets background to specified color
    pub fn set_background(&mut self, color: Rgb) {
        for i in self.secondary_buffer.iter_mut() {
            *i = color.to_int();
        }
    }

    // same as set_background but more efficient
    pub fn set_buffer(&mut self, vec: Vec<u32>) {
        self.secondary_buffer = vec.clone()
    }
}