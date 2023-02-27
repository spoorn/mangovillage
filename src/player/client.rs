use bevy::app::App;
use bevy::prelude::{AssetServer, Commands, default, Plugin, Res, Sprite, Vec2};
use bevy::sprite::SpriteBundle;

pub struct PlayerClientPlugin;
impl Plugin for PlayerClientPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(32.0)),
            ..default()
        },
        texture: asset_server.load("icon/test.png"),
        ..default()
    });
}
