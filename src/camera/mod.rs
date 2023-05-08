use bevy::app::App;
use bevy::prelude::{Camera3dBundle, Commands, default, Plugin, Transform, Vec3};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
            //.add_system(follow_player.in_set(OnUpdate(ClientState::Running)));
    }
}

fn setup_camera(mut commands: Commands) {
    let camera_translation = Vec3::new(0.0, 0.0, 50.0);
    let focus = Vec3::new(0.0, 0.0, 0.0);
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(camera_translation)
                .looking_at(focus, Vec3::Y),
            ..default()
        },
    ));
}

const ASPECT_RATIO: f32 = 16. / 9.;

// TODO: clean up and optimize
// TODO: Handle narrow maps
// fn follow_player(mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), (With<Camera>, Without<ClientPlayer>)>, player_query: Query<&Transform, (With<ClientPlayer>, With<Me>)>,
//                  level_query: Query<
//                      (&Transform, &Handle<LdtkLevel>),
//                      (Without<OrthographicProjection>, Without<ClientPlayer>),
//                  >,
//                  level_selection: Res<LevelSelection>,
//                  ldtk_levels: Res<Assets<LdtkLevel>>,) {
//     if let Ok(Transform {
//         translation: player_translation,
//         ..
//     }) = player_query.get_single() {
//         let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();
//         let player_translation = *player_translation;
//         
//         for (level_transform, level_handle) in level_query.iter() {
//             if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
//                 let level = &ldtk_level.level;
//                 if level_selection.is_match(&0, level) {
//                     let level_ratio = level.px_wid as f32 / level.px_hei as f32;
//                     
//                     if level_ratio > ASPECT_RATIO {
//                         // level is wider than the screen
//                         let y = (level.px_hei as f32 / 9.).round() * 9.;
//                         let x = y * ASPECT_RATIO;
//                         orthographic_projection.scaling_mode = ScalingMode::Fixed { 
//                             width: f32::min(x, 320.0), height: f32::min(y, 180.0)
//                         };
//                         // orthographic_projection.area.max.y = y/2.0;
//                         // orthographic_projection.area.max.x = x/2.0;
//                         // orthographic_projection.area.min.y = -y/2.0;
//                         // orthographic_projection.area.min.x = -x/2.0;
//                     } else {
//                         // level is taller than the screen
//                         let x = (level.px_wid as f32 / 16.).round() * 16.;
//                         let y = x / ASPECT_RATIO;
//                         orthographic_projection.scaling_mode = ScalingMode::Fixed {
//                             width: f32::min(x, 320.0), height: f32::min(y, 180.0)
//                         };
//                         // orthographic_projection.area.max.y = x/2.0;
//                         // orthographic_projection.area.max.x = y/2.0;
//                         // orthographic_projection.area.min.y = -x/2.0;
//                         // orthographic_projection.area.min.x = -y/2.0;
//                     }
//                     
//                     // orthographic_project is negative for left/bottom, positive for right/up
//                     // orthographic_projection with ScalingMode None is the px width/height shift of the camera from
//                     // relative camera's (0, 0).  Essentially, think of it as like the padding on top/bottom/left/right
//                     // from the center.  Level spawns with bottom left corner at (0, 0) world coordinates.
//                     //
//                     // camera.x - ortho.right > level.x
//                     // camera.x + ortho.right < level.x + level.width
//                     // camera.y - ortho.top > level.y
//                     // camera.y + ortho.top < level.y + level.height
//                     
//                     // Max the projection to 160x90 so maps have consistent zoom scale
//                     // orthographic_projection.area.max.y = f32::min(orthographic_projection.area.max.y, 90.0);
//                     // orthographic_projection.area.min.y = f32::max(orthographic_projection.area.min.y, -90.0);
//                     // orthographic_projection.area.min.x = f32::max(orthographic_projection.area.min.x, -160.0);
//                     // orthographic_projection.area.max.x = f32::min(orthographic_projection.area.max.x, 160.0);
//                     if let ScalingMode::Fixed { width, height } = orthographic_projection.scaling_mode {
//                         camera_transform.translation.x = f32::min(player_translation.x, level_transform.translation.x + level.px_wid as f32 - width / 2.0);
//                         camera_transform.translation.x = f32::max(camera_transform.translation.x, level_transform.translation.x + width / 2.0);
//                         camera_transform.translation.y = f32::min(player_translation.y, level_transform.translation.y + level.px_hei as f32 - height / 2.0);
//                         camera_transform.translation.y = f32::max(camera_transform.translation.y, level_transform.translation.y + height / 2.0);
//                     }
//                 }
//             }
//         }
//     }
// }