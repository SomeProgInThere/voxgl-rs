use std::collections::{HashMap, VecDeque};
use anyhow::Context;
use cgmath::{InnerSpace, Vector3};
use debug_print::debug_println;
use lifeguard::{StartingSize, Pool, pool};
use crate::voxgl::world::chunk::{Chunk, SIZE};
use crate::voxgl::world::mesh_builder::build_chunk_mesh;
use crate::voxgl::world::rendering::resources::VRAMResources;
use crate::voxgl::world::voxel::Voxel;

pub const RENDER_DISTANCE: i32 = 4;
pub const MAX_CHUNK_CAPACITY: usize = 10_000;
pub const MAX_CHUNK_LOAD_QUEUE: usize = 8;
pub const MAX_CHUNK_UNLOAD_QUEUE: usize = 8;

pub struct Chunks {
    chunk_map: HashMap<Vector3<i32>, Chunk>,
    chunk_pool: Pool<Chunk>,
    chunk_load_queue: VecDeque<Vector3<i32>>,
    chunk_unload_queue: VecDeque<Vector3<i32>>,

    pub position: Vector3<f32>,
    render_distance: i32,
}

impl Chunks {
    pub fn new() -> Self {
        Self {
            chunk_map: HashMap::with_capacity(MAX_CHUNK_CAPACITY),
            chunk_pool: pool().with(StartingSize(MAX_CHUNK_CAPACITY)).build(),
            chunk_load_queue: VecDeque::with_capacity(MAX_CHUNK_LOAD_QUEUE),
            chunk_unload_queue: VecDeque::with_capacity(MAX_CHUNK_UNLOAD_QUEUE),

            position: Vector3::<f32>::new(0.0, 0.0, 0.0),
            render_distance: RENDER_DISTANCE,
        }
    }

    pub fn get_chunk_mut(&mut self, chunk_pos: &Vector3<i32>) -> Option<&mut Chunk> {
        self.chunk_map.get_mut(chunk_pos)
    }

    pub fn try_get_voxel(&self, chunk_pos: &Vector3<i32>, local_pos: &Vector3<i32>) -> anyhow::Result<&Voxel> {
        let mut chunk_pos = *chunk_pos;
        let mut local_pos = *local_pos;
        make_coords_valid(&mut chunk_pos, &mut local_pos);

        let chunk = self.chunk_map.get(&chunk_pos).context("no chunk")?;
        chunk.get_voxel(&local_pos).context("no voxel")
    }

    pub fn build_chunk_in_queue(&mut self, device: &wgpu::Device, resources: &mut VRAMResources) {
        while let Some(chunk_pos) = self.chunk_load_queue.pop_front() {
            let mut chunk = self.chunk_pool.detached();
            let chunk_world_pos = chunk_to_world(&chunk_pos);
            chunk.build_voxel_data(&chunk_world_pos);

            debug_println!("[Chunk] load data at: {:?}", chunk_world_pos);
            if self.chunk_map.len() >= MAX_CHUNK_CAPACITY {
                return;
            }

            self.chunk_map.insert(chunk_pos, chunk);
            debug_println!("[Chunk] build at: {:?}", chunk_pos);
            if build_chunk_mesh(self, &chunk_pos, &chunk_world_pos, device, resources) {
                return;
            }
        }
    }

    pub fn is_chunk_busy(&self, chunk_pos: &Vector3<i32>) -> bool {
        self.chunk_map.contains_key(chunk_pos) || self.chunk_load_queue.contains(chunk_pos)
    }

    pub fn in_range(&self, chunk_pos: &Vector3<i32>) -> bool {
        let chunk_real_pos = chunk_to_world(chunk_pos);
        let delta = self.position - chunk_real_pos;
        let distance_sq: f32 = delta.magnitude2();

        let render_dist = (self.render_distance as f32) * SIZE as f32;
        let render_distance_sq = render_dist * render_dist;
        distance_sq < render_distance_sq
    }

