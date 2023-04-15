use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::{App, Camera2dBundle, Commands, EventReader, Input, MouseButton, Plugin, Query, Res, Transform, With, Without};
use bevy_render::prelude::{Camera, OrthographicProjection};
use crate::debug::{cursor_pos_system, init_cursor_pos_system};
use crate::player::components::ServerPlayer;

const ZOOM_SPEED: f32 = 0.05;
const SCAN_SPEED: f32 = 0.5;

pub struct DebugServerPlugin;
impl Plugin for DebugServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(init_cursor_pos_system)
            .add_system(cursor_pos_system)
            .add_system(zooming)
            .add_system(scanning);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn zooming(mut camera_query: Query<&mut OrthographicProjection, (With<Camera>, Without<ServerPlayer>)>, mut scroll_evr: EventReader<MouseWheel>) {
    let mut orthographic_projection = camera_query.single_mut();
    // zooming
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                orthographic_projection.scale -= ZOOM_SPEED * ev.y;
                if orthographic_projection.scale < 0.0 {
                    orthographic_projection.scale = 0.0;
                }
            }
            MouseScrollUnit::Pixel => {
                orthographic_projection.scale -= ZOOM_SPEED * ev.y;
                if orthographic_projection.scale < 0.0 {
                    orthographic_projection.scale = 0.0;
                }
            }
        }
    }
}

fn scanning(mut camera_query: Query<&mut Transform, (With<Camera>, Without<ServerPlayer>)>, buttons: Res<Input<MouseButton>>, mut motion_evr: EventReader<MouseMotion>) {
    if buttons.pressed(MouseButton::Left) {
        // Left Button is being held down
        let mut camera_transform = camera_query.single_mut();

        for ev in motion_evr.iter() {
            //println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
            camera_transform.translation.x -= ev.delta.x * SCAN_SPEED;
            camera_transform.translation.y += ev.delta.y * SCAN_SPEED;
        }
    }
}