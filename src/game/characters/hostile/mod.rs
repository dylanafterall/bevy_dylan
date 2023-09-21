mod rainbow_square;
mod star;

use rainbow_square::RainbowSquarePlugin;
use star::StarPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct HostilePlugin;

impl Plugin for HostilePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RainbowSquarePlugin, StarPlugin));
    }
}
