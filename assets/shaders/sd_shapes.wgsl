// https://gist.github.com/munrocket/30e645d584b5300ee69295e54674b3e4
// MIT License. © 2023 Inigo Quilez, Munrocket

/*
Contents:
     1) Circle
     2) Rounded Box
     3) Box
     4) Oriented Box
     5) Segment
     6) Rhombus
     7) Isosceles Trapezoid
     8) Parallelogram
     9) Equilateral Triangle
    10) Isosceles Triangle
    11) Triangle
    12) Uneven Capsule
    13) Pentagon
    14) Hexagon
    15) Octagon
    16) Hexagram
    17) Star 5
    18) Regular Star
    19) Pie
    20) Arc
    20b) Ring
    21) Horseshoe
    22) Vesica
    23) Moon
    24) Rounded Cross
    25) Egg
    26) Heart
    27) Cross
    28) Rounded X
    29) Polygon
    30) Ellipse
    31) Parabola
    32) Parabola Segment
    33) Quadratic Bezier
    34) Bobbly Cross
    35) Cut Disk
    36) Tunnel
    37) Quad Disk
    38) Arrow
    39) Stairs
*/

//  1) Circle ------------------------------------------------------------------
fn sdCircle(p: vec2f, r: f32) -> f32 {
    return length(p) - r;
}

//  2) Rounded Box -------------------------------------------------------------
// b.x = width
// b.y = height
// r.x = roundness top-right
// r.y = roundness boottom-right
// r.z = roundness top-left
// r.w = roundness bottom-left
fn sdRoundedBox(p: vec2f, b: vec2f, r: vec4f) -> f32 {
    var x = r.x;
    var y = r.y;
    x = select(r.z, r.x, p.x > 0.);
    y = select(r.w, r.y, p.x > 0.);
    x  = select(y, x, p.y > 0.);
    let q = abs(p) - b + x;
    return min(max(q.x, q.y), 0.) + length(max(q, vec2f(0.))) - x;
}

//  3) Box ---------------------------------------------------------------------
fn sdBox(p: vec2f, b: vec2f) -> f32 {
    let d = abs(p) - b;
    return length(max(d, vec2f(0.))) + min(max(d.x, d.y), 0.);
}

//  4) Oriented Box ------------------------------------------------------------
// same as sdSegment, but with a thickness 'th'
// a = start of line segment
// b = end of line segment
// th = thickness
fn sdOrientedBox(p: vec2f, a: vec2f, b: vec2f, th: f32) -> f32 {
    let l = length(b - a);
    let d = (b - a) / l;
    var q = p - (a + b) * 0.5;
    q = q * mat2x2<f32>(vec2f(d.x, d.y), vec2f(-d.y, d.x));
    q = abs(q) - vec2f(l, th) * 0.5;
    return length(max(q, vec2f(0.))) + min(max(q.x, q.y), 0.);
}

//  5) Segment -----------------------------------------------------------------
fn sdSegment(p: vec2f, a: vec2f, b: vec2f) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0., 1.);
    return length(pa - ba * h);
}

//  6) Rhombus -----------------------------------------------------------------
fn sdRhombus(p: vec2f, b: vec2f) -> f32 {
    let q = abs(p);
    let qb = dot(q, vec2f(b.x, -b.y));
    let bb = dot(b, vec2f(b.x, -b.y));
    let h = clamp((-2. * qb + bb) / dot(b, b), -1., 1.);
    let d = length(q - 0.5 * b * vec2f(1. - h, 1. + h));
    return d * sign(q.x * b.y + q.y * b.x - b.x * b.y);
}

//  7) Trapezoid ---------------------------------------------------------------
// r1 = first base width
// r2 = second base width
// he = height / length between bases
fn sdTrapezoid(p: vec2f, r1: f32, r2: f32, he: f32) -> f32 {
    let k1 = vec2f(r2, he);
    let k2 = vec2f(r2 - r1, 2. * he);
    let q = vec2f(abs(p.x), p.y);
    let ca = vec2f(q.x - min(q.x, select(r2, r1, q.y < 0.0)), abs(q.y) - he);
    let cb = q - k1 + k2 * clamp(dot(k1 - q, k2) / dot(k2, k2), 0., 1.);
    let s = select(1., -1., cb.x < 0.0 && ca.y < 0.0);
    return s * sqrt(min(dot(ca, ca), dot(cb, cb)));
}

