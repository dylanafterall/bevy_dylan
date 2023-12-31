#import bevy_pbr::forward_io::VertexOutput
#import "shaders/shader_utils.wgsl"::random2D_f

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    var uv = in.uv.xy;
    let random = random2D_f(uv);
    let color = vec3(random);

    let texture = textureSample(color_texture, color_sampler, in.uv);

    return texture * vec4(color, 1.0);
}