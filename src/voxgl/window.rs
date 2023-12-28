use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, dpi::PhysicalSize,
};

use super::{state::State, camera::player_camera::MAX_VERTICAL_FOV};

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_max_inner_size(PhysicalSize::<i32>::new(800, 800))
        .with_title("Voxgl")
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(window).await;
    let mut last_render_time = std::time::Instant::now(); 

    event_loop.run(move |event, _, control_flow| 
        match event {
            
            Event::DeviceEvent { event: DeviceEvent::MouseMotion{ delta }, .. } => {
                if state.cursor_grabbed {
                    state.camera_controller.process_mouse(delta.0, delta.1);
                }
            }

            Event::DeviceEvent { event: DeviceEvent::MouseWheel { delta }, .. } => {
                if state.cursor_grabbed {
                    match delta {
                        MouseScrollDelta::LineDelta(_, y) => {
                            state.camera.v_fov -= cgmath::Deg(y);
                            state.camera.v_fov.0 = state.camera.v_fov.0.clamp(1.0, MAX_VERTICAL_FOV);
                        },
                        _ => {},
                    } 
                }
            } 

            Event::WindowEvent {
                ref event, window_id,
            } 
    
            if window_id == state.window.id() && !state.input(event) => if !state.input(event) {
                match event {
                    
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::X),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,

                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
        
                    _ => {}                    
                }
            },

            Event::RedrawRequested(window_id) if window_id == state.window().id() => {

                let then = std::time::Instant::now();
                let dt = then - last_render_time;
                last_render_time = then;
                state.update(dt);

                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }

                let now = std::time::Instant::now();
                let curr_fps = 1.0 / (now - then).as_secs_f32();
                
                state.window.set_title(
                    &format!("Voxgl [fps: {:.1} | tri_count: {}]", curr_fps, state.chunks.get_vertex_count() / 3)
                );
            }

            Event::MainEventsCleared => {
                state.window().request_redraw();
            }
            _ => {}
        }
    );
}
