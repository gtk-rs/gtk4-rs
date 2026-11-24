struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

// Rotation angle in radians, updated every frame.
@group(0) @binding(0)
var<uniform> angle: f32;

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.7),
        vec2<f32>(-0.6, -0.5),
        vec2<f32>(0.6, -0.5),
    );
    var colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
    );
    let p = positions[index];
    let c = cos(angle);
    let s = sin(angle);

    var out: VertexOutput;
    // wgpu draws upside down on GL and flips the image when presenting to its own
    // surfaces. Here wgpu draws straight into the framebuffer of the GLArea, so flip y.
    out.position = vec4<f32>(p.x * c - p.y * s, -(p.x * s + p.y * c), 0.0, 1.0);
    out.color = colors[index];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
