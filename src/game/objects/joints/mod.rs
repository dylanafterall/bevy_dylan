mod fixed_joint;
mod prismatic_joint;
mod revolute_joint;

use fixed_joint::FixedJointPlugin;
use prismatic_joint::PrismaticJointPlugin;
use revolute_joint::RevoluteJointPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct JointsPlugin;

impl Plugin for JointsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                FixedJointPlugin,
                PrismaticJointPlugin,
                RevoluteJointPlugin,
            ));
    }
}