//  8) Parallelogram -----------------------------------------------------------
// wi = width
// he = height
// sk = skew
fn sdParallelogram(p: vec2f, wi: f32, he: f32, sk: f32) -> f32 {
    let e = vec2f(sk, he);
    var q: vec2f = select(p, -p, p.y < 0.);
    // horizontal edge
    var w: vec2f = q - e;
    w.x = w.x - clamp(w.x, -wi, wi);
    var d: vec2f = vec2f(dot(w, w), -w.y);
    // vertical edge
    let s = q.x * e.y - q.y * e.x;
    q = select(q, -q, s < 0.);
    var v: vec2f = q - vec2f(wi, 0.);
    v = v - e * clamp(dot(v, e) / dot(e, e), -1., 1.);
    d = min(d, vec2f(dot(v, v), wi * he - abs(s)));
    return sqrt(d.x) * sign(-d.y);
}

//  9) Equilateral Triangle ----------------------------------------------------
fn sdEquilateralTriangle(p: vec2f, r: f32) -> f32 {
    let k = sqrt(3.0);
    var q = vec2f(abs(p.x) - r, p.y + r / k);
    if(q.x + k * q.y > 0.0) { q = vec2(q.x - k * q.y, -k * q.x - q.y) / 2.0; }
    q.x -= clamp(q.x, -2.0 * r, 0.0);
    return -length(q) * sign(q.y);
}

// 10) Isosceles Triangle ------------------------------------------------------
// c.x = width
// c.y = height
fn sdIsoscelesTriangle(p: vec2f, c: vec2f) -> f32 {
    let q = vec2f(abs(p.x), p.y);
    let a = q - c * clamp(dot(q, c) / dot(c, c), 0., 1.);
    let b = q - c * vec2f(clamp(q.x / c.x, 0., 1.), 1.);
    let s = -sign(c.y);
    let d = min(vec2f(dot(a, a), s * (q.x * c.y - q.y * c.x)), vec2f(dot(b, b), s * (q.y - c.y)));
    return -sqrt(d.x) * sign(d.y);
}

// 11) Triangle ----------------------------------------------------------------
fn sdTriangle(p: vec2f, p0: vec2f, p1: vec2f, p2: vec2f) -> f32 {
    let e0 = p1 - p0; let e1 = p2 - p1; let e2 = p0 - p2;
    let v0 = p - p0; let v1 = p - p1; let v2 = p - p2;
    let pq0 = v0 - e0 * clamp(dot(v0, e0) / dot(e0, e0), 0., 1.);
    let pq1 = v1 - e1 * clamp(dot(v1, e1) / dot(e1, e1), 0., 1.);
    let pq2 = v2 - e2 * clamp(dot(v2, e2) / dot(e2, e2), 0., 1.);
    let s = sign(e0.x * e2.y - e0.y * e2.x);
    let d = min(min(vec2f(dot(pq0, pq0), s * (v0.x * e0.y - v0.y * e0.x)),
                  vec2f(dot(pq1, pq1), s * (v1.x * e1.y - v1.y * e1.x))),
                  vec2f(dot(pq2, pq2), s * (v2.x * e2.y - v2.y * e2.x)));
    return -sqrt(d.x) * sign(d.y);
}

// 12) Uneven Capsule ----------------------------------------------------------
// r1: radius first circle
// r2: radius second circle
// h: height / length between circles
fn sdUnevenCapsule(p: vec2f, r1: f32, r2: f32, h: f32) -> f32 {
    let q = vec2f(abs(p.x), p.y);
    let b = (r1 - r2) / h;
    let a = sqrt(1. - b * b);
    let k = dot(q, vec2f(-b, a));
    if (k < 0.) { return length(q) - r1; }
    if (k > a * h) { return length(q - vec2f(0., h)) - r2; }
    return dot(q, vec2f(a, b)) - r1;
}

