use bevy::prelude::{AssetServer, Commands, GlobalTransform, Plugin, Query, Res, Style, Text, TextBundle, TextStyle, UiRect, Val, Window, With};
use bevy::ui::PositionType;
use bevy::utils::default;
use bevy::window::PrimaryWindow;
use bevy_render::prelude::{Camera, Color};

use crate::debug::components::MouseCoordinateText;

pub mod client;
mod components;
pub mod server;

fn init_cursor_pos_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_section(
            "Not Initialized",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 14.0,
                color: Color::WHITE,
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                ..default()
            }),
        MouseCoordinateText
    ));
}

fn cursor_pos_system(
    // need to get window dimensions of primary window
    windows: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform)>,
    // text
    mut query: Query<(&mut Style, &mut Text), With<MouseCoordinateText>>
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    // let window = if let RenderTarget::Window(id) = camera.target {
    //     windows.get(id).unwrap()
    // } else {
    //     windows.get_primary().unwrap()
    // };
    // TODO: Make this work for multiple windows
    let window = windows.get_single().unwrap();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    let cursor_position = window.cursor_position();
    if let Some(world_position) = cursor_position
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let (mut style, mut text) = query.single_mut();
        let cursor_position = cursor_position.unwrap();
        let mut left_padding = cursor_position.x;
        let mut bottom_padding = cursor_position.y;
        if cursor_position.x >= window.width() - 90.0 {
            left_padding -= 90.0;
        } else {
            left_padding += 10.0;
        }
        if cursor_position.y >= window.height() - 40.0 {
            bottom_padding -= 40.0;
        } else {
            bottom_padding += 10.0;
        }
        style.position = UiRect {
            left: Val::Px(left_padding),
            bottom: Val::Px(bottom_padding),
            ..default()
        };
        text.sections[0].value = format!("({:.2}, {:.2})", world_position.x, world_position.y);
    }
}