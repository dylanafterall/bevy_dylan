#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import "shaders/distance_field_shapes.wgsl"::{smoothedge, circle, rect, roundRect, ring, hexagon, tri, ellipse, capsule}

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    var uv = in.uv.xy;
    let t = globals.time * 0.3;

    let t0 = (t * 2.0) - 8.0 * floor((t * 2.0) / 8.0);
    let t1 = (t * 2.0 + 1.0) - 8.0 * floor((t * 2.0 + 1.0) / 8.0);
    let i0 = i32(floor(t0));
    let i1 = i32(floor(t1));
    let f = fract(t0);

    uv -= vec2(0.5, 0.5);
    let color = vec3(smoothedge(mix(get_shape(uv, i0), get_shape(uv, i1), f)));

    let texture = textureSample(color_texture, color_sampler, in.uv);

    return texture * vec4(color, 1.0);
}

fn get_shape(st: vec2f, i: i32) -> f32 {
    switch(i) {
        case 0: {
            return circle(st, 0.4);
        }
        case 1: {
            return roundRect(st, vec2(0.32, 0.24), 0.08);
        }
        case 2: {
            return tri(st, 0.4);
        }
        case 3: {
            return capsule(st, vec2(-0.25, -0.25), vec2(0.25, 0.25), 0.2);
        }
        case 4: {
            return ellipse(st, vec2(0.9, 1.2), 0.4);
        }
        case 5: {
            return rect(st, vec2(0.4, 0.32));
        }
        case 6: {
            return ring(st, 0.4, 0.05);
        }
        default: {
            return hexagon(st, 0.4);
        }
    }
}