// 13) Pentagon ----------------------------------------------------------------
fn sdPentagon(p: vec2f, r: f32) -> f32 {
    let k = vec3f(0.809016994, 0.587785252, 0.726542528);
    var q: vec2f = vec2f(abs(p.x), p.y);
    q = q - 2. * min(dot(vec2f(-k.x, k.y), q), 0.) * vec2f(-k.x, k.y);
    q = q - 2. * min(dot(vec2f(k.x, k.y), q), 0.) * vec2f(k.x, k.y);
    q = q - vec2f(clamp(q.x, -r * k.z, r * k.z), r);
    return length(q) * sign(q.y);
}

// 14) Hexagon -----------------------------------------------------------------
fn sdHexagon(p: vec2f, r: f32) -> f32 {
    let k = vec3f(-0.866025404, 0.5, 0.577350269);
    var q: vec2f = abs(p);
    q = q - 2. * min(dot(k.xy, q), 0.) * k.xy;
    q = q - vec2f(clamp(q.x, -k.z * r, k.z * r), r);
    return length(q) * sign(q.y);
}

// 15) Octagon -----------------------------------------------------------------
fn sdOctagon(p: vec2f, r: f32) -> f32 {
    let k = vec3f(-0.9238795325, 0.3826834323, 0.4142135623);
    var q: vec2f = abs(p);
    q = q - 2. * min(dot(vec2f(k.x, k.y), q), 0.) * vec2f(k.x, k.y);
    q = q - 2. * min(dot(vec2f(-k.x, k.y), q), 0.) * vec2f(-k.x, k.y);
    q = q - vec2f(clamp(q.x, -k.z * r, k.z * r), r);
    return length(q) * sign(q.y);
}

// 16) Hexagram ----------------------------------------------------------------
fn sdHexagram(p: vec2f, r: f32) -> f32 {
    let k = vec4f(-0.5, 0.8660254038, 0.5773502692, 1.7320508076);
    var q: vec2f = abs(p);
    q = q - 2. * min(dot(k.xy, q), 0.) * k.xy;
    q = q - 2. * min(dot(k.yx, q), 0.) * k.yx;
    q = q - vec2f(clamp(q.x, r * k.z, r * k.w), r);
    return length(q) * sign(q.y);
}

/* // 17) Star 5 ------------------------------------------------------------------
fn sdStar5(p: vec2f, r: f32, rf: f32) -> f32 {
    let k1 = vec2f(0.809016994375, -0.587785252292);
    let k2 = vec2f(-k1.x, k1.y);
    var q: vec2f = vec2f(abs(p.x), p.y);
    q = q - 2. * max(dot(k1, q), 0.) * k1;
    q = q - 2. * max(dot(k2, q), 0.) * k2;
    q.x = abs(q.x);
    q.y = q.y - r;
    let ba = rf * vec2f(-k1.y, k1.x) - vec2f(0., 1.);
    let h = clamp(dot(q, ba) / dot(ba, ba), 0., r);
    return length(q - ba * h) * sign(q.y * ba.x - q.x * ba.y);
} */

// 18) Star --------------------------------------------------------------------
// n = number of star points
// m = angle divisor, between 2 and n (lower = fatter star, higher = thinner)
fn sdStar(p: vec2f, r: f32, n: u32, m: f32) -> f32 {
    let an = 3.141593 / f32(n);
    let en = 3.141593 / m;
    let acs = vec2f(cos(an), sin(an));
    let ecs = vec2f(cos(en), sin(en));
    let bn = (atan2(abs(p.x), p.y) % (2. * an)) - an;
    var q: vec2f = length(p) * vec2f(cos(bn), abs(sin(bn)));
    q = q - r * acs;
    q = q + ecs * clamp(-dot(q, ecs), 0., r * acs.y / ecs.y);
    return length(q) * sign(q.x);
}

// 19) Pie ---------------------------------------------------------------------
// sc = (sin, cos) of the angle
fn sdPie(p: vec2f, sc: vec2f, r: f32) -> f32 {
    let q = vec2f(abs(p.x), p.y);
    let l = length(q) - r;
    let m = length(q - sc * clamp(dot(q, sc), 0., r));
    return max(l, m * sign(sc.y * q.x - sc.x * q.y));
}

