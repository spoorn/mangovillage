use bevy::app::App;
use bevy::prelude::{Camera2dBundle, Commands, Plugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}