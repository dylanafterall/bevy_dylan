// Author @patriciogv - 2015
// http://patriciogonzalezvivo.com
// https://thebookofshaders.com/edit.php#11/circleWave-noise.frag
// Edited for WGSL and Bevy Engine

#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_pbr::utils::PI
#import "shaders/shader_utils.wgsl"::{random2D_v, gradient_noise, rotate2D}

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);

    let color = vec3(1.0) * shapeBorder(in.uv.xy, 0.8, 0.02);
    return texture * vec4(1.0 - color, 1.0);
}

fn shapeBorder(st: vec2f, radius: f32, width: f32) -> f32 {
    return shape(st, radius) - shape(st, radius - width);
}

fn shape(st: vec2f, radius: f32) -> f32 {
    let t = globals.time * 0.5;
	let uv = vec2(0.5) - st;
    let r = length(uv) * 2.0;
    var a = atan2(uv.y, uv.x);

    let m_x = a + t * 2.0;
    let m_y = PI * 2.0;
    let m_mod = m_x - m_y * floor(m_x / m_y);
    var m = abs(m_mod - PI) / 3.6;

    var f = radius;
    m += gradient_noise(uv + t * 0.1) * 0.5;
    // a *= 1.0 + abs(atan2(t * 0.2)) * 0.1;
    // a *= 1.0 + gradient_noise(uv + t * 0.1) * 0.1;
    f += sin(a * 50.0) * gradient_noise(uv + t * 0.2) * 0.1;
    f += (sin(a * 20.0) * 0.1 * pow(m, 2.0));
    return 1.0 - smoothstep(f, f + 0.007, r);
}