// 20) Arc ---------------------------------------------------------------------
// sc = sin/cos of the aperture
fn sdArc(p: vec2f, sc: vec2f, ra: f32, rb: f32) -> f32 {
    let q = vec2(abs(p.x), p.y);
    return  select(
                abs(length(p) - ra),
                length(q - sc * ra),
                sc.y * q.x > sc.x * q.y
            ) - rb;
}

// 20b) Ring -------------------------------------------------------------------
fn sdRing(p: vec2f, n: vec2f, r: f32, th: f32) -> f32 {
    var q = vec2(abs(p.x), p.y);
    q = mat2x2<f32>(n.x, n.y, -n.y, n.x) * q;

    return max(abs(length(q) - r) - th * 0.5,
                length(vec2(q.x, max(0.0, abs(r - q.y) - th * 0.5))) * sign(q.x));
}

// 21) Horseshoe ---------------------------------------------------------------
// l = length of straight ends of horseshoe
// w = width / thickness of horseshoe
fn sdHorseshoe(p: vec2f, sc: vec2f, r: f32, l: f32, w: f32) -> f32 {
    var q: vec2f = vec2f(abs(p.x), p.y);
    let m = length(p);
    q = q * mat2x2<f32>(vec2f(-sc.y, sc.x), vec2f(sc.x, sc.y));
    q = vec2f(select(m * sign(-sc.y), q.x, q.y > 0.0 || q.x > 0.), select(m, q.y, q.x > 0.));
    q = vec2f(q.x, abs(q.y - r)) - vec2f(l, w);
    return length(max(q, vec2f(0.))) + min(0., max(q.x, q.y));
}

// 22) Vesica ------------------------------------------------------------------
fn sdVesica(p: vec2f, r: f32, d: f32) -> f32 {
    let q = abs(p);
    let b = sqrt(r * r - d * d);
    let cond = (q.y -b) * d > q.x * b;
    return select(length(q - vec2f(-d, 0.))-r, length(q - vec2f(0., b)), cond);
}

// 23) Moon --------------------------------------------------------------------
fn sdMoon(p: vec2f, d: f32, ra: f32, rb: f32) -> f32 {
    let q = vec2f(p.x, abs(p.y));
    let a = (ra * ra - rb * rb + d * d) / (2. * d);
    let b = sqrt(max(ra * ra - a * a, 0.));
    if (d * (q.x * b - q.y * a) > d * d * max(b - q.y, 0.)) { return length(q-vec2f(a, b)); }
    return max((length(q) - ra), -(length(q - vec2f(d, 0.)) - rb));
}

/* // 24) Rounded Cross -----------------------------------------------------------
fn sdRoundedCross(p: vec2f, h: f32) -> f32 {
    let k = 0.5 * (h + 1. / h);
    let q = abs(p);
    let v1 = q - vec2f(1., k);
    let v2 = q - vec2f(0., h);
    let v3 = q - vec2f(1., 0.);
    let d1 = k - sqrt(dot(v1, v1));
    let d2 = sqrt(min(dot(v2, v2), dot(v3, v3)));
    return select(d2, d1, q.x < 1. && q.y < q.x * (k - h) + h);
} */

// 25) Egg ---------------------------------------------------------------------
fn sdEgg(p: vec2f, ra: f32, rb: f32) -> f32 {
    let k = sqrt(3.);
    let q = vec2f(abs(p.x), p.y);
    let r = ra - rb;
    let d1 = length(q) - r;
    let d2 = length(vec2f(q.x,  q.y - k * r));
    let d3 = length(vec2f(q.x + r, q.y)) - 2. * r;
    return select(select(d3, d2, k * (q.x + r) < q.y), d1, q.y < 0.) - rb;
}

/* // 26) Heart -------------------------------------------------------------------
fn sdHeart(p: vec2f) -> f32 {
    let q = vec2f(abs(p.x), p.y);
    let w = q - vec2f(0.25, 0.75);
    if (q.x + q.y > 1.0) { return sqrt(dot(w, w)) - sqrt(2.) / 4.; }
    let u = q - vec2f(0., 1.);
    let v = q - 0.5 * max(q.x + q.y, 0.);
    return sqrt(min(dot(u, u), dot(v, v))) * sign(q.x - q.y);
} */

