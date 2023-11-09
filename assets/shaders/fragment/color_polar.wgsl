#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::utils::{hsv2rgb, PI}

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);
    let uv = in.uv.xy;

    var color = vec3(0.0);

    // Use polar coordinates instead of cartesian
    let toCenter = vec2(0.5) - uv;
    let angle = atan2(toCenter.y, toCenter.x);
    let radius = length(toCenter) * 1.75;

    // Map the angle (-PI to PI) to the Hue (from 0 to 1)
    // and the Saturation to the radius
    color = hsv2rgb(angle / (2.0 * PI) + 0.5, radius, 1.0);

    return texture * vec4(color, 1.0);
}