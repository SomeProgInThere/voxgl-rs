use cgmath::{InnerSpace, SquareMatrix};
use wgpu::util::DeviceExt;
use crate::voxgl::camera::projection::Projection;
use crate::voxgl::world::rendering::utils;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (camera.projection.calc_matrix() * camera.calc_matrix()).into();
    }
}

pub struct Camera {
    pub position: cgmath::Point3<f32>,
    pub yaw: cgmath::Rad<f32>,
    pub pitch: cgmath::Rad<f32>,
    pub projection: Projection,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
}

impl Camera {
    pub fn new(
        position: cgmath::Point3<f32>,
        yaw: cgmath::Rad<f32>,
        pitch: cgmath::Rad<f32>,
        aspect: f32,
        v_fov: cgmath::Deg<f32>,
        z_near: f32,
        z_far: f32,
        uniform: &CameraUniform,
        device: &wgpu::Device,
    ) -> Self {

        let projection = Projection::new(aspect, v_fov, z_near, z_far);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera_buffer"),
            contents: bytemuck::cast_slice(&[*uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let layout = utils::create_bind_group_layout(
            &device,
            "camera_layout",
            0,
            wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
            projection,
            buffer,
            bind_group,
            layout,
        }
    }

    pub fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();
        let matrix = cgmath::Matrix4::look_to_rh(
            self.position,
            cgmath::Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            cgmath::Vector3::unit_y(),
        );
        matrix
    }
}