// 27) Cross -------------------------------------------------------------------
fn sdCross(p: vec2f, b: vec2f) -> f32 {
    var q: vec2f = abs(p);
    q = select(q.xy, q.yx, q.y > q.x);
    let t = q - b;
    let k = max(t.y, t.x);
    let w = select(vec2f(b.y - q.x, -k), t, k > 0.);
    return sign(k) * length(max(w, vec2f(0.)));
}

// 28) Rounded X ---------------------------------------------------------------
fn sdRoundedX(p: vec2f, w: f32, r: f32) -> f32 {
    let q = abs(p);
    return length(q - min(q.x + q.y, w) * 0.5) - r;
}

/* // 29) Polygon -----------------------------------------------------------------
const N: i32 = 5;
fn sdPolygon(p: vec2f, v: ptr<function, array<vec2f, 5>>) -> f32 {
    let c = *v;
    var d = dot(p - c[0], p - c[0]);
    var s: f32 = 1.;
    for (var i: i32 = 0; i < N; i = i + 1) {
    let j = (i + 1) % N;
    let e = c[i] - c[j];
    let w = p - c[j];
    let b = w - e * clamp(dot(w, e) / dot(e, e), 0., 1.);
    d = min(d, dot(b, b));
    let c1 = p.y >= c[j].y;
    let c2 = p.y < c[i].y;
    let c3 = e.x * w.y > e.y * w.x;
    let c = vec3<bool>(c1, c2, c3);
    if (all(c) || all(!c)) { s = -s; };
    }
    return s * sqrt(d);
} */

// 30) Ellipse -----------------------------------------------------------------
fn sdEllipse(p: vec2f, ab: vec2f) -> f32 {
    var q: vec2f = abs(p);
    var e: vec2f = ab;
    if (q.x > q.y) {
    q = q.yx;
    e = ab.yx;
    }
    let l = e.y * e.y - e.x * e.x;
    let m = e.x * q.x / l;
    let m2 = m * m;
    let n = e.y * q.y / l;
    let n2 = n * n;
    let c = (m2 + n2 - 1.) / 3.;
    let c3 = c * c * c;
    let b = c3 + m2 * n2 * 2.;
    let d = c3 + m2 * n2;
    let g = m + m * n2;
    var co: f32;
    if (d < 0.) {
    let h = acos(b / c3) / 3.0;
    let s = cos(h);
    let t = sin(h) * sqrt(3.);
    let rx = sqrt(-c * (s + t + 2.0) + m2);
    let ry = sqrt(-c * (s - t + 2.0) + m2);
    co = (ry + sign(l) * rx + abs(g) / (rx * ry) - m) / 2.;
    } else {
    let h = 2. * m * n * sqrt(d);
    let s = sign(b + h) * pow(abs(b + h), 1. / 3.);
    let u = sign(b - h) * pow(abs(b - h), 1. / 3.);
    let rx = -s - u - c * 4. + 2. * m2;
    let ry = (s - u) * sqrt(3.);
    let rm = sqrt(rx * rx + ry * ry);
    co = (ry / sqrt(rm - rx) + 2. * g / rm - m) / 2.;
    }
    let r = e * vec2f(co, sqrt(1.0-co*co));
    return length(r - q) * sign(q.y - r.y);
}

// 31) Parabola ----------------------------------------------------------------
fn sdParabola(pos: vec2f, k: f32) -> f32 {
    let p = vec2f(abs(pos.x), pos.y);
    let ik = 1. / k;
    let u = ik * (p.y - 0.5 * ik) / 3.;
    let v = 0.25 * ik * ik * p.x;
    let h = v * v - u * u * u;
    let r = sqrt(abs(h));
    let x = select(2. * cos(atan2(r, v) / 3.) * sqrt(u),
    pow(v + r, 1. / 3.) - pow(abs(v - r), 1. / 3.) * sign(r - v),
    h > 0.0);
    return length(p - vec2f(x, k * x * x)) * sign(p.x - x);
}

