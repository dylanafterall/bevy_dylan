// Author @kyndinfo - 2016
// http://www.kynd.info
// Title: Wipes
// https://thebookofshaders.com/edit.php?log=160909065049
// Edited for WGSL and Bevy Engine

#import bevy_pbr::mesh_vertex_output            MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings       globals
#import bevy_pbr::utils                         PI
#import "shaders/shader_utils.wgsl"             smooth_edge, linear_step
#import "shaders/easings.wgsl"                  cubicInOut
#import "shaders/sd_shapes.wgsl"                sdCircle

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: MeshVertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);
    let uv = in.uv.xy;
    let t = globals.time - 4.0 * floor(globals.time / 4.0);

    let BLUE = vec3(0.0, 0.0, 1.0);
    let GREEN = vec3(0.0, 1.0, 0.0);
    let RED = vec3(1.0, 0.0, 0.0);
    let WHITE = vec3(1.0, 1.0, 1.0);
    let BLACK = vec3(0.0, 0.0, 0.0);

    // wipe left to right
    var color = BLACK;
    let v0 = 1.0 - step(cubicInOut(linear_step(0.0, 0.7, t)), uv.x);
    let v1 = 1.0 - step(cubicInOut(linear_step(0.3, 1.0, t)), uv.x);
    color = mix(color, RED, v0 - v1);

    // wipe top to bottom
    let v2 = 1.0 - step(cubicInOut(linear_step(1.0, 1.7, t)), uv.y);
    let v3 = 1.0 - step(cubicInOut(linear_step(1.3, 2.0, t)), uv.y);
    color = mix(color, GREEN, v2 - v3);

    // wipe circle inner -> out
    let v4 = circlePlot(uv - vec2(0.5), cubicInOut(linear_step(2.0, 2.7, t)));
    let v5 = circlePlot(uv - vec2(0.5), cubicInOut(linear_step(2.3, 3.0, t)));
    color = mix(color, BLUE, v4 - v5);

    // wipe clock rotation
    let v6 = clockWipe(uv - vec2(0.5), cubicInOut(linear_step(3.0, 3.6, t)));
    let v7 = clockWipe(uv - vec2(0.5), cubicInOut(linear_step(3.4, 4.0, t)));
    color = mix(color, WHITE, v6 - v7);

    return texture * vec4(color, 1.0);
}

fn circlePlot(p: vec2f, radius: f32) -> f32 {
  return 1.0 - smooth_edge(sdCircle(p, radius), 1.0);
}

fn clockWipe(p: vec2f, t: f32) -> f32 {
    let a = atan2(-p.x, -p.y);
    let v = select(
                0.0,
                1.0,
                t * (2.0 * PI) > a + PI
            );
    return v;
}