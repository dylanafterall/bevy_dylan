#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::utils::hsv2rgb

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);

    // We map x (0.0 - 1.0) to the hue (0.0 - 1.0)
    // And the y (0.0 - 1.0) to the brightness
    let color = hsv2rgb(in.uv.x, 1.0, in.uv.y);

    return texture * vec4(color, 1.0);
}