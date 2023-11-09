// The MIT License
// https://www.youtube.com/c/InigoQuilez
// https://iquilezles.org/
// Copyright © 2014 Inigo Quilez
// This code is edited to support WGSL and Bevy Engine use

#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import "shaders/shader_utils.wgsl"::voronoi

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);
    let uv = in.uv.xy;

    let c = voronoi(8.0 * uv);

    // isolines
    var col = c.x * (0.5 + 0.5 * sin(64.0 * c.x)) * vec3(1.0);
    // borders
    col = mix(vec3(1.0, 0.6, 0.0), col, smoothstep(0.04, 0.07, c.x));
    // feature points
    let dd = length(c.yz);
    col = mix(vec3(1.0, 0.6, 0.1), col, smoothstep(0.0, 0.12, dd));
    col += vec3(1.0, 0.6, 0.1) * (1.0 - smoothstep(0.0, 0.04, dd));

    return texture * vec4(col, 1.0);
}
