use std::default::Default;
use rand::Rng;
use wgpu::{CompositeAlphaMode, PresentMode};
use wgpu_text::{glyph_brush::ab_glyph::FontRef, BrushBuilder};
use winit::{
    window::{Window, CursorGrabMode, CursorIcon}, 
    event::{WindowEvent, KeyboardInput, MouseButton, ElementState, VirtualKeyCode}
};

use crate::voxgl::{ 
    camera::{
        player_camera::{PlayerCamera, CameraUniform},
        camera_controller::CameraController,
    },
    texture::Texture, 
    world::chunks::Chunks,
    rendering::arena::MeshArena,
};

use super::rendering::pipeline;

pub struct State<'a> {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Window,
    pub render_pipeline: wgpu::RenderPipeline,
    pub depth_texture: Texture,
    pub arena: MeshArena,

    pub camera_uniform: CameraUniform,
    pub camera: PlayerCamera,
    pub camera_controller: CameraController,

    pub chunks: Chunks,
    pub sky_color: wgpu::Color,
    pub brush: wgpu_text::TextBrush<FontRef<'a>>,

    pub mouse_pressed: bool,
    pub cursor_grabbed: bool,
}

impl<'a> State<'a> {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::GL,
                ..Default::default()
            }
        );
        
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None
            },
            None
        ).await.unwrap();

        let swapchain_format = wgpu::TextureFormat::Rgba16Float;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Mailbox,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &config);
        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

        let brush = BrushBuilder::using_font_bytes(include_bytes!("resources/consolas.TTF")).unwrap()
                .build(&device, config.width, config.height, config.format);

        let mut camera_uniform = CameraUniform::new();
        let camera = PlayerCamera::new(
            cgmath::Point3::new(0.0, 20.0, 0.0),
            cgmath::Deg(-90.0).into(),
            cgmath::Deg(-20.0).into(),
            config.width as f32 / config.height as f32,
            0.1,
            100.0,
            &camera_uniform,
            &device
        );
        
        camera_uniform.update_view_proj(&camera);
        let camera_controller = CameraController::new(18.0, 1.0);

        let render_pipeline = pipeline::create_voxel_pipeline(
            &device,
            &[&camera.layout],
        );

        let mut arena = MeshArena::new();
        let mut chunks = Chunks::new();

        chunks.update_load_data_queue();
        chunks.update_load_mesh_queue();

        chunks.build_chunk_data_in_queue();
        chunks.build_chunk_meshes_in_queue(&device, &mut arena);

        let sky_color = wgpu::Color {
            r: 0.2, g: 0.4, b: 0.8, a: 1.0
        };

        Self {
            window, 
            surface, 
            device, 
            queue, 
            config, 
            size,
            render_pipeline,
            depth_texture,
            camera_uniform,
            camera,
            camera_controller,
		
            sky_color,
            chunks,
            brush,
            arena,

            mouse_pressed: false,
            cursor_grabbed: false,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.depth_texture = Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
            self.brush.resize_view(self.config.width as f32, self.config.height as f32, &self.queue);

            self.camera.projection.resize(new_size.width, new_size.height);
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => {
                if self.window.has_focus() {
                    self.camera_controller.process_keyboard(*key, *state);

                    if *key == VirtualKeyCode::Escape && *state == ElementState::Pressed {
                        self.grab_cursor();
                    }
                }
                true
            },
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            },
            _ => false,
        }
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        if self.cursor_grabbed {
            self.camera_controller.update(&mut self.camera, dt);
            self.camera_uniform.update_view_proj(&self.camera);
        }

        self.queue.write_buffer(
            &self.camera.buffer,
            0, 
            bytemuck::cast_slice(&[self.camera_uniform])
        );

        self.chunks.position = (self.camera.position.x, self.camera.position.y, self.camera.position.z).into();

        self.chunks.update_load_data_queue();
        self.chunks.update_load_mesh_queue();
        self.chunks.update_unload_mesh_queue();
        self.chunks.update_unload_data_queue();

        self.chunks.build_chunk_data_in_queue();
        self.chunks.build_chunk_meshes_in_queue(&self.device, &mut self.arena);
        
        self.chunks.unload_data_queue();
        self.chunks.unload_mesh_queue(&mut self.arena);
    }

    fn grab_cursor(&mut self) {
        self.cursor_grabbed = !self.cursor_grabbed;
        if self.cursor_grabbed {
            self.window.set_cursor_grab(CursorGrabMode::Confined).unwrap();
            self.window.set_cursor_visible(false);
        } else {
            self.window.set_cursor_grab(CursorGrabMode::None).unwrap();            
            self.window.set_cursor_visible(true);
        }
    }
}
