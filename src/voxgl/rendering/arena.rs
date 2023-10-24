
pub struct MeshArena {
    pub buffer: generational_arena::Arena<wgpu::Buffer>,
}

impl MeshArena {
    pub fn new() -> Self {
        Self {
            buffer: generational_arena::Arena::with_capacity(32)
        }
    }
}