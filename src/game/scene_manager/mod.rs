mod systems;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_state::<SceneState>()
            .add_systems(Update, (systems::handle_scene_transition,))
            .add_systems(OnEnter(SceneState::First), (systems::spawn_scene_sensors,))
            .add_systems(OnEnter(SceneState::Second), (systems::spawn_scene_sensors,))
            .add_systems(OnEnter(SceneState::Third), (systems::spawn_scene_sensors,))
            .add_systems(OnEnter(SceneState::Fourth), (systems::spawn_scene_sensors,))
            .add_systems(OnEnter(SceneState::Fifth), (systems::spawn_scene_sensors,))
            .add_systems(OnEnter(SceneState::Sixth), (systems::spawn_scene_sensors,))
            .add_systems(
                OnEnter(SceneState::Seventh),
                (systems::spawn_scene_sensors,),
            )
            .add_systems(OnEnter(SceneState::Eighth), (systems::spawn_scene_sensors,))
            .add_systems(OnEnter(SceneState::Ninth), (systems::spawn_scene_sensors,))
            .add_systems(OnEnter(SceneState::Zero), (systems::spawn_scene_sensors,))
            .add_systems(OnExit(SceneState::First), (systems::despawn_entities,))
            .add_systems(OnExit(SceneState::Second), (systems::despawn_entities,))
            .add_systems(OnExit(SceneState::Third), (systems::despawn_entities,))
            .add_systems(OnExit(SceneState::Fourth), (systems::despawn_entities,))
            .add_systems(OnExit(SceneState::Fifth), (systems::despawn_entities,))
            .add_systems(OnExit(SceneState::Sixth), (systems::despawn_entities,))
            .add_systems(OnExit(SceneState::Seventh), (systems::despawn_entities,))
            .add_systems(OnExit(SceneState::Eighth), (systems::despawn_entities,))
            .add_systems(OnExit(SceneState::Ninth), (systems::despawn_entities,))
            .add_systems(OnExit(SceneState::Zero), (systems::despawn_entities,));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SceneState {
    Inert,
    #[default]
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Zero,
}
