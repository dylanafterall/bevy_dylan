fn random2D(v: vec2<f32>) -> f32 {
    return fract(sin(dot(v.xy, vec2(12.9898, 78.233))) * 43758.5453123);
}

fn rotate2D(theta: f32) -> mat2x2<f32> {
    let c = cos(theta);
    let s = sin(theta);
    return mat2x2<f32>(c, s, -s, c);
}

// https://iquilezles.org/articles/smin
fn smin(a: f32, b: f32, k: f32) -> vec2f {
    let f1 = exp2(-k * a);
    let f2 = exp2(-k * b);
    return vec2(-log2(f1 + f2) / k, f2);
}