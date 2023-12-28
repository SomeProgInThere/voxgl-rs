use std::collections::{HashMap, VecDeque};
use anyhow::{*, Context};
use cgmath::{InnerSpace, Vector3};
use lifeguard::{StartingSize, Pool, pool};
use crate::voxgl::world::chunk::{ChunkData, ChunkMesh, SIZE};
use crate::voxgl::world::mesh_builder::{self};
use crate::voxgl::rendering::arena::MeshArena;
use crate::voxgl::world::voxel::Voxel;

pub const RENDER_DISTANCE: i32 = 8;

pub const MAX_DATA_LOAD: usize = 10_000;
pub const MAX_MESH_LOAD: usize = 10_000;

pub const MAX_DATA_LOAD_QUEUE: usize = 12;
pub const MAX_MESH_LOAD_QUEUE: usize = 8;
pub const MAX_DATA_UNLOAD_QUEUE: usize = 12;
pub const MAX_MESH_UNLOAD_QUEUE: usize = 8;

pub struct Chunks {
    chunk_data_map: HashMap<cgmath::Vector3<i32>, ChunkData>,
    chunk_mesh_map: HashMap<cgmath::Vector3<i32>, ChunkMesh>,

    chunk_data_pool: Pool<ChunkData>,
    chunk_mesh_pool: Pool<ChunkMesh>,

    chunk_data_load_queue: VecDeque<cgmath::Vector3<i32>>,
    chunk_mesh_load_queue: VecDeque<cgmath::Vector3<i32>>,
    chunk_data_unload_queue: VecDeque<cgmath::Vector3<i32>>,
    chunk_mesh_unload_queue: VecDeque<cgmath::Vector3<i32>>,

    render_distance: i32,
    pub position: cgmath::Vector3<f32>,
}

unsafe impl std::marker::Send for Chunks {}
unsafe impl std::marker::Sync for Chunks {}

impl Chunks {
    pub fn new() -> Self {
        let chunks = Self {
            chunk_data_map: HashMap::with_capacity(MAX_DATA_LOAD),
            chunk_mesh_map: HashMap::with_capacity(MAX_MESH_LOAD),
            
            chunk_data_pool: pool().with(StartingSize(MAX_DATA_LOAD)).build(),
            chunk_mesh_pool: pool().with(StartingSize(MAX_MESH_LOAD)).build(),
            
            chunk_data_load_queue: VecDeque::with_capacity(MAX_DATA_LOAD_QUEUE),
            chunk_mesh_load_queue: VecDeque::with_capacity(MAX_MESH_LOAD_QUEUE),
            chunk_data_unload_queue: VecDeque::with_capacity(MAX_DATA_UNLOAD_QUEUE),
            chunk_mesh_unload_queue: VecDeque::with_capacity(MAX_MESH_UNLOAD_QUEUE),
            
            position: cgmath::Vector3::<f32>::new(0., 0., 0.),
            render_distance: RENDER_DISTANCE,
        };
        chunks
    }

    pub fn build_chunk_data_in_queue(&mut self) {
        while let Some(chunk_pos) = self.chunk_data_load_queue.pop_front() {
            self.build_chunk_data(chunk_pos);
        }
    }

    pub fn try_get_voxel(&self, chunk_pos: &Vector3<i32>, local_pos: &Vector3<i32>) -> Result<&Voxel> {
        let mut chunk_pos = *chunk_pos;
        let mut local_pos = *local_pos;
        make_coords_valid(&mut chunk_pos, &mut local_pos);

        let chunk = self.chunk_data_map.get(&chunk_pos).context("no data")?;
        chunk.get_voxel(&local_pos).context("no voxel")
    }

    pub fn get_chunk_mesh_mut(&mut self, chunk_pos: &Vector3<i32>) -> Option<&mut ChunkMesh> {
        self.chunk_mesh_map.get_mut(chunk_pos)
    }

