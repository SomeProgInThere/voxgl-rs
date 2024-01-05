
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoxelId {
    Grass, Empty, Sand, Dirt, Stone, Snow
}

impl VoxelId {
    pub fn get_color(&self) -> wgpu::Color {
        match self {
            VoxelId::Empty => wgpu::Color { r: 0.00, g: 0.00, b: 0.00,  a: 1.0 },
            VoxelId::Grass => wgpu::Color { r: 0.21, g: 0.80, b: 0.01,  a: 1.0 },
            VoxelId::Sand  => wgpu::Color { r: 1.00, g: 0.88, b: 0.31,  a: 1.0 },
            VoxelId::Dirt  => wgpu::Color { r: 0.29, g: 0.20, b: 0.15,  a: 1.0 },
            VoxelId::Stone => wgpu::Color { r: 0.55, g: 0.55, b: 0.55,  a: 1.0 },
            VoxelId::Snow  => wgpu::Color { r: 0.97, g: 0.95, b: 0.97,  a: 1.0 },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Voxel {
    pub id: VoxelId,
}

impl Voxel {
    pub fn new() -> Self {
        Self { id: VoxelId::Empty }
    }

    pub fn is_solid(&self) -> bool {
        self.id != VoxelId::Empty 
    }
}
