// The MIT License
// Copyright © 2013 Inigo Quilez
// https://www.youtube.com/c/InigoQuilez
// https://iquilezles.org/
// This code is edited to support WGSL and Bevy Engine use

#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import "shaders/shader_utils.wgsl"::value_noise

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);
    let t = globals.time * 0.25;
    var uv = in.uv.xy;

    uv += t;    // animate/move the shader

    var f = 0.0;
    // left: value noise
    if(uv.x < 0.6) {
        f = value_noise(32.0 * uv);
    // right: fbm - fractal noise (4 octaves)
    } else {
        uv *= 8.0;
        let m = mat2x2<f32>(1.6, 1.2, -1.2, 1.6);
        f  = 0.5000 * value_noise(uv);
        uv = m * uv;
        f += 0.2500 * value_noise(uv);
        uv = m * uv;
        f += 0.1250 * value_noise(uv);
        uv = m * uv;
        f += 0.0625 * value_noise(uv);
        uv = m * uv;
    }
    f = 0.5 + 0.5 * f;

    f *= smoothstep(0.0, 0.005, abs(uv.x - 0.6));

    return texture * vec4(f, f, f, 1.0);
}