pub mod client;
pub mod server;
pub mod components;
pub mod resources;
pub mod util;

pub const LEVEL_IIDS: [&str; 6] = [
    "a2a50ff0-66b0-11ec-9cd7-c721746049b9",
    "db9e9610-9f30-11ed-9161-49318aa2f89c",
    "be35e6d0-9f30-11ed-8926-a5994758ef9b",
    "e1b4c500-9f30-11ed-9664-17bc3cc5593d",
    "13d9d970-9f30-11ed-9664-93dee238568f",
    "1b5d30e0-9f30-11ed-9664-7734d384d77a",
];

// Grid Size is used to properly map from LDTK Levels that don't exactly map pixel locations, width, height to a
// multiple of the grid size, so we pad coordinates to fit a multiple of it when creating the Map locations.
pub const GRID_SIZE: i32 = 8;