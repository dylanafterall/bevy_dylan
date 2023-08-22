mod systems;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // resources

            // plugins
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
                RapierDebugRenderPlugin::default(),
            ))
            // events

            // systems: startup
            .add_systems(Startup, (
                systems::setup_physics,
            ))
            // systems: update

            // launch app
            .run();
    }
}