    pub fn update_chunk_load_queue(&mut self) {
        if self.chunk_map.len() >= MAX_CHUNK_CAPACITY || self.chunk_load_queue.len() >= MAX_CHUNK_LOAD_QUEUE {
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
                    let chunk_busy = self.is_chunk_busy(&chunk_pos);
                    if chunk_busy {
                        continue;
                    }

                    let in_range = self.in_range(&current_chunk_pos);

                    if in_range {
                        self.chunk_load_queue.push_back(chunk_pos);
                        let adjust_bad_chunk_data = [
                            -Vector3::<i32>::unit_x(), Vector3::<i32>::unit_x(),
                            -Vector3::<i32>::unit_y(), Vector3::<i32>::unit_y(),
                            -Vector3::<i32>::unit_z(), Vector3::<i32>::unit_z(),
                        ].iter_mut()
                            .map(|v| *v + chunk_pos)
                            .any(|v| !self.chunk_map.contains_key(&v));

                        if !adjust_bad_chunk_data {
                            self.chunk_load_queue.push_back(chunk_pos);
                        }

                        if self.chunk_load_queue.len() >= MAX_CHUNK_LOAD_QUEUE {
                            debug_println!("done!");
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn update_chunk_unload_queue(&mut self) {
        let current_chunk_pos = Vector3::<i32>::new(
            (self.position.x / SIZE as f32) as i32,
            (self.position.y / SIZE as f32) as i32,
            (self.position.z / SIZE as f32) as i32,
        );

        let outside = self.chunk_map.iter()
            .filter(|(p, _)| {
                p.x < current_chunk_pos.x - self.render_distance
                    || p.x > current_chunk_pos.x + self.render_distance
                    || p.y < current_chunk_pos.y - self.render_distance
                    || p.y > current_chunk_pos.y + self.render_distance
                    || p.z < current_chunk_pos.z - self.render_distance
                    || p.z > current_chunk_pos.z + self.render_distance
                }
            )
            .map(|(p, _)| p)
            .collect::<Vec<_>>();

        for chunk_pos in outside {
            if self.chunk_unload_queue.contains(chunk_pos) {
                continue;
            }
            debug_println!("[Queue] update chunk unload: {:?}", chunk_pos);

            self.chunk_unload_queue.push_back(*chunk_pos);
            if self.chunk_unload_queue.len() >= MAX_CHUNK_UNLOAD_QUEUE {
                return;
            }
        }
    }

    pub fn unload_chunk_queue(&mut self, resources: &mut VRAMResources) {
        while let Some(chunk_pos) = self.chunk_unload_queue.pop_front() {
            if let Some(chunk) = self.chunk_map.remove(&chunk_pos) {

                debug_println!("[Queue] chunk unload at: {:?}", chunk_pos);
                if let Some(v_buf_key) = chunk.vertex_buffer {
                    if let Some(v_buf) = resources.arena_buffer.get_mut(v_buf_key) {
                        v_buf.destroy();
                    }
                    resources.arena_buffer.remove(v_buf_key);
                }

                if let Some(i_buf_key) = chunk.index_buffer {
                    if let Some(i_buf) = resources.arena_buffer.get_mut(i_buf_key) {
                        i_buf.destroy();
                    }
                    resources.arena_buffer.remove(i_buf_key);
                }

                self.chunk_pool.attach(chunk);
            }
        }
    }

    pub fn draw<'a, 'b>(
        &mut self, render_pass: &mut wgpu::RenderPass<'a>, camera_bind_group: &'a wgpu::BindGroup, resources: &'a VRAMResources
    ) -> anyhow::Result<()> {

        for (_, chunk) in self.chunk_map.iter() {
            let v_buf_index = chunk.vertex_buffer.as_ref().context("no vertices")?;
            let i_buf_index = chunk.index_buffer.as_ref().context("no indices")?;

            let vertex_buffer = resources.arena_buffer.get(*v_buf_index).context("no v_buf_index")?;
            let index_buffer = resources.arena_buffer.get(*i_buf_index).context("no i_buf_index")?;
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
        self.chunk_map.iter()
            .map(|(_, chunk)| chunk.vertex_count)
            .sum()
    }
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