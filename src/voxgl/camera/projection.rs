
#[rustfmt::skip]
pub const OPENGL_WGPU_MAT: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[derive(Debug)]
pub struct Projection {
    aspect: f32,
    z_near: f32,
    z_far: f32,
}

impl Projection {
    pub fn new(aspect: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            aspect,
            z_near,
            z_far,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self, v_fov: cgmath::Rad<f32>) -> cgmath::Matrix4<f32> {
        OPENGL_WGPU_MAT * cgmath::perspective(v_fov, self.aspect, self.z_near, self.z_far)
    }
}