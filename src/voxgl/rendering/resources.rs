
pub struct VRAMResources {
    pub arena_buffer: generational_arena::Arena<wgpu::Buffer>,
}

impl VRAMResources {
    pub fn new() -> Self {
        Self {
            arena_buffer: generational_arena::Arena::with_capacity(32)
        }
    }
}