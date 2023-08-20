use bevy::prelude::States;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ClientState {
    #[default]
    JoiningServer,
    LoadingLevel,
    LoadingPhysics,
    Running,
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum CameraState {
    Locked,
    #[default]
    Debug,
}
