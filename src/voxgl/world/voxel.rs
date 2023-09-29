
#[derive(Debug, Copy, Clone)]
pub struct Voxel {
    density: u8,
}

impl Voxel {
    pub fn new(density: u8) -> Self {
        Self { density }
    }

    pub fn is_solid(&self) -> bool {
        self.density > 0
    }

    pub fn set_density_frac(&mut self, frac: f32) {
        self.density = (frac * 255f32) as u8;
    }
}