    pub fn build_chunk_data(&mut self, chunk_pos: Vector3<i32>) {
        let mut chunk = self.chunk_data_pool.detached();
        let chunk_world_pos = chunk_to_world(&chunk_pos);

        chunk.build_voxel_data(&chunk_world_pos);
        //println!("loaded chunk data at world pos: {:?}", chunk_world_pos);
        self.chunk_data_map.insert(chunk_pos, chunk);
    }

    pub fn build_chunk_meshes_in_queue(&mut self, device: &wgpu::Device, arena: &mut MeshArena) {
        while let Some(chunk_pos) = self.chunk_mesh_load_queue.pop_front() {
            if self.chunk_mesh_map.len() >= MAX_DATA_LOAD {
                return;
            }
            
            let chunk_mesh = self.chunk_mesh_pool.detached();
            self.chunk_mesh_map.insert(chunk_pos, chunk_mesh);

            //println!("building chunk mesh at: {:?}", chunk_pos);
            let chunk_world_pos = chunk_to_world(&chunk_pos);
            if mesh_builder::build_chunk_mesh(self, &chunk_pos, &chunk_world_pos, device, arena) {
                return;
            }
        }
    }

    pub fn is_chunk_busy(&self, chunk_pos: &Vector3<i32>) -> bool {
        self.chunk_data_map.contains_key(chunk_pos) || self.chunk_data_load_queue.contains(chunk_pos)
    }

    pub fn is_mesh_busy(&self, chunk_pos: &Vector3<i32>) -> bool {
        self.chunk_mesh_map.contains_key(chunk_pos) || self.chunk_mesh_load_queue.contains(chunk_pos)
    }

    pub fn in_range(&self, chunk_pos: Vector3<i32>) -> bool {
        let chunk_real_pos = chunk_to_world(&chunk_pos);
        let delta = self.position - chunk_real_pos;

        let distance_sq: f32 = delta.magnitude2().into();
        let render_dist = (self.render_distance as f32) * SIZE as f32;
        let render_distance_sq = render_dist * render_dist;

        distance_sq < render_distance_sq
    }

