// Created by inigo quilez - iq/2016
// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License
// Edited from GLSL -> WGSL

#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);
    let t = globals.time;

    var uv = in.uv.xy + 0.01 * t;
    uv *= 0.75;

    var gra = vec2(0.0);
    var col = vec3(0.0);
    let ouv = uv;
    for(var i: i32 =0; i < 64; i++) {
        uv += (0.1 / 64.0) * cos(6.2831 * cos(6.2831 * uv.yx + 0.02 * t * vec2(1.7, 2.1)) + 0.1 * t * vec2(2.1, 1.3));
        let tex = texture.xyz;
        col += tex * (1.0 / 64.0);
        // gra += vec2(tex.x - texture(iChannel0, uv + vec2(1.0 / iChannelResolution[0].x, 0.0)).x,
        //            tex.x - texture(iChannel0, uv + vec2(0.0, 1.0 / iChannelResolution[0].y)).x);
    }

    col *= 12.0 * length(uv - ouv);
    //col += 0.08 * (gra.x + gra.y);

    return vec4(col, 1.0);
}