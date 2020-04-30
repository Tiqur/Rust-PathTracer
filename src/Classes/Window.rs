pub struct Window {
    pub width: usize,
    pub height: usize,
    pub primary_buffer: Vec<u32>,
    pub secondary_buffer: Vec<u32>
}

impl Window {
    pub fn swap_buffers(&mut self) {
        self.primary_buffer = self.secondary_buffer.clone()
    }
}