#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_pbr::utils::{PI, random1D}
#import bevy_render::view::View

@group(0) @binding(0) var<uniform> view: View;

// -----------------------------------------------------------------------------
fn random2D_f(v: vec2<f32>) -> f32 {
    return fract(sin(dot(v.xy, vec2(12.9898, 78.233))) * 43758.5453123);
}

fn random2D_v(v: vec2<f32>) -> vec2<f32> {
    var v2 = vec2(dot(v, vec2(127.1, 311.7)), dot(v, vec2(269.5, 183.3)));
    return -1.0 + 2.0 * fract(sin(v2) * 43758.5453123);
}

fn rotate2D(theta: f32) -> mat2x2<f32> {
    let c = cos(theta);
    let s = sin(theta);
    return mat2x2<f32>(c, s, -s, c);
}

fn smooth_edge(v: f32, f: f32) -> f32 {
    return smoothstep(0.0, f / view.viewport.z, v);
}

fn linear_step(begin: f32, end: f32, t: f32) -> f32 {
    return clamp((t - begin) / (end - begin), 0.0, 1.0);
}

// Value noise by Inigo Quilez - iq/2013
// https://www.shadertoy.com/view/lsf3WH
fn value_noise(st: vec2f) -> f32 {
    let i = floor(st);
    let f = fract(st);
    let u = f * f * (3.0 - 2.0 * f);
    return mix( mix( random2D_f( i + vec2(0.0,0.0) ),
                     random2D_f( i + vec2(1.0,0.0) ), u.x),
                mix( random2D_f( i + vec2(0.0,1.0) ),
                     random2D_f( i + vec2(1.0,1.0) ), u.x), u.y);
}

// Gradient Noise by Inigo Quilez - iq/2013
// https://www.shadertoy.com/view/XdXGW8
fn gradient_noise(st: vec2f) -> f32 {
    let i = floor(st);
    let f = fract(st);
    let u = f * f * (3.0 - 2.0 * f);
    return mix( mix( dot( random2D_v(i + vec2(0.0,0.0) ), f - vec2(0.0,0.0) ),
                     dot( random2D_v(i + vec2(1.0,0.0) ), f - vec2(1.0,0.0) ), u.x),
                mix( dot( random2D_v(i + vec2(0.0,1.0) ), f - vec2(0.0,1.0) ),
                     dot( random2D_v(i + vec2(1.0,1.0) ), f - vec2(1.0,1.0) ), u.x), u.y);
}

// Voronoi by Inigo Quilez - iq/2013
// https://www.shadertoy.com/view/ldl3W8
fn voronoi(x: vec2f) -> vec3f {
    let two_pi = 2.0 * PI;
    let n = floor(x);
    let f = fract(x);
    //----------------------------------
    // first pass: regular voronoi
    //----------------------------------
    var mg = vec2(0.0);
    var mr = vec2(0.0);
    var md = 8.0;

    for(var j: i32 = -1; j <= 1; j++) {
        for(var i: i32 = -1; i <= 1; i++) {
            let g = vec2(f32(i), f32(j));
            var o = random2D_v(n + g);

            // animate
            o = 0.5 + 0.5 * sin(globals.time + two_pi * o);

            let r = g + o - f;
            let d = dot(r, r);

            if(d < md) {
                md = d;
                mr = r;
                mg = g;
            }
        }
    }
    //----------------------------------
    // second pass: distance to borders
    //----------------------------------
    md = 8.0;
    for(var j: i32 = -2; j <= 2; j++) {
        for(var i: i32 = -2; i <= 2; i++) {
            let g = mg + vec2(f32(i), f32(j));
            var o = random2D_v(n + g);

            // animate
            o = 0.5 + 0.5 * sin(globals.time + two_pi * o);

            let r = g + o - f;

            if(dot(mr - r, mr - r) > 0.00001) {
                md = min(md, dot(0.5 * (mr + r), normalize(r - mr)));
            }
        }
    }
    return vec3(md, mr);
}

// Smooth Voronoi by Inigo Quilez - iq/2014
// https://www.shadertoy.com/view/ldB3zc
fn voronoi_smooth(x: vec2f, w: f32) -> vec4f {
    let two_pi = 2.0 * PI;
    let n = floor(x);
    let f = fract(x);

	var m = vec4(8.0, 0.0, 0.0, 0.0);

    for(var j: i32 = -2; j <= 2; j++) {
        for(var i: i32 = -2; i <= 2; i++) {
            let g = vec2(f32(i), f32(j));
            var o = random2D_v(n + g);

            // animate
            o = 0.5 + 0.5 * sin(globals.time + two_pi * o);

            // distance to cell
            let d = length(g - f + o);

            // cell color
            var col = 0.5 + 0.5 * sin(random1D(dot(n + g, vec2(7.0, 113.0))) * 2.5 + 3.5 + vec3(2.0, 3.0, 0.0));
            // in linear space
            col = col * col;

            // do the smooth min for colors and distances
            let h = smoothstep(-1.0, 1.0, (m.x - d) / w);
            m.x   = mix(m.x, d, h) - h * (1.0 - h) * w / (1.0 + 3.0 * w); // distance
            let m_col = mix(m.yzw, col, h) - h * (1.0 - h) * w / (1.0 + 3.0 * w); // color
            m.y = m_col.x;
            m.z = m_col.y;
            m.w = m_col.z;
        }
    }

	return m;
}

// https://iquilezles.org/articles/smin
fn smin(a: f32, b: f32, k: f32) -> vec2f {
    let f1 = exp2(-k * a);
    let f2 = exp2(-k * b);
    return vec2(-log2(f1 + f2) / k, f2);
}

fn fbm (st: vec2f) -> f32 {
    var uv = st;
    let OCTAVES = 6;

    // Initial values
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 0.0;

    // Loop of octaves
    for (var i: i32 = 0; i < OCTAVES; i++) {
        value += amplitude * value_noise(uv);
        uv *= 2.0;
        amplitude *= 0.5;
    }
    return value;
}