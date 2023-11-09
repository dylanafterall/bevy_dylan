// The MIT License
// Copyright © 2021 Inigo Quilez
// Edited from GLSL -> WGSL

#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import "shaders/shader_utils.wgsl"::smin
#import "shaders/sd_shapes.wgsl"::sdBox

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);
    let t = globals.time;

    var p = 2.0 * in.uv.xy - 1.0;

    // background color
    let mod1 = floor(p.x * 4.0) + floor(p.y * 4.0) - 2.0 * floor((floor(p.x * 4.0) + floor(p.y * 4.0)) / 2.0);
    var col = vec3(0.1 + 0.05 * mod1); // background color

    // compute SDF and its color
    var dmin = 1e20;
    var dcol = vec4(0.0, 0.0, 0.0, 0.0);

    // 10 boxes
    for(var i: i32 = 0; i < 10; i++) {
        let h = f32(i);

        // new box position and color
        let ce = vec2(1.5, 1.0) * cos(0.1 * t * vec2(1.3 + 0.1 * h, 1.7 + (7.0 - h) * h * 0.3) + h * vec2(11.1, 28.7) + vec2(0.0, 2.0));
        var co = 0.5 + 0.5 * cos(h * 1.0 + vec3(0.0, 2.0, 4.0));
        co = co * co;

        // distance to box
        let dis = sdBox(p - ce, vec2(0.4, 0.2));

        // smoothly blend SDFs
        let db = smin(dmin, dis, 10.0);
        dmin = db.x;

        // smoothly blend colors
        let w = db.y;
        dcol += vec4(co * w, w);
    }
    // resolve color
    dcol /= dcol.w;

    // draw SDF on top
    col = mix(col, dcol.xyz, 1.0 - smoothstep(0.0, 0.01, dmin));

    // gamma
    col = sqrt(col);

    // output to screen
    return texture * vec4(col, 1.0);
}