// 32) Parabola Segment --------------------------------------------------------
fn sdParabolaSegment(pos: vec2f, wi: f32, he: f32) -> f32 {
    let p = vec2f(abs(pos.x), pos.y);
    let ik = wi * wi / he;
    let u = ik * (he - p.y - 0.5 * ik) / 3.;
    let v = p.x * ik * ik * 0.25;
    let h = v * v - u * u * u;
    let r = sqrt(abs(h));
    var x: f32 = select(2. * cos(atan(r / v) / 3.) * sqrt(u),
    pow(v + r, 1. / 3.) - pow(abs(v - r), 1. / 3.) * sign(r - v),
    h > 0.0);
    x = min(x, wi);
    return length(p - vec2f(x, he - x * x / ik)) * sign(ik * (p.y - he) + p.x * p.x);
}

// 33) Quadratic Bezier --------------------------------------------------------
fn sdBezier(p: vec2f, A: vec2f, B: vec2f, C: vec2f) -> vec2f {
    let a = B - A;
    let b = A - 2. * B + C;
    let c = a * 2.;
    let d = A - p;
    let kk = 1. / dot(b, b);
    let kx = kk * dot(a, b);
    let ky = kk * (2. * dot(a, a) + dot(d, b)) / 3.;
    let kz = kk * dot(d, a);

    let p1 = ky - kx * kx;
    let p3 = p1 * p1 * p1;
    let q = kx * (2.0 * kx * kx - 3.0 * ky) + kz;
    var h: f32 = q * q + 4. * p3;

    var res: vec2f;
    if (h >= 0.) {
    h = sqrt(h);
    let x = (vec2f(h, -h) - q) / 2.;
    let uv = sign(x) * pow(abs(x), vec2f(1. / 3.));
    let t = clamp(uv.x + uv.y - kx, 0., 1.);
    let f = d + (c + b * t) * t;
    res = vec2f(dot(f, f), t);
    } else {
    let z = sqrt(-p1);
    let v = acos(q / (p1 * z * 2.)) / 3.;
    let m = cos(v);
    let n = sin(v) * 1.732050808;
    let t = clamp(vec2f(m + m, -n - m) * z - kx, vec2f(0.0), vec2f(1.0));
    let f = d + (c + b * t.x) * t.x;
    var dis: f32 = dot(f, f);
    res = vec2f(dis, t.x);

    let g = d + (c + b * t.y) * t.y;
    dis = dot(g, g);
    res = select(res, vec2f(dis, t.y), dis < res.x);
    }
    res.x = sqrt(res.x);
    return res;
}

// 34) Blobby Cross ------------------------------------------------------------
fn sdBlobbyCross(pos: vec2f, he: f32) -> f32 {
    var p: vec2f = abs(pos);
    p = vec2(abs(p.x - p.y), 1. - p.x - p.y) / sqrt(2.);

    let u = (he - p.y - 0.25 / he) / (6. * he);
    let v = p.x / (he * he * 16.);
    let h = v * v - u * u * u;

    var x: f32;
    var y: f32;
    if (h > 0.) {
        let r = sqrt(h);
        x = pow(v + r, 1. / 3.) - pow(abs(v - r), 1. / 3.) * sign(r - v);
    } else {
        let r = sqrt(u);
        x = 2. * r * cos(acos(v / (u * r)) / 3.);
    }
    x = min(x, sqrt(2.) / 2.);

    let z = vec2f(x, he * (1. - 2. * x * x)) - p;
    return length(z) * sign(z.y);
}

// 35) Cut Disk ----------------------------------------------------------------
fn sdCutDisk(p: vec2f, r: f32, h: f32) -> f32 {
    let w = sqrt(r * r - h * h);    // constant for any given shape
    let q = vec2(abs(p.x), p.y);
    let s = max((h - r) * q.x * q.x + w * w * (h + r - 2.0 * q.y), h * q.x - w * q.y);
    return  select(
                select(
                    length(q - vec2(w, h)),
                    h - q.y,
                    q.x < w
                ),
                length(q) - r,
                s < 0.0
            );
}

// 36) Tunnel ------------------------------------------------------------------
// wh = (width, height)
fn sdTunnel(p: vec2f, wh: vec2f) -> f32 {
    let p2 = vec2(abs(p.x), -p.y);
    var q = p2 - wh;

    var d0 = vec2(max(q.x, 0.0), q.y);
    let d1 = dot(d0, d0);
    q.x =   select(
                length(p2) - wh.x,
                q.x,
                p2.y > 0.0
            );
    d0 = vec2(q.x, max(q.y, 0.0));
    let d2 = dot(d0, d0);
    let d = sqrt(min(d1, d2));

    return  select(
                d,
                -d,
                max(q.x, q.y) < 0.0
            );
}

