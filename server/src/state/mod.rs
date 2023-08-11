use bevy::prelude::States;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ServerState {
    #[default]
    StartUp,
    LoadWorld,
    LoadPhysics,
    Running,
}
