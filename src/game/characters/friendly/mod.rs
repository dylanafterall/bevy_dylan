mod ball;
mod round_convex;

use ball::BallPlugin;
use round_convex::RoundConvexPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct FriendlyPlugin;

impl Plugin for FriendlyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                BallPlugin,
                RoundConvexPlugin,
            ));
    }
}