// 37) Quadratic Circle --------------------------------------------------------
fn sdQuadraticCircle(p: vec2f, r: f32) -> f32 {
    var q = abs(p);
    if (q.y > q.x) {
        q = q.yx;
    }

    let a = q.x - q.y;
    let b = q.x + q.y;
    let c = (2.0 * b - 1.0) / 3.0;
    var h = a * a + c * c * c;
    var t = 0.0;
    if(h >= 0.0) {
        h = sqrt(h);
        t = sign(h - a) * pow(abs(h - a), 1.0 / 3.0) - pow(h + a, 1.0 / 3.0);
    } else {
        let z = sqrt(-c);
        let v = acos(a / (c * z)) / 3.0;
        t = -z * (cos(v) + sin(v) * 1.732050808);
    }
    t *= 0.5;
    let w = vec2(-t, t) + 0.75 - t * t - p;
    return length(w) * sign(a * a * 0.5 + b - 1.5) - r;
}

// 38) Arrow -------------------------------------------------------------------
// a = start of arrow
// b = end of arrow
// w1 = arrow shaft thickness
// w2 = arrow head thickness
fn sdArrow(p: vec2f, a: vec2f, b: vec2f, w1: f32, w2: f32) -> f32 {
    // constant setup
    let k = 3.0;    // arrow head ratio
	let ba = b - a;
    let l2 = dot(ba, ba);
    let l = sqrt(l2);

    // pixel setup
    var pix = p - a;
    pix = mat2x2<f32>(ba.x, -ba.y, ba.y, ba.x) * pix / l;
    pix.y = abs(pix.y);
    let pz = pix - vec2(l - w2 * k, w2);

    // === distance (four segments) ===

    var q = pix;
    q.x -= clamp(q.x, 0.0, l - w2 * k);
    q.y -= w1;
    var di = dot(q,q);
    //----
    q = pz;
    q.y -= clamp(q.y, w1-w2, 0.0);
    di = min(di, dot(q,q));
    //----
    if(pix.x < w1) {
        q = pix;
        q.y -= clamp(q.y, 0.0, w1);
        di = min(di, dot(q,q));
    }
    //----
    if(pz.x > 0.0) {
        q = pz;
        q -= vec2(k, -1.0) * clamp((q.x * k - q.y) / (k * k + 1.0), 0.0, w2);
        di = min(di, dot(q,q));
    }

    // === sign ===

    var si = 1.0;
    let z = l - pix.x;
    //if( pix.x>0.0 && z>0.0 )
    if(min(pix.x, z) > 0.0) {
        let h = select(z / k, w1, pz.x < 0.0);
        if(pix.y < h) {
            si = -1.0;
        }
    }
    return si * sqrt(di);
}

// 39) Stairs ------------------------------------------------------------------
fn sdStairs(p: vec2f, wh: vec2f, n: f32) -> f32 {
    let ba = wh * n;
    let d01 = p - vec2(clamp(p.x, 0.0, ba.x), 0.0);
    let d02 = p - vec2(ba.x, clamp(p.y, 0.0, ba.y));
    var d = min(dot(d01, d01), dot(d02, d02));
    var s = sign(max(-p.y, p.x - ba.x));

    let dia = length(wh);
    var q = mat2x2<f32>(wh.x, -wh.y, wh.y, wh.x) * p / dia;
    let id = clamp(round(q.x / dia), 0.0, n - 1.0);
    q.x = q.x - id * dia;
    q = mat2x2<f32>(wh.x, wh.y, -wh.y, wh.x) * p / dia;

    let hh = wh.y / 2.0;
    q.y -= hh;
    if(q.y > hh * sign(q.x)) {
        s = 1.0;
    }
    q = select(-q, q, id < 0.5 || q.x > 0.0);
    var d03 = q - vec2(0.0, clamp(q.y, -hh, hh));
    d = min(d, dot(d03, d03));
    d03 = q - vec2(clamp(q.x, 0.0, wh.x), hh);
    d = min(d, dot(d03, d03));

    return sqrt(d) * s;
}
