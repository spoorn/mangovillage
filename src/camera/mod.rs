use bevy::app::App;
use bevy::prelude::{Camera2dBundle, Commands, Plugin, Query, Transform, With, Without};
use bevy_render::prelude::{Camera, OrthographicProjection};
use crate::player::components::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(follow_player);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn follow_player(mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>, player_query: Query<&Transform, With<Player>>) {
    let mut cam_trans = camera_query.single_mut();
    if !player_query.is_empty() {
        let player_trans = player_query.single();
        cam_trans.translation = player_trans.translation;
    }
}