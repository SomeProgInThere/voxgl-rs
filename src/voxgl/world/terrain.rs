use bracket_noise::prelude::{FastNoise, NoiseType};
use crate::voxgl::world::chunk::ChunkData;

use super::voxel::VoxelId;

impl ChunkData {
    pub fn build_voxel_data(&mut self, chunk_world_pos: &cgmath::Vector3<f32>) {
        let mut noise = FastNoise::seeded(0);

        for (index, voxel) in self.voxels.iter_mut().enumerate() {
            let local_pos = Self::get_local_pos(index as i32);

            let x = chunk_world_pos.x + local_pos.x as f32;
            let y = chunk_world_pos.y + local_pos.y as f32;
            let z = chunk_world_pos.z + local_pos.z as f32;

            let height = get_height(&mut noise, x, z, 64f32);
            
            if  y >= height - 1f32 && y <= height {
                voxel.set_new_id(VoxelId::Grass);
            } 
            if  y >= height - 4f32 && y <= height - 1f32 {
                voxel.set_new_id(VoxelId::Dirt);
            } 
            if  y >= 0f32 && y <= 10f32 {
                voxel.set_new_id(VoxelId::Stone);
            }
        }
    }
}

fn get_height(noise: &mut FastNoise, x: f32, y: f32, max: f32) -> f32 {
    noise.set_noise_type(NoiseType::Simplex);
    noise.set_fractal_octaves(4);
    noise.set_frequency(0.01);
    
    let height_mapped = (noise.get_noise(x, y) + 1.0) / 2.0;
    let height_extrapolated = height_mapped * (max - 1f32) + 1f32;
    
    height_extrapolated
}