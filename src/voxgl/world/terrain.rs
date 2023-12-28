use bracket_noise::prelude::{FastNoise, NoiseType};
use crate::voxgl::world::chunk::ChunkData;

impl ChunkData {
    pub fn build_voxel_data(&mut self, chunk_world_pos: &cgmath::Vector3<f32>) {
        let mut noise = FastNoise::seeded(0);

        for (index, voxel) in self.voxels.iter_mut().enumerate() {
            let local_pos = Self::get_local_pos(index as i32);

            let x = chunk_world_pos.x + local_pos.x as f32;
            let y = chunk_world_pos.y + local_pos.y as f32;
            let z = chunk_world_pos.z + local_pos.z as f32;

            if  y >= 0f32 && y <= get_height(&mut noise, x, z) {
                voxel.set_density_frac(1f32);
            }
        }
    }
}

fn get_height(noise: &mut FastNoise, x: f32, y: f32) -> f32 {
    noise.set_noise_type(NoiseType::Simplex);
    noise.set_fractal_octaves(5);
    noise.set_frequency(0.02);
    
    clamped_normalize(noise.get_noise(x, y), 100f32)
}

fn clamped_normalize(value: f32, factor: f32) -> f32 {
    (value.max(-1.0).min(1.0) + 1.0) / 2.0 * factor
}