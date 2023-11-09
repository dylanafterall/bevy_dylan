// Created by inigo quilez - iq/2020
// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License.
// Edited from GLSL -> WGSL

#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);
    let t = globals.time * 3.0;

    var p = 12.0 * in.uv.xy;

    // 0: triangles
    // 1: squares
    let shape = 0;

    if(shape == 0) {
        p.x += 0.5 * p.y;
    }

    let f = fract(p);
    let i = floor(p);

    var id = fract(fract(dot(i, vec2(0.436, 0.173))) * 45.0);
    if(shape == 0) {
        if(f.x > f.y) {
            id += 1.3;
        }
    }

    let col = 0.5 + 0.5 * cos(0.7 * id + vec3(0.0, 1.5, 2.0) + 4.0);
    let pha = smoothstep(-1.0, 1.0, sin(0.2 * i.x + 0.2 * t + id * 1.0));

    var pat = vec2(0.0, 0.0);

    if(shape == 0) {
        pat = min(vec2(0.5 - abs(f - 0.5)), vec2(abs(f.x - f.y))) - 0.3 * pha;
    } else {
        pat = 0.5 - abs(f - 0.5) - 0.5 * pha;
    }

    pat = vec2(smoothstep(0.04, 0.07, pat.x), smoothstep(0.04, 0.07, pat.y));

    return texture * vec4(col * pat.x * pat.y, 1.0);
}