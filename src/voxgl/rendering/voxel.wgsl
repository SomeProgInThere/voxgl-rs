// Vertex shader

struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) shading: f32,
}

@vertex
fn vs_main(vert_in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    var face_shade = array<f32, 6>(
        1.0, 0.4, // top, bottom 
        0.4, 0.7, // right, left
        0.4, 0.7, // front, back
    );

    out.color = vert_in.color;
    out.clip_position = camera.view_proj * vec4<f32>(vert_in.position, 1.0);
    var face_id: i32;

    if vert_in.normal.x > 0.0 { face_id = 2; } 
    else if vert_in.normal.x < 0.0 { face_id = 3; }
    else if vert_in.normal.y > 0.0 { face_id = 0; }
    else if vert_in.normal.y < 0.0 { face_id = 1; }
    else if vert_in.normal.z > 0.0 { face_id = 4; }
    else if vert_in.normal.z < 0.0 { face_id = 5; }

    out.shading = face_shade[face_id];
    return out;
}

// Fragment shader

@fragment
fn fs_main(vert_in: VertexOutput) -> @location(0) vec4<f32> {
    var in = vert_in;
    in.color *= in.shading;
    return in.color;
}
