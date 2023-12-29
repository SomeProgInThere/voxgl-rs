use crate::voxgl::texture;
use crate::voxgl::rendering::{utils, vertex::Vertex};

pub fn create_voxel_pipeline(device: &wgpu::Device, layouts: &[&wgpu::BindGroupLayout]) -> wgpu::RenderPipeline {
    let shader_module = utils::create_shader_module(
        &device,
        include_str!("voxel.wgsl"),
        "voxel_shader",
    );

    let pipeline_layout = utils::create_pipeline_layout(
        &device, "voxel_pipeline", layouts
    );

    let render_pipeline = utils::create_render_pipeline(
        &device,
        &pipeline_layout,
        wgpu::TextureFormat::Rgba16Float,
        texture::Texture::DEPTH_FORMAT,
        &[Vertex::desc()],
        shader_module,
        "voxel_pipeline",
    );
    render_pipeline
}
