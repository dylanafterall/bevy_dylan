mod bucket;
mod bumper;
pub mod conveyor_belt;
mod elevator;
mod grav_shift;
pub mod joints;
pub mod one_way_platform;
mod platform;
mod rope;

use bucket::BucketPlugin;
use bumper::BumperPlugin;
use conveyor_belt::ConveyorBeltPlugin;
use elevator::ElevatorPlugin;
use grav_shift::GravShiftPlugin;
use joints::JointsPlugin;
use one_way_platform::OneWayPlatformPlugin;
use platform::PlatformPlugin;
use rope::RopePlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BumperPlugin,
            ConveyorBeltPlugin,
            ElevatorPlugin,
            GravShiftPlugin,
            BucketPlugin,
            JointsPlugin,
            OneWayPlatformPlugin,
            PlatformPlugin,
            RopePlugin,
        ));
    }
}
