use cgmath::Vector3;
use crate::voxgl::world::voxel::Voxel;
use generational_arena::Index;

use super::voxel::VoxelId;

pub const SIZE: usize = 16;

lazy_static::lazy_static! {
    pub static ref BIT_SIZE: i32 = (SIZE as f32).log2() as i32;
}

pub struct ChunkData {
    pub voxels: [Voxel; SIZE * SIZE * SIZE],
}

impl lifeguard::Recycleable for ChunkData {
    fn new() -> Self {
        ChunkData::new()
    }

    fn reset(&mut self) {
        for voxel in self.voxels.iter_mut() {
            voxel.set_new_id(VoxelId::Empty);
        }
    }
}

impl ChunkData {
    pub fn new() -> Self {
        Self {
            voxels: [Voxel::new(); SIZE * SIZE * SIZE],
        }
    }

    pub fn get_voxel(&self, pos: &Vector3<i32>) -> Option<&Voxel> {
        self.voxels.get(Self::get_index(pos.x, pos.y, pos.z))
    }

    pub fn get_index(x: i32, y: i32, z: i32) -> usize {
        (z | (y << *BIT_SIZE) | (x << (*BIT_SIZE * 2))) as usize
    }

    pub fn get_local_pos(index: i32) -> Vector3<i32> {
        Vector3 {
            x: (index as f32 / (SIZE * SIZE) as f32) as i32,
            y: ((index as f32 / SIZE as f32) % SIZE as f32) as i32,
            z: (index as f32 % SIZE as f32) as i32,
        }
    }
}

pub struct ChunkMesh {
    pub vertex_buffer: Option<Index>,
    pub index_buffer: Option<Index>,
    pub index_count: u32,
    pub vertex_count: u32,
}

impl lifeguard::Recycleable for ChunkMesh {
    fn new() -> Self {
        ChunkMesh::new()
    }

    fn reset(&mut self) {
        self.vertex_buffer = None;
        self.index_buffer = None;
        self.index_count = 0;
    }
}

impl ChunkMesh {
    pub fn new() -> Self {
        Self {
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
}