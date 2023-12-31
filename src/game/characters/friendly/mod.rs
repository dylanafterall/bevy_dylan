mod ball;
pub mod bloom_triangle;
mod elastic;
mod pentagon;

use ball::BallPlugin;
use bloom_triangle::BloomTrianglePlugin;
use elastic::ElasticPlugin;
use pentagon::PentagonPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct FriendlyPlugin;

impl Plugin for FriendlyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BallPlugin,
            PentagonPlugin,
            BloomTrianglePlugin,
            ElasticPlugin,
        ));
    }
}
