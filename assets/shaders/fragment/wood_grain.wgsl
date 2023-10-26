// https://thebookofshaders.com/edit.php#11/wood.frag

#import bevy_pbr::mesh_vertex_output        MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings   globals
#import bevy_pbr::utils                     PI
#import "shaders/shader_utils.wgsl"         value_noise, rotate2D

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: MeshVertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);
    let t = globals.time * 0.1;

    var pos = in.uv.yx * vec2(10.0, 3.0);
    var pattern = pos.x;

    // Add noise
    pos = rotate2D(value_noise(pos)) * pos + t;

    // Draw lines
    pattern = lines(pos, 0.5);

    return texture * vec4(vec3(pattern), 1.0);
}

fn lines(pos: vec2f, b: f32) -> f32 {
    let scale = 10.0;
    let p = pos * scale;
    return smoothstep(0.0, 0.5 + b * 0.5, abs((sin(p.x * PI) + b * 2.0)) * 0.5);
}