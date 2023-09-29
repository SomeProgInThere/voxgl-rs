use bracket_noise::prelude::{FastNoise, NoiseType};
use cgmath::Vector3;
use crate::voxgl::world::voxel::Voxel;
use generational_arena::Index;

pub const SIZE: usize = 16;
// pub const MAX_HEIGHT: usize = 128;

lazy_static::lazy_static! {
    pub static ref BIT_SIZE: i32 = (SIZE as f32).log2() as i32;
}

pub struct Chunk {
    // Chunk Data
    pub voxels: [Voxel; SIZE * SIZE * SIZE],
    // Chunk Mesh
    pub vertex_buffer: Option<Index>,
    pub index_buffer: Option<Index>,
    pub index_count: u32,
    pub vertex_count: u32,
}

impl lifeguard::Recycleable for Chunk {
    fn new() -> Self {
        Chunk::new()
    }

    fn reset(&mut self) {
        for voxel in self.voxels.iter_mut() {
            voxel.set_density_frac(0f32);
        }
        self.vertex_buffer = None;
        self.index_buffer = None;
        self.index_count = 0;
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            voxels: [Voxel::new(0); SIZE * SIZE * SIZE],
            vertex_buffer: None,
            index_buffer: None,
            vertex_count: 0,
            index_count: 0,
        }
    }

    pub fn update_mesh_buffers(&mut self, v_buf: Index, i_buf: Index, v_count: u32, i_count: u32) {
        self.vertex_buffer = Some(v_buf);
        self.index_buffer = Some(i_buf);
        self.vertex_count = v_count;
        self.index_count = i_count;
    }

    pub fn build_voxel_data(&mut self, chunk_world_pos: &Vector3<f32>) {
        let mut noise = FastNoise::seeded(1337);
        noise.set_noise_type(NoiseType::Simplex);
        noise.set_frequency(0.02);
        noise.set_fractal_octaves(5);

        for (index, voxel) in self.voxels.iter_mut().enumerate() {
            let local_pos = get_local_pos(index as i32);

            let x = chunk_world_pos.x + local_pos.x as f32;
            let y = chunk_world_pos.y + local_pos.y as f32;
            let z = chunk_world_pos.z + local_pos.z as f32;

            let noise_value = noise.get_noise3d(x, y, z);
            // let clamped_noise_value = noise_value.max(-1.0).min(1.0);
            // let height = ((clamped_noise_value + 1.0) / 2.0 * MAX_HEIGHT as f32) as u32;
            if noise_value > 0.0f32 {
                voxel.set_density_frac(1f32);
            }
        }
    }

    pub fn get_voxel(&self, pos: &Vector3<i32>) -> Option<&Voxel> {
        self.voxels.get(get_index(pos))
    }
}

pub fn get_index(pos: &Vector3<i32>) -> usize {
    (pos.z | (pos.y << *BIT_SIZE) | (pos.x << (*BIT_SIZE * 2))) as usize
}

pub fn get_local_pos(index: i32) -> Vector3<i32> {
    Vector3 {
        x: (index as f32 / (SIZE * SIZE) as f32) as i32,
        y: ((index as f32 / SIZE as f32) % SIZE as f32) as i32,
        z: (index as f32 % SIZE as f32) as i32,
    }
}