    pub fn update_load_mesh_queue(&mut self) {
        if self.chunk_mesh_map.len() >= MAX_MESH_LOAD || self.chunk_mesh_load_queue.len() >= MAX_MESH_LOAD_QUEUE {
            return;
        }

        for y in -self.render_distance..self.render_distance {
            for z in -self.render_distance..self.render_distance {
                for x in -self.render_distance..self.render_distance {
                    let current_chunk_pos = Vector3::<i32>::new(
                        (self.position.x / SIZE as f32) as i32,
                        (self.position.y / SIZE as f32) as i32,
                        (self.position.z / SIZE as f32) as i32,
                    );
                    let chunk_pos = current_chunk_pos + Vector3::<i32>::new(x, y, z);

                    if self.is_mesh_busy(&chunk_pos) {
                        continue;
                    }

                    let in_range = self.in_range(current_chunk_pos);
                    use cgmath::Vector3 as vec;
                    let adj_chunk_data_bad = [
                        -vec::<i32>::unit_x(),
                        vec::<i32>::unit_x(),
                        -vec::<i32>::unit_y(),
                        vec::<i32>::unit_y(),
                        -vec::<i32>::unit_z(),
                        vec::<i32>::unit_z(),
                    ]
                    .iter_mut()
                    .map(|v| *v + chunk_pos)
                    .any(|v| !self.chunk_data_map.contains_key(&v));

                    if in_range && !adj_chunk_data_bad {
                        self.chunk_mesh_load_queue.push_back(chunk_pos);

                        if self.chunk_mesh_load_queue.len() >= MAX_MESH_LOAD_QUEUE {
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn update_unload_data_queue(&mut self) {
        let current_chunk_pos = Vector3::<i32>::new(
            (self.position.x / SIZE as f32) as i32,
            (self.position.y / SIZE as f32) as i32,
            (self.position.z / SIZE as f32) as i32,
        );

        let outside = self.chunk_mesh_map.iter()
            .filter(|(p, _m)| {
                p.x < current_chunk_pos.x - self.render_distance
                    || p.x > current_chunk_pos.x + self.render_distance
                    || p.y < current_chunk_pos.y - self.render_distance
                    || p.y > current_chunk_pos.y + self.render_distance
                    || p.z < current_chunk_pos.z - self.render_distance
                    || p.z > current_chunk_pos.z + self.render_distance
            })
            .map(|(p, _m)| p)
            .collect::<Vec<_>>();

        for chunk_pos in outside {
            if self.chunk_data_unload_queue.contains(chunk_pos) {
                continue;
            }

            //println!("queueing chunk for data unload: {:?}", chunk_pos);
            self.chunk_data_unload_queue.push_back(*chunk_pos);
            if self.chunk_data_unload_queue.len() >= MAX_DATA_UNLOAD_QUEUE {
                return;
            }
        }
    }

    pub fn update_unload_mesh_queue(&mut self) {
        let current_chunk_pos = Vector3::<i32>::new(
            (self.position.x / SIZE as f32) as i32,
            (self.position.y / SIZE as f32) as i32,
            (self.position.z / SIZE as f32) as i32,
        );

        let outside = self.chunk_mesh_map.iter()
            .filter(|(p, _m)| {
                p.x < current_chunk_pos.x - self.render_distance
                    || p.x > current_chunk_pos.x + self.render_distance
                    || p.y < current_chunk_pos.y - self.render_distance
                    || p.y > current_chunk_pos.y + self.render_distance
                    || p.z < current_chunk_pos.z - self.render_distance
                    || p.z > current_chunk_pos.z + self.render_distance
            })
            .map(|(p, _m)| p)
            .collect::<Vec<_>>();

        for chunk_pos in outside {
            if self.chunk_mesh_unload_queue.contains(chunk_pos) {
                continue;
            }

            //println!("queueing chunk for mesh unload: {:?}", chunk_pos);
            self.chunk_mesh_unload_queue.push_back(*chunk_pos);
            if self.chunk_mesh_unload_queue.len() >= MAX_MESH_UNLOAD_QUEUE {
                return;
            }
        }
    }

    pub fn unload_data_queue(&mut self) {
        while let Some(chunk_pos) = self.chunk_data_unload_queue.pop_front() {
            if let Some(chunk_data) = self.chunk_data_map.remove(&chunk_pos) {
                //println!("unloading data at: {:?}", chunk_pos);
                self.chunk_data_pool.attach(chunk_data);
            }
        }
    }

    pub fn update_load_data_queue(&mut self) {
        if self.chunk_data_map.len() >= MAX_DATA_LOAD || self.chunk_data_load_queue.len() >= MAX_DATA_LOAD_QUEUE {
            return;
        }

        for y in -self.render_distance..self.render_distance {
            for z in -self.render_distance..self.render_distance {
                for x in -self.render_distance..self.render_distance {
                    let current_chunk_pos = Vector3::<i32>::new(
                        (self.position.x / SIZE as f32) as i32,
                        (self.position.y / SIZE as f32) as i32,
                        (self.position.z / SIZE as f32) as i32,
                    );

                    let chunk_pos = current_chunk_pos + Vector3::<i32>::new(x, y, z);
                    if self.is_chunk_busy(&chunk_pos) {
                        continue;
                    }

                    let in_range = self.in_range(current_chunk_pos);
                    if in_range {
                        self.chunk_data_load_queue.push_back(chunk_pos);
                    }

                    if self.chunk_data_load_queue.len() >= MAX_DATA_LOAD_QUEUE {
                        //println!("done");
                        return;
                    }
                }
            }
        }
    }

    pub fn unload_mesh_queue(&mut self, arena: &mut MeshArena) {
        while let Some(chunk_pos) = self.chunk_mesh_unload_queue.pop_front() {
            if let Some(chunk_mesh) = self.chunk_mesh_map.remove(&chunk_pos) {
                //println!("unloading mesh at: {:?}", chunk_pos);

                if let Some(v_buf_key) = chunk_mesh.vertex_buffer {
                    if let Some(v_buf) = arena.buffer.get_mut(v_buf_key) {
                        v_buf.destroy();
                    }
                    arena.buffer.remove(v_buf_key);
                }
                
                if let Some(i_buf_key) = chunk_mesh.index_buffer {
                    if let Some(i_buf) = arena.buffer.get_mut(i_buf_key) {
                        i_buf.destroy();
                    }
                    arena.buffer.remove(i_buf_key);
                }
                
                self.chunk_mesh_pool.attach(chunk_mesh);
            }
        }
    }

    pub fn draw<'a, 'b>(
        &mut self, render_pass: &mut wgpu::RenderPass<'a>, camera_bind_group: &'a wgpu::BindGroup, arena: &'a MeshArena
    ) -> Result<()> {

        for (_, chunk) in self.chunk_mesh_map.iter() {
            let v_buf_index = chunk.vertex_buffer.as_ref().context("no vertices")?;
            let i_buf_index = chunk.index_buffer.as_ref().context("no indices")?;

            let vertex_buffer = arena.buffer.get(*v_buf_index).context("no v_buf_index")?;
            let index_buffer = arena.buffer.get(*i_buf_index).context("no i_buf_index")?;
            let _ = draw_chunk(
                render_pass,
                chunk.index_count,
                camera_bind_group,
                vertex_buffer,
                index_buffer
            );
        }
        Ok(())
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.chunk_mesh_map.iter()
            .map(|(_, chunk)| chunk.vertex_count)
            .sum()
    }
}

pub fn adjacent_voxels<'a>(
    chunks: &'a Chunks, local_pos: Vector3<i32>, chunk_pos: &Vector3<i32>
) -> anyhow::Result<(&'a Voxel, &'a Voxel, &'a Voxel, &'a Voxel)> {

    let (x, y, z) = (local_pos.x, local_pos.y, local_pos.z);
    let voxel = chunks.try_get_voxel(chunk_pos, &Vector3::new(x, y, z))
        .context("no voxel")?;
    let back = chunks.try_get_voxel(chunk_pos, &Vector3::new(x, y, z - 1))
        .context("no back")?;
    let left = chunks.try_get_voxel(chunk_pos, &Vector3::new(x - 1, y, z))
        .context("no left")?;
    let bottom = chunks.try_get_voxel(chunk_pos, &Vector3::new(x, y - 1, z))
        .context("no bottom")?;

    Ok((voxel, back, left, bottom))
}

fn make_coords_valid(chunk_pos: &mut Vector3<i32>, local_pos: &mut Vector3<i32>) {
    let chunk_size = SIZE as i32;
    while local_pos.x < 0 {
        local_pos.x += chunk_size;
        chunk_pos.x -= 1;
    }
    while local_pos.x > chunk_size {
        local_pos.x -= chunk_size;
        chunk_pos.x += 1;
    }
    while local_pos.y < 0 {
        local_pos.y += chunk_size;
        chunk_pos.y -= 1;
    }
    while local_pos.y > chunk_size {
        local_pos.y -= chunk_size;
        chunk_pos.y += 1;
    }
    while local_pos.z < 0 {
        local_pos.z += chunk_size;
        chunk_pos.z -= 1;
    }
    while local_pos.z > chunk_size {
        local_pos.z -= chunk_size;
        chunk_pos.z += 1;
    }
}

fn chunk_to_world(chunk_pos: &Vector3<i32>) -> Vector3<f32> {
    Vector3::<f32>::new(
        chunk_pos.x as f32 * SIZE as f32,
        chunk_pos.y as f32 * SIZE as f32,
        chunk_pos.z as f32 * SIZE as f32,
    )
}

fn draw_chunk<'a, 'b>(
    render_pass: &mut wgpu::RenderPass<'a>, i_count: u32, camera_bind_group: &'a wgpu::BindGroup, v_buf: &'a wgpu::Buffer, i_buf: &'a wgpu::Buffer
) -> anyhow::Result<()> {

    render_pass.set_vertex_buffer(0, v_buf.slice(..));
    render_pass.set_index_buffer(i_buf.slice(..), wgpu::IndexFormat::Uint32);
    render_pass.set_bind_group(0, &camera_bind_group, &[]);
    render_pass.draw_indexed(0..i_count, 0, 0..1);

    Ok(())
}