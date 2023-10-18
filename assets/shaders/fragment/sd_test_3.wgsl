#import bevy_pbr::mesh_vertex_output            MeshVertexOutput
#import "shaders/sd_shapes.wgsl"                sdArc, sdRing, sdHorseshoe, sdVesica, sdMoon, sdArrow, sdEgg, sdRoundedX, sdCross
#import bevy_pbr::utils                         PI, HALF_PI
#import bevy_render::view                       View

@group(0) @binding(0) var<uniform> view: View;

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: MeshVertexOutput,) -> @location(0) vec4<f32> {
    var uv = in.uv.xy;

    var d = sdArc(uv - vec2(0.2), vec2(sin(0.5 * HALF_PI), cos(0.5 * HALF_PI)), 0.08, 0.02);
    d = min(d, sdRing(uv - vec2(0.5, 0.2), vec2(cos(PI), sin(HALF_PI)), 0.08, 0.04));
    d = min(d, sdHorseshoe(uv - vec2(0.8, 0.2), vec2(sin(0.6 * HALF_PI), cos(0.6 * HALF_PI)), 0.07, 0.05, 0.015));
    d = min(d, sdVesica(uv - vec2(0.2, 0.5), 0.08, 0.04));
    d = min(d, sdMoon(uv - vec2(0.5), 0.04, 0.1, 0.07));
    d = min(d, sdArrow(uv - vec2(0.8, 0.5), vec2(-0.1, 0.0), vec2(0.1, 0.0), 0.02, 0.04));
    d = min(d, sdEgg(uv - vec2(0.2, 0.8), 0.08, 0.04));
    d = min(d, sdRoundedX(uv - vec2(0.5, 0.8), 0.1, 0.03));
    d = min(d, sdCross(uv - vec2(0.8), vec2(0.1, 0.03)));
    let color = vec3(smoothedge(d));

    let texture = textureSample(color_texture, color_sampler, in.uv);
    return texture * vec4(color, 1.0);
}

fn smoothedge(v: f32) -> f32 {
    return smoothstep(0.0, 1.0 / view.viewport.z, v);
}