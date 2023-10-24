use wgpu::{BindGroupLayout, ShaderModule};

pub fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    color_format: wgpu::TextureFormat,
    depth_format: wgpu::TextureFormat,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    shader_module: wgpu::ShaderModule,
    label: &str,
) -> wgpu::RenderPipeline {

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(layout),

        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: "vs_main",
            buffers: vertex_layouts,
        },

        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: "fs_main",
            targets: &[wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }.into()],
        }),

        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            conservative: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
        },

        depth_stencil: Some(wgpu::DepthStencilState {
            format: depth_format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),

        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    render_pipeline
}

use std::borrow::Cow;

pub fn create_shader_module(device: &wgpu::Device, shader_str: &str, label: &str) -> ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_str)),
    })
}

pub fn create_bind_group_layout(
    device: &wgpu::Device, label: &str, binding_location: u32, visibility: wgpu::ShaderStages,
) -> BindGroupLayout {

    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: binding_location,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    })
}

pub fn create_pipeline_layout(
    device: &wgpu::Device, label: &str, bind_group_layouts: &[&BindGroupLayout]
) -> wgpu::PipelineLayout {

    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts,
        push_constant_ranges: &[],
    })
}