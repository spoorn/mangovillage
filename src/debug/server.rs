/// Largely taken from https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html.

use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::{AmbientLight, App, AssetServer, Camera3dBundle, Commands, debug, default, EventReader, Handle, Input, KeyCode, Mat3, MouseButton, Plugin, Quat, Query, Reflect, Res, ResMut, Scene, Transform, Vec2, Vec3, Window, With, Without};
use bevy::window::PrimaryWindow;
use bevy_render::prelude::{Camera, Color, Projection, Visibility};

use crate::debug::components::PanOrbitCamera;
use crate::debug::resources::{MeshVisibility, ZoomSpeed};
use crate::player::components::ServerPlayer;

const PAN_SPEED: f32 = 1.0;
const ORBIT_SPEED: f32 = 2.0;

pub struct DebugServerPlugin;
impl Plugin for DebugServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
            .insert_resource(MeshVisibility { visible: true })
            .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .insert_resource(ZoomSpeed::default())
            .add_startup_system(setup_camera)
            .add_system(toggle_visibility)
            .add_system(update_zoom_speed)
            // .add_startup_system(init_cursor_pos_system)
            // .add_system(cursor_pos_system)
            .add_system(zoom)
            .add_system(pan)
            .add_system(orbit);
    }
}

fn toggle_visibility(buttons: Res<Input<KeyCode>>, mut query: Query<(&mut Visibility), With<Handle<Scene>>>, mut mesh_vis: ResMut<MeshVisibility>) {
    if buttons.just_pressed(KeyCode::H) {
        mesh_vis.visible = !mesh_vis.visible;
        let visibility = if mesh_vis.visible { Visibility::Inherited } else { Visibility::Hidden };
        for (mut vis) in query.iter_mut() {
            vis.apply(&visibility);
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let camera_translation = Vec3::new(0.0, 0.0, 20.0);
    let focus = Vec3::new(0.0, 0.0, 0.0);
    commands.spawn((
       Camera3dBundle {
            transform: Transform::from_translation(camera_translation)
                .looking_at(focus, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            focus,
            radius: (camera_translation - focus).length(),
            ..default()
        }
    ));
}

fn update_zoom_speed(mut zoom_speed: ResMut<ZoomSpeed>, buttons: Res<Input<KeyCode>>) {
    let mut changed = false;
    if buttons.just_pressed(KeyCode::LBracket) {
        zoom_speed.speed = f32::max(0.01, zoom_speed.speed - 0.5);
        changed = true;
    } else if buttons.just_pressed(KeyCode::RBracket) {
        zoom_speed.speed += 0.5;
        changed = true;
    }
    if changed {
        debug!("Update zoom speed to {}", zoom_speed.speed);
    }
}

fn zoom(mut camera_query: Query<(&mut PanOrbitCamera, &mut Transform), (With<Camera>, Without<ServerPlayer>)>, mut scroll_evr: EventReader<MouseWheel>, zoom_speed: Res<ZoomSpeed>) {
    let (mut pan_orbit, mut transform) = camera_query.single_mut();
    
    let scroll: f32 = scroll_evr.iter().map(|ev| ev.y).sum::<f32>() * zoom_speed.speed;

    if scroll.abs() > 0.0 {
        pan_orbit.radius -= scroll;
        pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        update_camera_transform(&mut transform, &pan_orbit);
    }
    
    scroll_evr.clear();
}

fn pan(windows: Query<&Window, With<PrimaryWindow>>, mut camera_query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection), (With<Camera>, Without<ServerPlayer>)>, buttons: Res<Input<MouseButton>>, mut motion_evr: EventReader<MouseMotion>) {
    if buttons.pressed(MouseButton::Middle) {
        let mut pan = motion_evr.iter().map(|ev| ev.delta).sum::<Vec2>() * PAN_SPEED;
        
        if pan.length_squared() > 0.0 {
            let (mut pan_orbit, mut transform, projection) = camera_query.single_mut();
            
            // make panning distance independent of resolution and FOV
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;

            update_camera_transform(&mut transform, &pan_orbit);
        }
    }
}

// https://math.stackexchange.com/questions/360286/what-does-multiplication-of-two-quaternions-give
fn orbit(windows: Query<&Window, With<PrimaryWindow>>, mut camera_query: Query<(&PanOrbitCamera, &mut Transform, &Projection), (With<Camera>, Without<ServerPlayer>)>, buttons: Res<Input<MouseButton>>, mut motion_evr: EventReader<MouseMotion>) {
    if buttons.pressed(MouseButton::Right) {
        let mut rotation_move = motion_evr.iter().map(|ev| ev.delta).sum::<Vec2>() * ORBIT_SPEED;
        
        if rotation_move.length_squared() > 0.0 {
            let (pan_orbit, mut transform, projection) = camera_query.single_mut();

            let window = get_primary_window_size(&windows);
            // Make rotation speed independent of resolution and fov
            if let Projection::Perspective(projection) = projection {
                rotation_move *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            
            // I think multiplying by PI and 2.0 was arbitrary from the original example?
            let delta_x = {
                //let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                let delta = rotation_move.x;
                if pan_orbit.upside_down { -delta } else { delta }
            };

            let delta_y = rotation_move.y; //rotation_move.y / window.y * std::f32::consts::PI;
            // negative to flip direction
            let yaw = Quat::from_rotation_z(-delta_x);  // Top down view where +Z is camera view, so we rotate/spin around Z
            let pitch = Quat::from_rotation_x(-delta_y);
            // Rotation matrix rotates the radius vector - right to left
            // https://forum.unity.com/threads/understanding-rotations-in-local-and-world-space-quaternions.153330/
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
            
            update_camera_transform(&mut transform, pan_orbit);
        }
    }
}

#[inline]
fn get_primary_window_size(windows: &Query<&Window, With<PrimaryWindow>>) -> Vec2 {
    let window = windows.get_single().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

#[inline]
fn update_camera_transform(transform: &mut Transform, pan_orbit: &PanOrbitCamera) {
    let rot_matrix = Mat3::from_quat(transform.rotation);
    // Rotation matrix rotates the radius vector - right to left
    // https://forum.unity.com/threads/understanding-rotations-in-local-and-world-space-quaternions.153330/
    transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
}