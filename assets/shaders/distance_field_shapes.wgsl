// Author @kynd - 2016
// Title: Distance Field Shapes
// http://www.kynd.info

#import bevy_render::view   View
@group(0) @binding(0) var<uniform> view: View;

fn smoothedge(v: f32) -> f32 {
    return smoothstep(0.0, 1.0 / view.viewport.z, v);
}

fn smoothen(d1: f32, d2: f32) -> f32 {
    let k = 1.5;
    return -log(exp(-k * d1) + exp(-k * d2)) / k;
}

fn circle(p: vec2<f32>, radius: f32) -> f32 {
    return length(p) - radius;
}

fn rect(p: vec2<f32>, size: vec2<f32>) -> f32 {
    let d = abs(p) - size;
    return min(max(d.x, d.y), 0.0) + length(vec2(max(d.x, 0.0), max(d.y, 0.0)));
}

fn roundRect(p: vec2<f32>, size: vec2<f32>, radius: f32) -> f32 {
    let d = abs(p) - size;
    return min(max(d.x, d.y), 0.0) + length(vec2(max(d.x, 0.0), max(d.y, 0.0))) - radius;
}

fn ring(p: vec2<f32>, radius: f32, width: f32) -> f32 {
    return abs(length(p) - radius * 0.5) - width;
}

fn hexagon(p: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p);
    return max(abs(q.y), q.x * 0.866025 + q.y * 0.5) - radius;
}

fn tri(p: vec2<f32>, size: f32) -> f32 {
    let q = abs(p);
    return max(q.x * 0.866025 + p.y * 0.5, -p.y * 0.5) - size * 0.5;
}

fn ellipse(p: vec2<f32>, r: vec2<f32>, s: f32) -> f32 {
    return (length(p / r) - s);
}

fn capsule(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa,ba)/dot(ba,ba), 0.0, 1.0);
    return length(pa - ba * h) - r;
}

/* fn polygon(p: vec2<f32>, vertices: i32, size: f32) -> f32 {
    let a = atan(p.x / p.y) + 0.2;
    let b = 6.28319 / f32(vertices);
    return cos(floor(0.5 + a / b) * b - a) * length(p) - size;
} */
