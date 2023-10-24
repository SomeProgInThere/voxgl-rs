use bracket_noise::prelude::{FastNoise, NoiseType};
use crate::voxgl::world::chunk::ChunkData;

impl ChunkData {
    pub fn build_voxel_data(&mut self, chunk_world_pos: &cgmath::Vector3<f32>) {
        let mut n1 = FastNoise::seeded(0);
        n1.set_noise_type(NoiseType::Perlin);
        n1.set_fractal_octaves(3);
        n1.set_frequency(0.04);
        let h1 = 20f32;

        for (index, voxel) in self.voxels.iter_mut().enumerate() {
            let local_pos = Self::get_local_pos(index as i32);

            let x = chunk_world_pos.x + local_pos.x as f32;
            let y = chunk_world_pos.y + local_pos.y as f32;
            let z = chunk_world_pos.z + local_pos.z as f32;

            let height_sum = [
                clamped_normalize(n1.get_noise(x, z), h1),
            ].iter().sum::<f32>();

            if  y >= 0f32 && y <= height_sum {
                voxel.set_density_frac(1f32);
            }
        }
    }
}

fn clamped_normalize(value: f32, factor: f32) -> f32 {
    (value.max(-1.0).min(1.0) + 1.0) / 2.0 * factor
}