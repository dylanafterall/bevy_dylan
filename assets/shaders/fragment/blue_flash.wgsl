#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    let blue_wave = (0.5 * sin(globals.time)) + 0.5;

    let texture = textureSample(color_texture, color_sampler, in.uv);

    return texture * vec4(in.uv.x, in.uv.y, blue_wave, 1.0);
}