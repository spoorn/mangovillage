use crate::debug::component::PanOrbitCamera;
use crate::debug::resource::{MeshVisibility, ZoomSpeed};
use crate::state::CameraState;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::DebugRenderContext;

const PAN_SPEED: f32 = 1.0;
const ORBIT_SPEED: f32 = 2.0;

pub struct DebugCameraPlugin;
impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CameraState>()
            .insert_resource(MeshVisibility { visible: true })
            .insert_resource(ZoomSpeed::default())
            .add_systems(Startup, setup_camera)
            .add_systems(Update, toggle_debug)
            .add_systems(Update, (toggle_visibility, update_zoom_speed, zoom, pan, orbit).run_if(in_state(CameraState::Debug)));
    }
}

// Z is towards user, Y is vertical, X is horizontal
fn setup_camera(mut commands: Commands) {
    let camera_translation = Vec3::new(0.0, 0.0, 1000.0);
    let focus = Vec3::new(0.0, 0.0, 0.0);
    commands.spawn((
        Camera3dBundle { transform: Transform::from_translation(camera_translation).looking_at(focus, Vec3::Y), ..default() },
        PanOrbitCamera { focus, radius: (camera_translation - focus).length(), ..default() },
    ));
}

/// Toggle debug camera state
fn toggle_debug(
    buttons: Res<Input<KeyCode>>,
    camera_state: Res<State<CameraState>>,
    mut next_camera_state: ResMut<NextState<CameraState>>,
    mut mesh_vis: ResMut<MeshVisibility>,
    mut mesh_query: Query<&mut Visibility, With<Handle<Scene>>>,
    mut debug_render_context: ResMut<DebugRenderContext>,
) {
    if buttons.just_pressed(KeyCode::F1) {
        match camera_state.get() {
            CameraState::Locked => {
                next_camera_state.set(CameraState::Debug);
            }
            CameraState::Debug => {
                mesh_vis.visible = true;
                set_mesh_visibilities(&mut mesh_query, mesh_vis.visible);
                debug_render_context.enabled = false;
                next_camera_state.set(CameraState::Locked);
            }
        }
    }
}

/// Toggle visibility of meshes and debug render lines
fn toggle_visibility(
    buttons: Res<Input<KeyCode>>,
    mut mesh_query: Query<&mut Visibility, With<Handle<Scene>>>,
    mut mesh_vis: ResMut<MeshVisibility>,
    mut debug_render_context: ResMut<DebugRenderContext>,
) {
    if buttons.just_pressed(KeyCode::F2) {
        mesh_vis.visible = !mesh_vis.visible;
        set_mesh_visibilities(&mut mesh_query, mesh_vis.visible);
    }

    if buttons.just_pressed(KeyCode::F3) {
        debug_render_context.enabled = !debug_render_context.enabled;
    }
}

fn set_mesh_visibilities(mesh_query: &mut Query<&mut Visibility, With<Handle<Scene>>>, visible: bool) {
    let visibility = if visible { Visibility::Inherited } else { Visibility::Hidden };
    for mut vis in mesh_query.iter_mut() {
        vis.apply(&visibility);
    }
}

/// Update camera zoom speed
fn update_zoom_speed(mut zoom_speed: ResMut<ZoomSpeed>, buttons: Res<Input<KeyCode>>) {
    let mut changed = false;
    if buttons.pressed(KeyCode::BracketLeft) {
        zoom_speed.speed = f32::max(0.01, zoom_speed.speed - 5.0);
        changed = true;
    } else if buttons.pressed(KeyCode::BracketRight) {
        zoom_speed.speed += 5.0;
        changed = true;
    }
    if changed {
        debug!("Update zoom speed to {}", zoom_speed.speed);
    }
}

fn zoom(
    mut camera_query: Query<(&mut PanOrbitCamera, &mut Transform), With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
    zoom_speed: Res<ZoomSpeed>,
) {
    let (mut pan_orbit, mut transform) = camera_query.single_mut();

    let scroll: f32 = scroll_evr.iter().map(|ev| ev.y).sum::<f32>() * zoom_speed.speed;

    if scroll.abs() > 0.0 {
        pan_orbit.radius = f32::max(pan_orbit.radius - scroll, 0.05);
        update_camera_transform(&mut transform, &pan_orbit);
    }

    scroll_evr.clear();
}

fn pan(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection), With<Camera>>,
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
) {
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
fn orbit(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&PanOrbitCamera, &mut Transform, &Projection), With<Camera>>,
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    if buttons.pressed(MouseButton::Left) {
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
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };

            //rotation_move.y / window.y * std::f32::consts::PI;
            let delta_y = rotation_move.y;
            // negative to flip direction
            // yaw around Z axis
            // Top down view where +Z is camera view, so we rotate/spin around Z
            let yaw = Quat::from_rotation_z(-delta_x);
            // pitch around X axis
            let pitch = Quat::from_rotation_x(-delta_y);
            // Rotation matrix rotates the radius vector - right to left
            // https://forum.unity.com/threads/understanding-rotations-in-local-and-world-space-quaternions.153330/
            // Think of this as composition,
            // first we rotate the yaw around Z axis, then apply the camera rotation, so it rotates the correct global Z axis
            // second we rotate to where the camera is, then we rotate the pitch so it's the local camera's pitch
            transform.rotation = yaw * transform.rotation; // rotate around global z axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis

            update_camera_transform(&mut transform, pan_orbit);
        }
    }
}

#[inline]
fn get_primary_window_size(windows: &Query<&Window, With<PrimaryWindow>>) -> Vec2 {
    let window = windows.get_single().unwrap();
    Vec2::new(window.width(), window.height())
}

#[inline]
fn update_camera_transform(transform: &mut Transform, pan_orbit: &PanOrbitCamera) {
    let rot_matrix = Mat3::from_quat(transform.rotation);
    // Rotation matrix rotates the radius vector - right to left
    // https://forum.unity.com/threads/understanding-rotations-in-local-and-world-space-quaternions.153330/
    // snippet:
    // There is just one really simple rule you need to memorize: Order matters.
    //
    //     Rotate around a local axis: rotation = rotation * Quaternion.AngleAxis(10, Vector3.Up);
    //     Rotate around a world axis: rotation = Quaternion.AngleAxis(10, Vector3.Up) * rotation;
    //
    // So, as you can see above, putting the desired rotation last rotates around a local axis, putting it first rotates around a world axis. There's not much more to know about combining Quaternions.
    //     You also don't need to know the local axis nor transform any desired rotation axis.
    //     Simply chose the right combine order and you're golden.
    transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
}
