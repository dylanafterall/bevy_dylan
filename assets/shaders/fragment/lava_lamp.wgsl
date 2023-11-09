// https://thebookofshaders.com/edit.php#11/splatter.frag

#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import "shaders/shader_utils.wgsl"::gradient_noise

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);
    let t = (1.0 - sin(globals.time * 0.1)) * 5.0;
    var uv = in.uv.xy;

    var color = vec3(0.0);

    uv += gradient_noise(uv * 2.0) * t; // Animate the coordinate space
    // color = vec3(1.0) * smoothstep(0.18, 0.2, gradient_noise(uv)); // Big black drops
    color += smoothstep(0.15, 0.2, gradient_noise(uv * 10.0)); // Black splatter
    color -= smoothstep(0.35, 0.4, gradient_noise(uv * 10.0)); // Holes on splatter

    return texture * vec4(1.0 - color, 1.0);
}