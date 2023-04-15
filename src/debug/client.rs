use bevy::app::App;
use bevy::prelude::Plugin;

use crate::debug::{cursor_pos_system, init_cursor_pos_system};

pub struct DebugClientPlugin;
impl Plugin for DebugClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_cursor_pos_system)
            .add_system(cursor_pos_system);
    }
}
