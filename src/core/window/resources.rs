use bevy::prelude::*;

// store window resolutions to choose between
#[derive(Resource)]
pub struct ResolutionSettings {
    pub _800_x_1280: Vec2,
    pub _1024_x_768: Vec2,
    pub _1280_x_1024: Vec2,
    pub _1280_x_720: Vec2,
    pub _1280_x_800: Vec2,
    pub _1360_x_768: Vec2,
    pub _1366_x_768: Vec2,
    pub _1440_x_900: Vec2,
    pub _1600_x_900: Vec2,
    pub _1680_x_1050: Vec2,
    pub _1920_x_1200: Vec2,
    pub _1920_x_1080: Vec2,
    pub _2560_x_1440: Vec2,
    pub _2560_x_1600: Vec2,
    pub _2560_x_1080: Vec2,
    pub _2880_x_1800: Vec2,
    pub _3440_x_1440: Vec2,
    pub _3840_x_2160: Vec2,
    pub _5120_x_1440: Vec2,
}