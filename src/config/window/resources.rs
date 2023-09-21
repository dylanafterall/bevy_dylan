use bevy::prelude::*;

// -----------------------------------------------------------------------------
#[derive(Resource)]
pub struct ResolutionSettings {
    pub _1024_x_768: Vec2,  // 4:3          (XGA - Apple iPad)
    pub _1280_x_1024: Vec2, // 5:4          (SXGA)
    pub _1280_x_720: Vec2,  // 16:9         (WXGA 720p)
    pub _1280_x_800: Vec2,  // 8:5          (WXGA)
    pub _1360_x_768: Vec2,  // ~16:9
    pub _1366_x_768: Vec2,  // ~16:9        (WXGA HD)
    pub _1440_x_900: Vec2,  // 8:5          (WSXGA)
    pub _1600_x_900: Vec2,  // 16:9         (HD+ 900p)
    pub _1680_x_1050: Vec2, // 8:5          (WSXGA+)
    pub _1920_x_1200: Vec2, // 8:5          (WUXGA)
    pub _1920_x_1080: Vec2, // 16:9         (FHD 1080p)
    pub _2560_x_1440: Vec2, // 16:9         (WQHD 2K)
    pub _2560_x_1600: Vec2, // 8:5          (WQXGA)
    pub _2560_x_1080: Vec2, // 21:9         (UW-FHD)
    pub _2880_x_1800: Vec2, // 8:5          (MBP Retina)
    pub _3440_x_1440: Vec2, // 21:9         (UW-QHD)
    pub _3840_x_2160: Vec2, // 16:9         (4K UHD-1)
    pub _5120_x_1440: Vec2, // 32:9         (DQHD)
}

impl Default for ResolutionSettings {
    fn default() -> ResolutionSettings {
        ResolutionSettings {
            _1024_x_768: Vec2::new(1024.0, 768.0),
            _1280_x_1024: Vec2::new(1280.0, 1024.0),
            _1280_x_720: Vec2::new(1280.0, 720.0),
            _1280_x_800: Vec2::new(1280.0, 800.0),
            _1360_x_768: Vec2::new(1360.0, 768.0),
            _1366_x_768: Vec2::new(1366.0, 768.0),
            _1440_x_900: Vec2::new(1440.0, 900.0),
            _1600_x_900: Vec2::new(1600.0, 900.0),
            _1680_x_1050: Vec2::new(1680.0, 1050.0),
            _1920_x_1200: Vec2::new(1920.0, 1200.0),
            _1920_x_1080: Vec2::new(1920.0, 1080.0),
            _2560_x_1440: Vec2::new(2560.0, 1440.0),
            _2560_x_1600: Vec2::new(2560.0, 1600.0),
            _2560_x_1080: Vec2::new(2560.0, 1080.0),
            _2880_x_1800: Vec2::new(2880.0, 1800.0),
            _3440_x_1440: Vec2::new(3440.0, 1440.0),
            _3840_x_2160: Vec2::new(3840.0, 2160.0),
            _5120_x_1440: Vec2::new(5120.0, 1440.0),
        }
    }
}
