use opensimplex_noise_rs::OpenSimplexNoise;

use crate::voxgl::world::chunk::ChunkData;

use super::{voxel::VoxelId, chunk::CHUNK_SIZE};

impl ChunkData {
    pub fn build_voxel_data(&mut self, chunk_world_pos: &cgmath::Vector3<f32>) {
        let generator = OpenSimplexNoise::new(None);

        for (index, voxel) in self.voxels.iter_mut().enumerate() {
            let local_pos = Self::get_local_pos(index as i32);

            let x = chunk_world_pos.x + local_pos.x as f32;
            let y = (chunk_world_pos.y + local_pos.y as f32) as i32;
            let z = chunk_world_pos.z + local_pos.z as f32;
            
            let height = get_height(&generator, x as f64, z as f64);
            
            if y < height - 1 {
                voxel.id = VoxelId::Stone;
            } 
            if y >= 8 && y < height {
                voxel.id = VoxelId::Grass;
            }
            if y >= 6 && y < height - 8 {
                voxel.id = VoxelId::Sand;
            }
        }
    }
}

fn get_height(generator: &OpenSimplexNoise, x: f64, z: f64) -> i32 {
    let mut a1 = CHUNK_SIZE as f64 * 2.0;
    let (a2, a4, a8) = (a1 * 0.5, a1 * 0.25, a1 * 0.125);

    let f1 = 0.005;
    let (f2, f4, f8) = (f1 * 2.0, f1 * 4.0, f1 * 8.0);

    if generator.eval_2d(x * 0.1, z * 0.1) < 0.0 {
        a1 /= 1.04;
    }

    let mut height = 0.0;
    height += generator.eval_2d(x * f1, z * f1) * a1 + a1;
    height += generator.eval_2d(x * f2, z * f2) * a2 - a2;
    height += generator.eval_2d(x * f4, z * f4) * a4 + a4;
    height += generator.eval_2d(x * f8, z * f8) * a8 - a8;

    height.max(1.0) as i32
}