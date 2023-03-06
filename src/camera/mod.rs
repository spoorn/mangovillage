use bevy::app::App;
use bevy::prelude::{Assets, Camera2dBundle, Commands, Handle, Plugin, Query, Res, SystemSet, Transform, With, Without};
use bevy_ecs_ldtk::{LdtkLevel, LevelSelection};
use bevy_render::prelude::{Camera, OrthographicProjection};

use crate::player::components::{Me, ClientPlayer};
use crate::state::ClientState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system_set(SystemSet::on_update(ClientState::Running).with_system(follow_player));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const ASPECT_RATIO: f32 = 16. / 9.;

// TODO: clean up and optimize
fn follow_player(mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), (With<Camera>, Without<ClientPlayer>)>, player_query: Query<&Transform, (With<ClientPlayer>, With<Me>)>,
                 level_query: Query<
                     (&Transform, &Handle<LdtkLevel>),
                     (Without<OrthographicProjection>, Without<ClientPlayer>),
                 >,
                 level_selection: Res<LevelSelection>,
                 ldtk_levels: Res<Assets<LdtkLevel>>,) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single() {
        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();
        let player_translation = *player_translation;
        
        for (level_transform, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let level_ratio = level.px_wid as f32 / level.px_hei as f32;
                    
                    orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                    if level_ratio > ASPECT_RATIO {
                        // level is wider than the screen
                        let y = (level.px_hei as f32 / 9.).round() * 9.;
                        let x = y * ASPECT_RATIO;
                        orthographic_projection.top = y/2.0;
                        orthographic_projection.right = x/2.0;
                        orthographic_projection.bottom = -y/2.0;
                        orthographic_projection.left = -x/2.0;
                    } else {
                        // level is taller than the screen
                        let y = (level.px_wid as f32 / 16.).round() * 16.;
                        let x = y / ASPECT_RATIO;
                        orthographic_projection.top = x/2.0;
                        orthographic_projection.right = y/2.0;
                        orthographic_projection.bottom = -x/2.0;
                        orthographic_projection.left = -y/2.0;
                    }
                    
                    // orthographic_project is negative for left/bottom, positive for right/up
                    // orthographic_projection with ScalingMode None is the px width/height shift of the camera from
                    // relative camera's (0, 0).  Essentially, think of it as like the padding on top/bottom/left/right
                    // from the center.  Level spawns with bottom left corner at (0, 0) world coordinates.
                    //
                    // camera.x - ortho.right > level.x
                    // camera.x + ortho.right < level.x + level.width
                    // camera.y - ortho.top > level.y
                    // camera.y + ortho.top < level.y + level.height
                    
                    // Max the projection to 160x90 so maps have consistent zoom scale
                    orthographic_projection.top = f32::min(orthographic_projection.top, 90.0);
                    orthographic_projection.bottom = f32::max(orthographic_projection.bottom, -90.0);
                    orthographic_projection.left = f32::max(orthographic_projection.left, -160.0);
                    orthographic_projection.right = f32::min(orthographic_projection.right, 160.0);
                    
                    camera_transform.translation.x = f32::min(player_translation.x, level_transform.translation.x + level.px_wid as f32 - orthographic_projection.right);
                    camera_transform.translation.x = f32::max(camera_transform.translation.x, level_transform.translation.x + orthographic_projection.right);
                    camera_transform.translation.y = f32::min(player_translation.y, level_transform.translation.y + level.px_hei as f32 - orthographic_projection.top);
                    camera_transform.translation.y = f32::max(camera_transform.translation.y, level_transform.translation.y + orthographic_projection.top);
                }
            }
        }
    }
}