mod firework;
mod force_field;
mod infinity;
mod portal;
mod whirlwind;

use firework::FireworkPlugin;
use force_field::ForceFieldPlugin;
use infinity::InfinityPlugin;
use portal::PortalPlugin;
use whirlwind::WhirlwindPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FireworkPlugin,
            ForceFieldPlugin,
            InfinityPlugin,
            PortalPlugin,
            WhirlwindPlugin,
        ));
    }
}
