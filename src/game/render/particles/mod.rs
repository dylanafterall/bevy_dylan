mod firework;
mod infinity;
mod portal;
mod red_particles;

// use firework::FireworkPlugin;
// use infinity::InfinityPlugin;
use portal::PortalPlugin;
// use red_particles::RedParticlesPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // FireworkPlugin,
            // InfinityPlugin,
            PortalPlugin,
            // RedParticlesPlugin,
        ));
    }
}
