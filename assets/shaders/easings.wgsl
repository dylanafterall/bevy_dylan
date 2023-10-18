#import bevy_pbr::utils     PI, HALF_PI

// Robert Penner's easing functions in GLSL
// https://github.com/stackgl/glsl-easings

fn lin(t: f32) -> f32 {
    return t;
}

fn exponentialIn(t: f32) -> f32 {
    return  select(
                pow(2.0, 10.0 * (t - 1.0)),     // F
                t,                              // T
                t == 0.0                        // cond
            );
}

fn exponentialOut(t: f32) -> f32 {
    return  select(
                1.0 - pow(2.0, -10.0 * t),
                t,
                t == 1.0
            );
}

fn exponentialInOut(t: f32) -> f32 {
    return  select(
                select(
                    -0.5 * pow(2.0, 10.0 - (t * 20.0)) + 1.0,
                    0.5 * pow(2.0, (20.0 * t) - 10.0),
                    t < 0.5
                ),
                t,
                t == 0.0 || t == 1.0
            );
}

fn sineIn(t: f32) -> f32 {
    return sin((t - 1.0) * HALF_PI) + 1.0;
}

fn sineOut(t: f32) -> f32 {
    return sin(t * HALF_PI);
}

fn sineInOut(t: f32) -> f32 {
    return -0.5 * (cos(PI * t) - 1.0);
}

fn qinticIn(t: f32) -> f32 {
    return pow(t, 5.0);
}

fn qinticOut(t: f32) -> f32 {
    return 1.0 - (pow(t - 1.0, 5.0));
}

fn qinticInOut(t: f32) -> f32 {
    return  select(
                -0.5 * pow(2.0 * t - 2.0, 5.0) + 1.0,
                16.0 * pow(t, 5.0),
                t < 0.5
            );
}

fn quarticIn(t: f32) -> f32 {
    return pow(t, 4.0);
}

fn quarticOut(t: f32) -> f32 {
    return pow(t - 1.0, 3.0) * (1.0 - t) + 1.0;
}

fn quarticInOut(t: f32) -> f32 {
    return  select(
                -8.0 * pow(t - 1.0, 4.0) + 1.0,
                8.0 * pow(t, 4.0),
                t < 0.5
            );
}

fn quadraticInOut(t: f32) -> f32 {
    let p = 2.0 * t * t;
    return  select(
                -p + (4.0 * t) - 1.0,
                p,
                t < 0.5
            );
}

fn quadraticIn(t: f32) -> f32 {
    return t * t;
}

fn quadraticOut(t: f32) -> f32 {
    return -t * (t - 2.0);
}

fn cubicIn(t: f32) -> f32 {
    return t * t * t;
}

fn cubicOut(t: f32) -> f32 {
    let f = t - 1.0;
    return f * f * f + 1.0;
}

fn cubicInOut(t: f32) -> f32 {
    return  select(
                0.5 * pow(2.0 * t - 2.0, 3.0) + 1.0,
                4.0 * t * t * t,
                t < 0.5
            );
}

fn elasticIn(t: f32) -> f32 {
    return sin(13.0 * t * HALF_PI) * pow(2.0, 10.0 * (t - 1.0));
}

fn elasticOut(t: f32) -> f32 {
    return sin(-13.0 * (t + 1.0) * HALF_PI) * pow(2.0, -10.0 * t) + 1.0;
}

fn elasticInOut(t: f32) -> f32 {
    return  select(
                0.5 * sin(-13.0 * HALF_PI * ((2.0 * t - 1.0) + 1.0)) * pow(2.0, -10.0 * (2.0 * t - 1.0)) + 1.0,
                0.5 * sin(13.0 * HALF_PI * 2.0 * t) * pow(2.0, 10.0 * (2.0 * t - 1.0)),
                t < 0.5
            );
}

fn circularIn(t: f32) -> f32 {
    return 1.0 - sqrt(1.0 - t * t);
}

fn circularOut(t: f32) -> f32 {
    return sqrt((2.0 - t) * t);
}

fn circularInOut(t: f32) -> f32 {
    return  select(
                0.5 * (sqrt((3.0 - 2.0 * t) * (2.0 * t - 1.0)) + 1.0),
                0.5 * (1.0 - sqrt(1.0 - 4.0 * t * t)),
                t < 0.5
            );
}

const BOUNCE_A = 0.363636;
const BOUNCE_B = 0.727273;
const BOUNCE_C = 0.900000;
const BOUNCE_CA = 12.066482;
const BOUNCE_CB = 19.635457;
const BOUNCE_CC = 8.898061;

fn bounceOut(t: f32) -> f32 {
    let t2 = t * t;

    return  select(
                select(
                    select(
                        10.8 * t * t - 20.52 * t + 10.72,
                        BOUNCE_CA * t2 - BOUNCE_CB * t + BOUNCE_CC,
                        t < BOUNCE_C
                    ),
                    9.075 * t2 - 9.9 * t + 3.4,
                    t < BOUNCE_B
                ),
                7.5625 * t2,
                t < BOUNCE_A
            );
}

fn bounceIn(t: f32) -> f32 {
    return 1.0 - bounceOut(1.0 - t);
}

fn bounceInOut(t: f32) -> f32 {
    return  select(
                0.5 * bounceOut(t * 2.0 - 1.0) + 0.5,
                0.5 * (1.0 - bounceOut(1.0 - t * 2.0)),
                t < 0.5
            );
}

fn backIn(t: f32) -> f32 {
    return pow(t, 3.0) - t * sin(t * PI);
}

fn backOut(t: f32) -> f32 {
    let f = 1.0 - t;
    return 1.0 - (pow(f, 3.0) - f * sin(f * PI));
}

fn backInOut(t: f32) -> f32 {
    let f = select(
                1.0 - (2.0 * t - 1.0),
                2.0 * t,
                t < 0.5
            );

    let g = pow(f, 3.0) - f * sin(f * PI);

    return  select(
                0.5 * (1.0 - g) + 0.5,
                0.5 * g,
                t < 0.5
            );
}