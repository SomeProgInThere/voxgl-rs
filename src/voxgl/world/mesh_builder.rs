use anyhow::Context;
use cgmath::Vector3;
use debug_print::debug_println;
use wgpu::util::DeviceExt;
use crate::voxgl::world::{
    chunk::SIZE,
    quad::{Quad, Face},
    rendering::{vertex::Vertex, resources::VRAMResources},
    voxel::Voxel,
};
use crate::voxgl::world::chunks::Chunks;

pub fn build_chunk_mesh(
    chunks: &mut Chunks, chunk_pos: &Vector3<i32>, chunk_world_pos: &Vector3<f32>, device: &wgpu::Device, resources: &mut VRAMResources
) -> bool {

    let chunk_size = SIZE as i32;
    let mut quads = Vec::<Quad>::new();

    for x in 0..chunk_size {
        for y in 0..chunk_size {
            for z in 0..chunk_size {

                let voxel_local_pos = Vector3::<f32>::new(x as f32, y as f32, z as f32);
                let voxel_world_pos = chunk_world_pos + voxel_local_pos;
                if let Ok(
                    (voxel, back, left, bottom)
                ) = adjacent_voxels(chunks, Vector3 {x, y, z}, chunk_pos) {
                    process_voxel(
                        &voxel,
                        voxel_world_pos,
                        &left,
                        &bottom,
                        &back,
                        &mut quads
                    );
                }
            }
        }
    }
    if quads.is_empty() {
        debug_println!("Empty quads!");
    }

    let mut vertices = Vec::<Vertex>::new();
    let mut indices = Vec::<u32>::new();

    process_quads(&quads, &mut vertices, &mut indices);
    if let Some(chunk) = chunks.get_chunk_mut(chunk_pos) {
        let v_count = vertices.len() as u32;
        let i_count = indices.len() as u32;

        let (v_buf, i_buf) = construct_buffers(vertices, indices, device);
        let v_buf = resources.arena_buffer.insert(v_buf);
        let i_buf = resources.arena_buffer.insert(i_buf);
        chunk.update_mesh_buffers(v_buf, i_buf, v_count, i_count);
        return i_count != 0;
    }
    false
}

fn adjacent_voxels<'a>(
    chunks: &'a Chunks, local_pos: Vector3<i32>, chunk_pos: &Vector3<i32>
) -> anyhow::Result<(&'a Voxel, &'a Voxel, &'a Voxel, &'a Voxel)> {

    let (x, y, z) = (local_pos.x, local_pos.y, local_pos.z);
    let voxel = chunks.try_get_voxel(chunk_pos, &Vector3 { x, y, z })
        .context("no voxel")?;
    let back = chunks.try_get_voxel(chunk_pos, &Vector3 { x, y, z: z - 1 })
        .context("no back")?;
    let left = chunks.try_get_voxel(chunk_pos, &Vector3 { x: x - 1, y, z })
        .context("no left")?;
    let bottom = chunks.try_get_voxel(chunk_pos, &Vector3 { x, y: y - 1, z })
        .context("no bottom")?;

    Ok((voxel, back, left, bottom))
}

fn process_voxel(
    voxel: &Voxel, voxel_pos: Vector3<f32>, left: &Voxel, bottom: &Voxel, back: &Voxel, quads: &mut Vec<Quad>
) {
    match voxel.is_solid() {
        true => {
            // voxel is solid
            if !left.is_solid() {
                quads.push(Quad::from_face(Face::Left, voxel_pos));
            }
            if !bottom.is_solid() {
                quads.push(Quad::from_face(Face::Bottom, voxel_pos));
            }
            if !back.is_solid() {
                quads.push(Quad::from_face(Face::Back, voxel_pos));
            }
        }

        false => {
            // voxel is not solid
            if left.is_solid() {
                quads.push(Quad::from_face(Face::Right, voxel_pos));
            }
            if bottom.is_solid() {
                quads.push(Quad::from_face(Face::Top, voxel_pos));
            }
            if back.is_solid() {
                quads.push(Quad::from_face(Face::Front, voxel_pos));
            }
        }
    }
}

fn process_quads(quads: &Vec<Quad>, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    let mut v_index = 0;
    for quad in quads {
        for index in 0..4 {
            vertices.push(Vertex {
                position: quad.vertices[index].into(),
                color: quad.color.into(),
            });
        }

        indices.push(v_index);
        indices.push(v_index + 1);
        indices.push(v_index + 2);
        indices.push(v_index);
        indices.push(v_index + 2);
        indices.push(v_index + 3);

        v_index += 4;
    }
}

fn construct_buffers(vertices: Vec<Vertex>, indices: Vec<u32>, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("voxel_chunk_vertices"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("voxel_chunk_indices"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    (vertex_buffer, index_buffer)
}
