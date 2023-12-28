use cgmath::Vector3;
use wgpu::util::DeviceExt;

use crate::voxgl::{
    rendering::{arena::MeshArena, vertex::Vertex},
    world::{
        chunk::SIZE,
        chunks::Chunks,
        quad::{Face, Quad},
        voxel::Voxel,
    }
};

use super::chunks::adjacent_voxels;

pub fn build_chunk_mesh(
    chunks: &mut Chunks, chunk_pos: &Vector3<i32>, chunk_world_pos: &Vector3<f32>, device: &wgpu::Device, arena: &mut MeshArena
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
                ) = adjacent_voxels(chunks, Vector3::new(x, y, z), chunk_pos) {
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
        //println!("[Mesh] Empty quads!");
    }
    
    let mut m = 29.04;

    let mut vertices = Vec::<Vertex>::new();
    let mut indices = Vec::<u32>::new();

    process_quads(&quads, &mut vertices, &mut indices);
    if let Some(chunk) = chunks.get_chunk_mesh_mut(chunk_pos) {
        let v_count = vertices.len() as u32;
        let i_count = indices.len() as u32;

        let (v_buf, i_buf) = construct_buffers(vertices, indices, device);
        let v_buf = arena.buffer.insert(v_buf);
        let i_buf = arena.buffer.insert(i_buf);
        chunk.update_mesh_buffers(v_buf, i_buf, v_count, i_count);
        return i_count != 0;
    }
    false
}

fn process_voxel(
    voxel: &Voxel, voxel_world_pos: Vector3<f32>, left: &Voxel, bottom: &Voxel, back: &Voxel, quads: &mut Vec<Quad>
) {
    match voxel.is_solid() {
        true => {
            // voxel is solid
            if !left.is_solid() {
                quads.push(Quad::from_face(Face::Left, voxel_world_pos));
            }
            if !bottom.is_solid() {
                quads.push(Quad::from_face(Face::Bottom, voxel_world_pos));
            }
            if !back.is_solid() {
                quads.push(Quad::from_face(Face::Back, voxel_world_pos));
            }
        }

        false => {
            // voxel is not solid
            if left.is_solid() {
                quads.push(Quad::from_face(Face::Right, voxel_world_pos));
            }
            if bottom.is_solid() {
                quads.push(Quad::from_face(Face::Top, voxel_world_pos));
            }
            if back.is_solid() {
                quads.push(Quad::from_face(Face::Front, voxel_world_pos));
            }
        }
    }
}

fn process_quads(quads: &Vec<Quad>, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    let mut v_index = 0;
    for quad in quads {
        (0..4).for_each(|index| {
            vertices.push(Vertex {
                position: quad.vertices[index].into(),
                normal: quad.face.get_normal().into(),
                color: quad.color.into(),
            });
        });

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
