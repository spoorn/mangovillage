/// LDTK Level coordinates to our Map coordinates, AKA Transform
pub fn ldtk_to_map_coordinates(grid_size: i32, ldtk_coords: (i32, i32), level_height: i32) -> (f32, f32) {
    // ldtk x-coordinate is the same as our Map coordinates, 0 is left side
    // ldtk y-coordinate is flipped, starting with 0 from the top, while our Map starts with 0 at the bottom
    // Map coordinates are relative to the level px size as the level starts with bottom left corner as (0, 0)
    // So we use LDTK level dimensions to flip the y-coordinate direction.
    //
    // LDTK also has a grid size for each tile, and if the width or height of the Level is not a multiple of the grid
    // size, padding is automatically added, perhaps by bevy_ecs_ldtk or something more under the hood.
    // We first round up the height to the nearest multiple of the grid size, then convert to Map coordinates.
    // We don't need to do this for the x-coordinate as it starts on the same side.
    let mut y_padding = grid_size - level_height % grid_size;
    // If level_height is already a multiple of grid size
    if y_padding == 8 {
        y_padding = 0;
    }
    (ldtk_coords.0 as f32, (level_height + y_padding - ldtk_coords.1) as f32)
}
